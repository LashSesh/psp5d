use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn run_success_and_replay_divergence_exit_codes() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");
    let rd_path = repo_root.join("examples/golden_trace/rd.json");
    let input_path = repo_root.join("examples/golden_trace/input/input.txt");

    let tmp = tempdir().expect("tmp");
    let run_dir = tmp.path().join("run");
    fs::create_dir_all(&run_dir).expect("mk run dir");

    let mut run_cmd = Command::new(env!("CARGO_BIN_EXE_psp5d"));
    run_cmd
        .arg("run")
        .arg(&input_path)
        .arg("--rd")
        .arg(&rd_path)
        .arg("--out")
        .arg(&run_dir);
    run_cmd.assert().code(0);

    let trace_path = run_dir.join("trace.jsonl");
    let mut lines: Vec<String> = fs::read_to_string(&trace_path)
        .expect("read trace")
        .lines()
        .map(ToOwned::to_owned)
        .collect();
    let mut first: serde_json::Value = serde_json::from_str(&lines[0]).expect("line0");
    first["out_digest"] = serde_json::json!(
        "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    );
    lines[0] = serde_json::to_string(&first).expect("serialize line");
    fs::write(&trace_path, format!("{}\n", lines.join("\n"))).expect("write tampered");

    let mut replay_cmd = Command::new(env!("CARGO_BIN_EXE_psp5d"));
    replay_cmd
        .arg("replay")
        .arg(run_dir.join("evidence.json"))
        .arg("--input")
        .arg(&input_path)
        .arg("--rd")
        .arg(&rd_path);
    replay_cmd.assert().code(30);
}
