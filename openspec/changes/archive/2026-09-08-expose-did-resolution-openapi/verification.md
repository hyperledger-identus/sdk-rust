# Verification receipt

Verification status: changed-scope passed
Verification date: 2026-09-09
Base: develop@094b074d625bca2cd469ae29b38652324f78e94b
Reviewed implementation head: 978e17b99d9429d05810b361686bcf2941e00339

## Behavioral evidence

- `cargo test -p identus-did-resolver-http --no-default-features`: 20/20 tests
  passed with Utoipa absent from the graph.
- `cargo test -p identus-did-resolver-http --features openapi`: 21/21 tests
  passed, including deterministic OpenAPI structure coverage.
- `cargo test --workspace --all-features`: all 667 runnable tests passed; 22
  configured diagnostics remained ignored. `--list` discovered 689 tests.
- The new test covers the only path and GET operation; exact parameter order
  and locations; all implemented response statuses; three success media types;
  error media type; `Vary` headers; limit values; extension-option disclosure;
  absence of POST; and repeat-serialization equality.

## Compiler, documentation and factory evidence

- `cargo clippy -p identus-did-resolver-http --all-targets --all-features --
  -D warnings` passed.
- `cargo fmt --all -- --check`, `git diff --check`, locked Cargo metadata and
  `cargo doc --workspace --all-features --no-deps` passed under Rust 1.98.1.
- `scripts/factory research-ready expose-did-resolution-openapi`,
  `constraints-ready` and `check` passed with 51/51 OpenSpec items.
- Exact changed production-source inspection found no unsafe, panic, unwrap or
  expect path. Both implementation commits are signed and carry DCO trailers.

## Dependency and deferred environment evidence

- `cargo tree -p identus-did-resolver-http --no-default-features` and
  `cargo tree -p identus-did` contain no Utoipa package.
- The opt-in graph adds exact `utoipa 5.5.0` and `utoipa-gen 5.5.0`; `syn`,
  `quote`, `proc-macro2`, serde, serde_json and indexmap already existed in the
  lockfile. No generator macro is invoked by SDK source.
- Direct Nix, nextest and cargo-deny executables are unavailable in this shell;
  the repository factory wrapper passed and hosted/weekly lanes remain the
  authority for those checks.
- Installed `cargo-audit 0.20.1` fetched the current advisory database but
  stopped because it cannot parse CVSS 4.0 in RUSTSEC-2026-0073. This is a
  local tool-parser limitation, not an advisory match.

## Result

All changed-scope and locally available repository gates pass. Hosted DCO,
policy, factory, file-hygiene and Linux fast tests remain mandatory before
merge; the exhaustive Nix matrix remains scheduled weekly by policy.
