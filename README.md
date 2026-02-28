# PSP5D Engine

Deterministische Rust-Referenzimplementierung für **PSP-core** (modellagnostischer Kernel) plus **PSP5D-Model-Pack** und **Triton-Layer**.

Die Codebase ist als Cargo-Workspace aufgebaut und fokussiert auf:

- reproduzierbare Zustandsübergänge,
- replay-fähige Artefakte (Trace/Evidence/Manifest/Ledger),
- klare Trennung zwischen Core-Engine und Modellspezifik.

---

## Inhaltsverzeichnis

1. [Ziele und Eigenschaften](#ziele-und-eigenschaften)
2. [Repository-Struktur](#repository-struktur)
3. [Architektur im Überblick](#architektur-im-überblick)
4. [Build & Setup](#build--setup)
5. [CLI-Workflows](#cli-workflows)
6. [Artefakte eines Runs](#artefakte-eines-runs)
7. [Determinismus-Vertrag](#determinismus-vertrag)
8. [Testen](#testen)
9. [Spezifikation & Schemata](#spezifikation--schemata)
10. [Lizenz](#lizenz)

---

## Ziele und Eigenschaften

- **Deterministische Ausführung:** identische Inputs + identisches RD erzeugen identische Digests.
- **Replaybarkeit:** ein Run kann über persistierte Artefakte überprüft werden.
- **Modellwand (Model Wall):** `psp5d_core` ist unabhängig vom PSP5D-Modellpack.
- **Schemavalidierung:** zentrale JSON-Artefakte werden gegen eingebettete JSON-Schemata prüfbar.

---

## Repository-Struktur

```text
./
├─ crates/
│  ├─ psp5d_core/          # Modellagnostischer Kernel (Engine, Digests, Replay, Manifest, Schema)
│  ├─ psp5d_model_psp5d/   # PSP5D-Modelllogik (UIR, HDAG, TriMoebius, Observer, Gates, Frontends)
│  ├─ psp5d_layer_triton/  # Deterministischer Triton-Operator-Layer (Spiral, Spectral, Gate, TIC)
│  └─ psp5d_cli/           # CLI-Binary `psp5d` (run/replay/audit)
├─ spec/
│  ├─ SPEC_PSP5D_ENGINE_v1_1.md
│  └─ schemas/             # JSON-Schemata für RD/Trace/Manifest/etc.
├─ examples/
│  ├─ rd_min.json
│  ├─ input_small/
│  └─ golden_trace/
├─ LICENSE
└─ NOTICE
```

---

## Architektur im Überblick

### 1) `psp5d_core` (Kernel)

Der Core stellt die deterministische Ausführungsmaschine bereit:

- `Engine::run(...)` iteriert ein `Program` in fester Rollenfolge und erzeugt pro Schritt Digests + Trace-Eintrag.
- `Program::psp_core_default_cycle()` definiert den Standard-Zyklus (`ingest` → `canon` → ... → `emit`).
- `RunDescriptor` (RD) kapselt die erlaubten Laufparameter (Versionen, Policies, Canon-Settings).
- Replay-Hilfen (`first_divergence`, `verify_manifest_consistency`) prüfen Reproduzierbarkeit.

### 2) `psp5d_model_psp5d` (Model-Pack)

Der Modellpack implementiert PSP5D-spezifische Bausteine und Frontends.
Aktuell enthält er außerdem eine deterministische Beispiel-Engine (`run_10_steps`), die den Zählerzustand fortschreibt und pro Rolle Evidence erzeugt.

### 3) `psp5d_layer_triton` (Operator-Layer)

Triton ergänzt deterministische Operatoren für spektrale Bewertung, Spiral-Exploration, Solve/Coagula-Gating und TIC-Kristallisierung.
Die Operatoren sind als `EngineOperator`-Implementierungen ausgeführt und können in Engine-Programme eingebunden werden.

### 4) `psp5d_cli` (Tooling)

Die CLI orchestriert den End-to-End-Flow:

- Run ausführen,
- Artefakte schreiben,
- Replay validieren,
- Ledger-Head auditieren.

---

## Build & Setup

> Voraussetzungen: Rust (Edition 2021, `rust-version = 1.75` laut Workspace-Metadaten).

```bash
cargo build
cargo run -p psp5d_cli -- --help
```

---

## CLI-Workflows

Die CLI gibt maschinenlesbare JSON-Ausgaben auf stdout aus.

## 1) Run

Erzeugt einen deterministischen Run und schreibt Artefakte ins Zielverzeichnis.

```bash
cargo run -p psp5d_cli -- \
  run examples/golden_trace/input/input.txt \
  --rd examples/golden_trace/rd.json \
  --out /tmp/psp5d_run
```

Erwartete Dateien in `--out`:

- `trace.jsonl`
- `evidence.json`
- `manifest.json`
- `ledger.jsonl`

## 2) Replay

Validiert einen vorher erzeugten Run gegen denselben Input + RD.

```bash
cargo run -p psp5d_cli -- \
  replay /tmp/psp5d_run/evidence.json \
  --input examples/golden_trace/input/input.txt \
  --rd examples/golden_trace/rd.json
```

Bei Abweichung liefert die CLI einen deterministischen Fehler mit erstem Divergenzpunkt.

## 3) Audit

Prüft, ob der übergebene Head-Digest zum Ledger passt.

```bash
# Beispiel-Head aus dem letzten Ledger-Eintrag lesen
HEAD=$(tail -n 1 /tmp/psp5d_run/ledger.jsonl | jq -r '.digest')

cargo run -p psp5d_cli -- \
  audit /tmp/psp5d_run/ledger.jsonl \
  --head "$HEAD"
```

## Exit-Codes (aktuelle Implementierung)

- `0`: Erfolg
- `10`: valider Run, aber kein Commit (`emit` nicht erreicht/gesetzt)
- `20`: deterministischer Verifikations-/Policy-Fehler
- `30`: Replay-Divergenz
- `40`: I/O- oder JSON-Parsing-Fehler

---

## Artefakte eines Runs

- **Trace (`trace.jsonl`)**: sequentielle Schrittliste mit Rollen-, Operator- und Digest-Informationen.
- **Evidence (`evidence.json`)**: verdichtete, replay-relevante Schrittmetadaten inkl. RD-/Trace-Bindung.
- **Manifest (`manifest.json`)**: Bindung von RD/Input/Trace/Evidence/Head-Digest.
- **Ledger (`ledger.jsonl`)**: verkettete Digest-Historie (`prev_digest` → `digest`) für Audit-Zwecke.

---

## Determinismus-Vertrag

Die Umsetzung folgt den zentralen Prinzipien:

1. **Keine versteckten Kanäle** (kein impliziter Zugriff auf nicht deklarierte Entropiequellen).
2. **RD-Autorität** für Kanonisierung und Laufparameter.
3. **Digest-Bindung** aller relevanten Artefakte.
4. **Deterministische Fehlerklassen** (z. B. Replay-Divergenz mit erstem Abweichungsschritt).

---

## Testen

Gesamte Workspace-Tests:

```bash
cargo test
```

Nur CLI-Tests:

```bash
cargo test -p psp5d_cli
```

Nützlicher Fokus während Entwicklung:

```bash
cargo test -p psp5d_core
cargo test -p psp5d_model_psp5d
cargo test -p psp5d_layer_triton
```

---

## Spezifikation & Schemata

- Normative Spezifikation: `spec/SPEC_PSP5D_ENGINE_v1_1.md`
- JSON-Schemata: `spec/schemas/*.schema.json`

Der Core lädt diese Schemata per `include_str!` und ermöglicht programmgesteuerte Prüfung via `validate_against_schema(...)`.

---

## Lizenz

Dieses Projekt steht unter **Apache-2.0**. Siehe `LICENSE` und `NOTICE`.
