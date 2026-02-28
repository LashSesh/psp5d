use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use psp5d_core::{
    build_evidence, build_manifest, first_divergence, verify_ledger_head,
    verify_manifest_consistency, LedgerEntry, ReplayManifest, RunDescriptor, TraceEntry,
};
use psp5d_model_psp5d::run_10_steps;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
#[command(name = "psp5d")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        input_path: PathBuf,
        #[arg(long)]
        rd: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    Replay {
        evidence_path: PathBuf,
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        rd: PathBuf,
    },
    Audit {
        ledger_path: PathBuf,
        #[arg(long)]
        head: String,
    },
}

#[derive(Serialize)]
struct JsonOut {
    ok: bool,
    code: i32,
    message: String,
    data: Value,
}

fn main() {
    let cli = Cli::parse();
    let result = run(cli);
    let (code, out) = match result {
        Ok(v) => (v.code, v),
        Err((code, message, data)) => (
            code,
            JsonOut {
                ok: false,
                code,
                message,
                data,
            },
        ),
    };
    let line = serde_json::to_string(&out).expect("json output");
    println!("{line}");
    std::process::exit(code);
}

fn run(cli: Cli) -> Result<JsonOut, (i32, String, Value)> {
    match cli.command {
        Commands::Run {
            input_path,
            rd,
            out,
        } => cmd_run(&input_path, &rd, &out),
        Commands::Replay {
            evidence_path,
            input,
            rd,
        } => cmd_replay(&evidence_path, &input, &rd),
        Commands::Audit { ledger_path, head } => cmd_audit(&ledger_path, &head),
    }
}

fn read_rd(rd_path: &Path) -> Result<RunDescriptor, (i32, String, Value)> {
    let raw =
        fs::read_to_string(rd_path).map_err(|e| (40, e.to_string(), json!({"path":rd_path})))?;
    serde_json::from_str(&raw).map_err(|e| (40, e.to_string(), json!({"path":rd_path})))
}

fn digest_input(path: &Path, rd: &RunDescriptor) -> Result<String, (i32, String, Value)> {
    let raw = fs::read_to_string(path).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
    psp5d_core::digest_sha256_jcs(&json!({"input":raw}), rd)
        .map_err(|e| (40, e.to_string(), json!({"path":path})))
}

fn cmd_run(input_path: &Path, rd_path: &Path, out: &Path) -> Result<JsonOut, (i32, String, Value)> {
    let rd = read_rd(rd_path)?;
    let input_digest = digest_input(input_path, &rd)?;
    let (_state, trace) = run_10_steps(&rd).map_err(|e| (20, e.to_string(), json!({})))?;

    let evidence = build_evidence(&trace, &rd, input_digest.clone())
        .map_err(|e| (20, e.to_string(), json!({})))?;
    let trace_digest = psp5d_core::digest_sha256_jcs(
        &serde_json::to_value(&trace).map_err(|e| (20, e.to_string(), json!({})))?,
        &rd,
    )
    .map_err(|e| (20, e.to_string(), json!({})))?;
    let head_digest = trace
        .last()
        .map(|t| t.out_digest.clone())
        .unwrap_or_else(|| "sha256:0".to_string());
    let manifest = build_manifest(&rd, &input_digest, &trace_digest, &evidence, &head_digest)
        .map_err(|e| (20, e.to_string(), json!({})))?;

    fs::create_dir_all(out).map_err(|e| (40, e.to_string(), json!({"out":out})))?;
    write_trace_jsonl(&out.join("trace.jsonl"), &trace)?;
    write_json(&out.join("evidence.json"), &evidence)?;
    write_json(&out.join("manifest.json"), &manifest)?;
    let ledger = make_ledger(&trace);
    write_ledger_jsonl(&out.join("ledger.jsonl"), &ledger)?;

    let committed = trace.iter().any(|t| t.role == "emit");
    let code = if committed { 0 } else { 10 };
    Ok(JsonOut {
        ok: committed,
        code,
        message: if committed {
            "run complete".to_string()
        } else {
            "gate failed: valid run but no commit".to_string()
        },
        data: json!({
            "run_dir": out,
            "trace_digest": trace_digest,
            "head_digest": head_digest,
            "manifest_path": out.join("manifest.json"),
            "evidence_path": out.join("evidence.json")
        }),
    })
}

fn cmd_replay(
    evidence_path: &Path,
    input_path: &Path,
    rd_path: &Path,
) -> Result<JsonOut, (i32, String, Value)> {
    let rd = read_rd(rd_path)?;
    let input_digest = digest_input(input_path, &rd)?;

    let run_dir = if evidence_path.is_dir() {
        evidence_path.to_path_buf()
    } else {
        evidence_path
            .parent()
            .ok_or((
                40,
                "invalid evidence path".to_string(),
                json!({"path":evidence_path}),
            ))?
            .to_path_buf()
    };

    let expected_trace: Vec<TraceEntry> = read_trace_jsonl(&run_dir.join("trace.jsonl"))?;
    let expected_evidence: psp5d_core::Evidence = read_json(&run_dir.join("evidence.json"))?;
    let expected_manifest: ReplayManifest = read_json(&run_dir.join("manifest.json"))?;

    if expected_manifest.input_digest != input_digest {
        return Err((
            20,
            "input digest mismatch with replay pack".to_string(),
            json!({"expected":expected_manifest.input_digest,"actual":input_digest}),
        ));
    }

    verify_manifest_consistency(&expected_evidence, &expected_manifest)
        .map_err(|e| (20, e.to_string(), json!({})))?;

    let (_state, actual_trace) = run_10_steps(&rd).map_err(|e| (20, e.to_string(), json!({})))?;
    if let Some(div) = first_divergence(&expected_trace, &actual_trace) {
        return Err((
            30,
            "replay divergence".to_string(),
            serde_json::to_value(&div).map_err(|e| (20, e.to_string(), json!({})))?,
        ));
    }

    Ok(JsonOut {
        ok: true,
        code: 0,
        message: "replay successful".to_string(),
        data: json!({"run_dir": run_dir}),
    })
}

fn cmd_audit(ledger_path: &Path, head: &str) -> Result<JsonOut, (i32, String, Value)> {
    let entries = read_ledger_jsonl(ledger_path)?;
    verify_ledger_head(&entries, head)
        .map_err(|e| (20, e.to_string(), json!({"ledger":ledger_path,"head":head})))?;
    Ok(JsonOut {
        ok: true,
        code: 0,
        message: "audit successful".to_string(),
        data: json!({"entries":entries.len(),"head":head}),
    })
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), (i32, String, Value)> {
    let s = serde_json::to_string(value).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
    fs::write(path, s).map_err(|e| (40, e.to_string(), json!({"path":path})))
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, (i32, String, Value)> {
    let raw = fs::read_to_string(path).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
    serde_json::from_str(&raw).map_err(|e| (40, e.to_string(), json!({"path":path})))
}

fn write_trace_jsonl(path: &Path, trace: &[TraceEntry]) -> Result<(), (i32, String, Value)> {
    let mut buf = String::new();
    for t in trace {
        let line =
            serde_json::to_string(t).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
        buf.push_str(&line);
        buf.push('\n');
    }
    fs::write(path, buf).map_err(|e| (40, e.to_string(), json!({"path":path})))
}

fn read_trace_jsonl(path: &Path) -> Result<Vec<TraceEntry>, (i32, String, Value)> {
    let raw = fs::read_to_string(path).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let row: TraceEntry =
            serde_json::from_str(line).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
        out.push(row);
    }
    Ok(out)
}

fn make_ledger(trace: &[TraceEntry]) -> Vec<LedgerEntry> {
    let mut prev =
        "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string();
    let mut out = Vec::with_capacity(trace.len());
    for (i, t) in trace.iter().enumerate() {
        let digest = t.out_digest.clone();
        out.push(LedgerEntry {
            seq: i as u64,
            digest: digest.clone(),
            prev_digest: prev,
            kind: t.role.clone(),
            payload_digest: t.op_evidence_digest.clone(),
        });
        prev = digest;
    }
    out
}

fn write_ledger_jsonl(path: &Path, entries: &[LedgerEntry]) -> Result<(), (i32, String, Value)> {
    let mut buf = String::new();
    for e in entries {
        let line =
            serde_json::to_string(e).map_err(|err| (40, err.to_string(), json!({"path":path})))?;
        buf.push_str(&line);
        buf.push('\n');
    }
    fs::write(path, buf).map_err(|e| (40, e.to_string(), json!({"path":path})))
}

fn read_ledger_jsonl(path: &Path) -> Result<Vec<LedgerEntry>, (i32, String, Value)> {
    let raw = fs::read_to_string(path).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let row: LedgerEntry =
            serde_json::from_str(line).map_err(|e| (40, e.to_string(), json!({"path":path})))?;
        out.push(row);
    }
    Ok(out)
}
