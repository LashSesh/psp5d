# SPEC_PSP5D_ENGINE_v1_1

Status: Normative
Version: 1.1

## 1. Scope

This document defines a deterministic execution framework named **PSP-core** and a model-specific pack named **PSP5D**.

- PSP-core defines execution artifacts that are model-agnostic.
- PSP5D defines domain structures and operators that bind onto PSP-core abstractions.
- Implementations MUST preserve deterministic replay under this specification.

## 2. Formal entities

### 2.1 Sigma (Σ): state

`Σ` is the full mutable state snapshot at a logical step boundary.

Minimum structure:

- `sigma_id`: unique state identifier.
- `payload`: model-specific bytes or structured fields.
- `digest`: content digest over canonical encoding.

Requirements:

- Canonical encoding MUST be deterministic.
- Equal canonical encodings MUST imply equal digest.

### 2.2 Omega (Ω): operators

`Ω = {ω_1 .. ω_n}` is the set of pure transition operators.

Operator contract:

- Input: `(Σ_in, params, context)`.
- Output: `(Σ_out, evidence_fragment)`.
- Operators MUST be deterministic for identical `(Σ_in, params, context)`.
- Operators MUST NOT access undeclared side channels.

### 2.3 Run Descriptor (RD)

`RD` defines all admissible execution parameters.

Required fields:

- `spec_version`.
- `engine_version`.
- `model_pack_version`.
- `seed_policy`: `none | fixed(seed_id, bytes)`.
- `io_policy`: explicit list of admitted external inputs.
- `normalization_profile`: canonicalization rules for text/number encodings.

RD is authoritative: any input not admitted by RD MUST invalidate a run.

### 2.4 Trace

`Trace` is an ordered sequence of steps.

Each step MUST include:

- `index`.
- `op_code`.
- `op_params` (canonical form).
- `sigma_before_digest`.
- `sigma_after_digest`.
- `evidence_ref`.

Trace MUST be append-only and index-contiguous (`0..k`).

### 2.5 Evidence

`Evidence` is verifiable metadata supporting each transition.

Evidence SHOULD include:

- operator-local diagnostics,
- optional deterministic timing class (not wall-clock timestamps),
- hashes of admitted input material.

Evidence MUST be bound by digest to its trace step.

### 2.6 Manifest

`Manifest` binds replay-critical artifacts.

Manifest MUST contain:

- hash of RD,
- hash of initial Σ,
- hash of full Trace,
- hash set of Evidence blobs,
- declared hash algorithm.

## 3. Replay rules

Given `(RD, Σ0, Trace, Evidence, Manifest)`:

1. Validate manifest hash consistency.
2. Recompute each `ω_i` in step order.
3. Verify `sigma_before_digest` and `sigma_after_digest` at each step.
4. Verify evidence bindings.
5. Accept replay only if all checks pass.

Replay MUST fail on first mismatch with deterministic error classification.

## 4. PSP5D model pack

PSP5D specializes PSP-core with the following entities.

### 4.1 UIR

**UIR (Unified Interaction Record)** is the atomic model payload.

UIR fields:

- `uir_id`: stable identifier.
- `axes[5]`: coordinates for 5D hypercube projection.
- `hdag_links`: deterministic adjacency list.
- `tri_moebius_phase`: signed phase marker.
- `gate_mask`: bitmask of enabled gates.

### 4.2 Hypercube

The PSP5D hypercube is a deterministic coordinate space where each UIR maps to one vertex projection.

Rules:

- Coordinate normalization MUST be canonical.
- Projection and inverse projection MUST be total for admitted ranges.

### 4.3 HDAG

**HDAG** is a hash-directed acyclic graph over UIR references.

Rules:

- Node identity MUST be digest-based.
- Edge ordering MUST be canonical.
- Cycles are forbidden; insertions creating cycles MUST fail deterministically.

### 4.4 TriMoebius transform

TriMoebius is a 3-channel reversible transform over UIR phase space.

Rules:

- Transform MUST be invertible under declared profile.
- Rounding/quantization behavior MUST be profile-bound in RD.

### 4.5 Observer R5

Observer R5 produces deterministic summaries over Σ and Trace prefixes.

Rules:

- Observer output MUST be a pure function of declared inputs.
- Any heuristic thresholds MUST be encoded in RD.

### 4.6 Gates

Gates are policy predicates evaluated before operator execution.

Rules:

- Gate evaluation order MUST be fixed.
- Gate failures MUST produce stable error codes.
- Gate decisions MUST be logged to Evidence.

## 5. Model wall and dependency constraints

- `psp5d_core` MUST NOT depend on `psp5d_model_psp5d`.
- `psp5d_model_psp5d` MAY depend on `psp5d_core`.
- CLI MAY depend on both.

## 6. CLI contract

Binary name: `psp5d`.

Minimum commands:

- `psp5d --help`: print usage.
- `psp5d validate-manifest <path>`: validate structural integrity.
- `psp5d replay <run_dir>`: perform replay checks.
- `psp5d inspect-rd <path>`: print normalized RD summary.

Exit codes:

- `0`: success.
- `2`: invalid input/arguments.
- `3`: deterministic replay mismatch.
- `4`: policy violation (undeclared channel/gate failure).

## 7. File formats

Implementations SHOULD use UTF-8 JSON for interchange with canonical key ordering.

- `rd.json`
- `state_0.json`
- `trace.jsonl` (one step per line)
- `evidence/` (content-addressed blobs)
- `manifest.json`

Canonicalization:

- canonical JSON MUST follow RFC 8785 (JCS) byte serialization,
- numbers MUST be finite; apply `RD.canon.float_policy` before JCS (default: Q16.16 round-half-even),
- strings: NFC normalization,
- objects: lexicographically ordered keys under JCS,
- digest format: `sha256:<hex>` over canonical UTF-8 bytes.

## 8. Acceptance checklist

A conforming workspace MUST satisfy:

1. Build success for entire Cargo workspace.
2. CLI help invocation success.
3. Presence of this specification file.
4. Presence of Apache-2.0 license text.
5. Demonstrable model wall (core compiles independently of model crate).

