## Why

Issue #34 established a bounded, dependency-free parser for the generic W3C
`Did` and `DidUrl` lexical boundary. Its deterministic matrices are broad, but
they cannot continuously search hostile byte combinations for panics, offset
mistakes, or disagreement between public constructors and serialization.

Issue #35 (`IDR-005b`) adds that generative assurance in sdk-rust. This keeps
generic parser safety reusable by NeoPRISM, midnight-identity, Lace and Oxid,
while method state and chain-specific validation remain downstream. Apollo has
no DID parser to preserve and may be deprecated.

## What Changes

- Pin the W3C DID Core Recommendation, editor/test-suite source revisions, the
  repository nightly, locked Nix `cargo-fuzz`, and exact `libfuzzer-sys`.
- Add independent `Did` and `DidUrl` libFuzzer targets which consume arbitrary
  bytes and assert public round-trip and component-slice invariants.
- Add small original seed corpora and dictionaries spanning standards and
  consumer-shaped lexical boundaries without method semantics.
- Provide one script for corpus replay, fixed-run smoke, and bounded soak modes.
- Add cargo-fuzz to the Nix development shell and a path-scoped Ubuntu workflow
  with deterministic PR/push smoke plus scheduled/manual bounded soak.
- Document crash minimization, regression promotion, resource ceilings,
  performance evidence, and residual risk.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `did-core`: makes sanitizer-backed generic DID lexical fuzzing reproducible
  while preserving public API and the production dependency cone.

## Impact

- **Issue:** #35, child of #5 / `IDR-005`.
- **Affected surface:** independent `fuzz/` workspace, Nix development shell,
  focused workflow, script/docs, canonical DID Core requirements, ADR 0018.
- **Public/wire compatibility:** unchanged; the targets consume existing APIs.
- **Dependencies:** no published crate dependency changes; fuzz-only packages
  remain outside the root workspace dependency graph.
- **Consumers:** all downstream repositories remain read-only and method-owned.
- **Rollback:** revert this focused PR; no release, persisted data, consumer
  repository, chain state, product repository, or `main` branch is changed.
