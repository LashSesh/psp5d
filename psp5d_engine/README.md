# PSP5D Engine

Reference Rust workspace for deterministic PSP-core execution with a model-specific PSP5D pack.

## Repository layout

- `crates/psp5d_core`: model-agnostic deterministic kernel (state, operators, trace, replay).
- `crates/psp5d_model_psp5d`: PSP5D-specific semantics (UIR, hypercube, HDAG, TriMoebius, R5 observer, gates).
- `crates/psp5d_cli`: `psp5d` binary for inspect/validate/replay workflows.
- `spec/SPEC_PSP5D_ENGINE_v1_1.md`: normative specification.

## Quickstart

```bash
cargo build
cargo run -p psp5d_cli -- --help
```

## Determinism contract

The engine and model pack follow strict replayability guarantees:

1. **No hidden channels**: runtime behavior must not depend on host wall clock, ambient randomness, network ordering, or mutable global process state.
2. **Run Descriptor (RD) authority**: any allowed entropy/input source must be declared in RD and materialized in trace evidence.
3. **Byte-stable replay**: replaying an accepted trace + manifest must reproduce equivalent state transitions and digest outputs.
4. **Model wall**: `psp5d_core` is fully model-agnostic and must compile without a dependency on `psp5d_model_psp5d`.

See the spec for formal definitions and acceptance checklist.
