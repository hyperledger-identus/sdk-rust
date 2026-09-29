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
protected delivery uses squash merge. The implementation therefore merged
first, and the issue-linked closeout generates and pins its report from the
durable protected squash before archiving this change.

## Protected implementation evidence

- PR #442 passed DCO, pull-request policy, every file-hygiene check, and the
  exact-head fast gate in 7m47s before guarded merge.
- The guarded squash merge produced protected
  `develop@7a95db2b40ee16b2ae14380eb438c6db239793cd`.
- A canonical v2 report generated from that exact protected revision has
  source fingerprint
  `0c190d272ea56e542e2af153e148591839ca9b62e70cfda1f85ef217b71411bb`,
  population projection
  `2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
  and report digest
  `aafedad49e8fb8696c5071aca898f9461aa9f48d675c47623692a8bff2e30d7f`.
- The refreshed report has no production module above 1,000 authored nonblank
  lines and no wallet-conformance function signal. Every unrelated hotspot
  disposition remains unchanged.
- PR #442 metrics are retained in the private v2 store and published on the
  merged PR. The one failed/retried metadata check records the initial
  non-closing issue reference; no code or post-CI push was required.

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
