# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in the SDK has a bounded object-safe
`DidResolver`, `ResolutionOptions` and
`DidResolutionResult`, but intentionally no transport. NeoPRISM's
`lib/did-resolver-http` proves the adapter shape with Axum. Its 182-line source
and 576-line test suite were inspected at current `main` revision
`d4608fe3d662ebaf425c4a6380f6d241cffe74c3`; source/test SHA-256 values are
`101a040ea6f59f86f91d2ca054f43646e6d88f5d4527ddf4df5ef1d063568708`
and `a3d5699625fb525433efc67bf982e9cf4ceee6cc1386ccc531a20f21c75b4418`.
The donor is Apache-2.0.

The donor is an extraction source, not a drop-in implementation. It splits
one `Accept` value into a `HashSet`, does not implement quality or specificity,
ignores query options marked TODO, lets unsupported media fall through to a
document response and accepts a caller route string that can panic during
router construction. Its older error mapping also includes a type absent from
the SDK's pinned nine-kind W3C model.

## Normative sources

- [W3C DID Resolution v1](https://www.w3.org/TR/did-resolution/) is the
  current Candidate Recommendation Draft published 2026-08-28. The assessed
  source revision is
  [`w3c/did-resolution@71a50058`](https://github.com/w3c/did-resolution/tree/71a50058090417f9947b9f13985fc8b561a4ad59).
- [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html) defines `Accept`
  grammar, media ranges, wildcards, quality values and precedence.
- [`axum 0.8.9`](https://docs.rs/axum/0.8.9/axum/) defines the selected router,
  extractor and response surface.
- [`headers-accept 0.3.0`](https://docs.rs/headers-accept/0.3.0/headers_accept/)
  defines the selected private negotiation engine.

The W3C HTTP binding requires a full result for
`application/did-resolution`, a DID document for another supported
representation, and status mappings of 400 for invalid DID/DID URL/options,
404 for not found, 406 for unsupported representation, 500 for invalid
document/internal or extension errors, 501 for unsupported method/feature,
and 410 when document metadata marks deactivation.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| NeoPRISM HTTP adapter | `d4608fe3` | `oracle` | Proven Axum shape and W3C fixtures, but negotiation, bounds, query and route behavior need SDK-owned adaptation. | Revisit donor parity when its semantics or standard baseline changes. |
| `axum` | exact 0.8.9 | `adopt` | Current maintained framework, Rust 1.80 MSRV, state-closed router composition and no server runtime needed with defaults off plus `json`. | Advisory, maintenance, Rust/target failure, or an accepted framework-neutral binding. |
| `headers-accept` | exact 0.3.0 | `conditional-adopt` privately | Handles quoted commas, specificity, order and wildcards with a narrow API and current `http` 1.x compatibility. SDK preflight closes its invalid-`q` fail-open behavior. | Remove if strict wrapper cannot prove bounded RFC behavior or maintenance/security evidence regresses. |
| `accept-header` | 0.2.3 | `not-adopt` | Stale, `http` 0.2 cone, raw-value diagnostics and incomplete negotiation semantics. | A maintained `http` 1.x release with strict bounded behavior. |
| `negotiator` | 0.1.0 | `not-adopt` | Zero-dependency/no-std design is attractive, but the new 0.1 crate has broad JavaScript-style behavior and little adoption for a three-representation server. | Maturity and a named broader negotiation consumer. |
| Hand-written full negotiation | local | `not-adopt` | Reimplementing RFC precedence and quoted parameter parsing is unnecessary. | Candidate failure with no suitable maintained alternative. |

## Compatibility and dependency evidence

The exact versions and features are `axum =0.8.9` with default features
disabled plus `json`, and `headers-accept =0.3.0`. Axum is MIT, declares MSRV
Rust 1.80 and keeps listener/runtime features out of the normal adapter graph.
Its release tag object is
`31872e2e543b9cbb0e671ca7d678448b38cfb4c7`.

`headers-accept 0.3.0` is MIT and depends on `headers-core 0.3`, `http 1.1`
and `mediatype 0.21`. Its assessed tag object is
`6cc50f3c740070a9fdc47a559418c8898ee0b4da`; the repository remained active
at source revision `36bda0995513b025312fed75d7947b66865ff149` on the retrieval
date. It forbids unsafe code. `mediatype 0.21` is MIT, has no required
transitive dependency and contains no authored unsafe code.

The direct and resolved dependency cone in the integrated lockfile adds 19
external package names: 17 in the normal
host-adapter cone plus development-only `tokio` and `tokio-macros`. The normal
cone has no Hyper, Tokio, Reqwest, listener, socket, TLS or native dependency;
Tower's service abstraction is pulled by Axum without its test-only `util`
feature. Direct release checksums are
`31b698c5f9a010f6573133b09e0de5408834d0c82f8d7475a89fc1867a71cd90`
for Axum, `78beb98ea9a8f76b4a38c5ca651db6aa84701f307b63b4e0d78342ad4219c7e4`
for `headers-accept`, and
`120fa187be19d9962f0926633453784691731018a2bf936ddb4e29101b79c4a7`
for `mediatype`.

Neither dependency type crosses the public SDK facade boundary. The
crate accepts and returns Axum's `Router`, which is its deliberate optional
framework boundary; all DID values, resolver behavior and wire bodies remain
Identus-owned. Public and wire compatibility therefore preserves every core
type and JSON shape while adding only a new optional HTTP package. The adapter
is host-tested and excluded from portable target matrices in this slice.

## Security, privacy and maintenance evidence

Both direct crates are MIT licensed and exact release provenance is pinned
above; the NeoPRISM oracle is Apache-2.0. Axum, `headers-accept` and
`mediatype` contain no authored unsafe. Axum's normal cone does reach scoped
unsafe in `matchit 0.8.4` for route-tree value references/`Send`/`Sync`
invariants and `sync_wrapper 1.0.2` for pin projection and its exclusive-access
`Sync` contract. These established framework internals are not exposed by the
SDK facade; the SDK adds no unsafe and the fixed route removes caller control
from tree construction. No native code is present in the selected normal
feature cone.

Fresh lockfile-based `cargo deny --locked check` reports advisories, bans,
licenses and sources all okay; only existing unmatched-allowance/duplicate-Syn
warnings remain. Current Nix `cargo audit --deny warnings` loads 1,242
advisories and reports no vulnerability across 161 locked crate dependencies.
These checks, exact checksums and reachable unsafe/native inspection provide
the supply-chain evidence before merge.

Maintenance, release and security posture is acceptable for an experimental,
unpublished adapter: Axum is current and declares an MSRV below the etalon;
`headers-accept` is active but small and does not declare an MSRV, so exact
Rust 1.98 integration tests are authoritative. The protocol or draft currency
is explicit: DID Resolution remains a Candidate Recommendation Draft and must
be refreshed before publication or certification.

### Resource and privacy policy

The aggregate bytes of all repeated `Accept` fields are checked before
parsing and capped at 8 KiB. At most 32 media ranges are accepted. Header
values must be visible/valid HTTP text. A private preflight recognizes quoted
sections, comma boundaries and parameters and rejects duplicate or malformed
`q` values before `headers-accept` performs selection. Valid but unsupported
ranges yield 406; malformed or oversized input yields 400.

The route is fixed at `/{did}`. Invalid path extraction or DID grammar returns
a redacted standard `invalidDid` envelope. Non-empty queries return
`invalidOptions`; no option is silently discarded. Handler and router fallback
responses vary on `Accept`. Failure and deactivated responses use the full
result media type.
Document responses require the resolver's metadata `contentType` to be
present and equal to the negotiated representation; absence or mismatch is a
500 internal-error envelope rather than mislabeled bytes.

The resolver itself controls I/O, cancellation and method-specific limits.
This adapter adds no timeout, request concurrency, authentication, rate-limit
or response compression policy; those remain host responsibilities and must
be explicit before internet exposure.

## Rejected or deferred candidates

`accept-header 0.2.3`, `negotiator 0.1.0`, an unchanged donor copy and a full
local negotiation implementation are rejected or deferred for the reasons in
the candidate table and ADR. OpenAPI, bounded query option decoding,
dereferencing, registration, portable targets and publication are deferred to
named consumers under parent #10 rather than bundled into this slice.

## Open questions and blockers

No research blocker prevents implementation. Stop rather than merge on:
unbounded header work, permissive malformed `q`, incorrect W3C status/body
mapping, leaked caller input in errors, public candidate types, an unapproved
unsafe/native dependency path, advisory/license failure, or a production
server-runtime edge. Query option support, OpenAPI and publication require
separate child issues under #10.

Rollback is one focused PR revert removing the crate, workspace dependencies,
rulebook/inventory entry and canonical capability. No stored representation or
wire migration is required because no existing public type changes.

## Evidence commands

Exact commands and unrun checks are separated. Repository and donor `rg`,
`git`, `wc` and SHA-256 inspection located the
contracts and divergences. Official W3C/RFC sources were retrieved on the
recorded date. `cargo info --verbose` confirmed current candidate versions,
features, licenses and declared MSRV. Cargo integration, source checksums,
resolved-cone inspection, advisory/license gates and 15 deterministic focused
tests pass. The complete local Nix matrix passes at reviewed implementation
head `d31e8c395f10f9e90c1b46c9a9f0bca3e88cdb30`: 653 workspace tests pass
with 22 configured skips and 129 KMP-compat tests pass with one configured
skip. The host-only HTTP crate is intentionally outside the WASM, Android and
iOS package selectors; their existing portable core package set passes. Local
Nix omits incompatible x86_64-Linux, which remains hosted-CI evidence.
