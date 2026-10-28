# Verification receipt

Verification date: 2026-10-28
Issue: #557
Develop base: `48c61920b8553d5401375338a5b217421b4c3c10`
Planning contract: `3dc45b207aa8e7ee892bc804489765ab9f913940`
Preimplementation receipt: `1391996538be16da4f32afb54ced680f1bd9578d`
Evidence source head: `02584a585d204d4f8b5be256802b35c5f4bb0fb8`

## Focused behavior and E2E evidence

- `cargo test -p identus-did-key --all-features`: passed unit,
  conformance, attributed-vector, property and registry E2E tests.
- The E2E route creates all four supported direct public-key forms, parses
  their canonical identifiers, composes an opt-in generic registry, resolves
  deterministic DID documents, serializes DID JSON and DID JSON-LD, and
  reparses the resulting document.
- Negative and boundary tests cover unsupported multibase/codec forms,
  truncated and overlong varints, wrong lengths, non-canonical aliases, weak
  or non-contributory material, oversized identifiers and early resolver
  rejection.
- Strict all-target, all-feature Clippy and crate rustdoc passed with warnings
  denied.

## Fuzz and performance evidence

- The bounded `did_key` fuzz target passed a 300-second soak: 43,875,485
  executions, 987 features and 529 MiB peak RSS under a 1,024 MiB limit, with
  no crash or timeout.
- After final corpus and security corrections, a 4,096-run smoke passed with
  47 MiB peak RSS and no crash or timeout.
- `cargo bench -p identus-did-key --bench directional` measured directional
  medians of 1,946 ns for creation, 5,348 ns for parsing, 5,142 ns for local
  resolution and 95 ns for early oversized rejection.
- The versioned vector packet and quality registry bind differential,
  property, fuzz and benchmark obligations to durable source paths.

## Workspace and factory evidence

- Full workspace all-feature and no-default-feature tests passed after the
  final Ed25519 and resolver hardening.
- `scripts/factory check` passed, including dependency, support-policy,
  resource-boundary, vector, quality-evidence, ledger and OpenSpec mutation
  contracts.
- The focused `rust-source-contract` Nix derivation passed after retaining the
  exact attributed did:key fixture in the clean source.
- The exact implementation head passed the final Codex CLI review with no
  actionable defects.

## Compiler, portability and dependency evidence

- Locked MSRV 1.89 evidence passed for the new crate and its dependency cone.
- Exact builds passed for `wasm32-unknown-unknown`, `aarch64-apple-ios` and
  `aarch64-linux-android`.
- The production dependency cone contains existing workspace crates plus the
  already-adopted bounded encoding and key primitives; no network, persistence,
  runtime, binding or product package was introduced.

These are SDK source, compile, test, fuzz, benchmark and local-resolution
claims. They are not claims of private-key custody, signing, live network
interoperability, downstream adoption, certification, binding packaging or
release. Protected Linux CI must still validate the exact PR head.
