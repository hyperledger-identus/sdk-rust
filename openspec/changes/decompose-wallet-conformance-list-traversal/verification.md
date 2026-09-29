# Verification evidence

## Identity

- Issue: #441
- Exact protected develop base:
  `5031e7178bee1b031e9f7e84e677d03f95e936d3`
- Planning commit: `215e3025e7de1d04595bb7f9be3f22853fa27518`
- Preimplementation receipt commit: `8ff4a98`
- Reviewed implementation head:
  `0f2b2df7eeedbe173d307bd52445ef77fb4d87f8`

## Behavioral compatibility

- The pre-change focused suite passed 7 tests with one ignored release
  diagnostic.
- A closed test-only fault matrix binds adapter failure, overlong pages,
  duplicate entries, wrong/excess membership, and repeated cursors to their
  exact existing `(step, kind)` projection.
- A redacted request transcript proves the successful three-call traversal:
  first request followed by two continuations, each with page size one.
- The post-change focused and workspace suites pass with the same successful
  combined `19/3` report across all three list-capable production ports.
- Characterization proved the former `list-termination` branch unreachable:
  the validated `StoragePage` type forbids an empty continued page, so every
  continuation must add an entry and duplicate/excess membership fails first.
  The public `PaginationDidNotTerminate` variant remains source compatible.

## Public and dependency compatibility

- `cargo-public-api 0.52.0` with the repository compiler and subprocess-scoped
  `RUSTC_BOOTSTRAP=1` reports no removed, changed, or added simplified public
  item between the protected base and implementation head.
- Root/workspace manifests, the wallet-conformance manifest, and `Cargo.lock`
  are unchanged. No dependency, feature, unsafe, native, FFI, serialization,
  or wire contract changed.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| List traversal SLOC | 82 | no list function above threshold |
| List traversal cognitive complexity | 11 | no list function above threshold |
| List traversal cyclomatic complexity | 17 | no list function above threshold |
| Wallet-conformance over-threshold modules | 0 | 0 |

The improvement is one private `ListEvidence<Entry>` state owner plus a small
async coordinator. It is not a forwarding-only helper chain, moved production
code, generated implementation, waiver, or weaker threshold.

The canonical baseline cannot safely pin a feature-branch commit because
protected delivery uses squash merge. After the implementation PR merges, a
second issue-linked closeout PR will generate and pin the report from the
durable protected squash, remove only the completed list disposition, archive
the OpenSpec change, and close #441.

## Local gates

- Focused wallet-conformance tests: 7 passed, 1 ignored diagnostic.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: no added, removed, or changed simplified item.
- Source distribution, factory contract, OpenSpec, format, diff, and flake
  evaluation gates: passed.
- Canonical Nix `rust-test` gate: passed.
