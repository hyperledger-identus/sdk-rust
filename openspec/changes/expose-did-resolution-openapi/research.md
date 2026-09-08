# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation at base revision
`094b074d625bca2cd469ae29b38652324f78e94b`,
`identus-did-resolver-http` exposes a state-closed fixed `GET /{did}` Axum
router. It implements three response representations, strict RFC-shaped
content negotiation, bounded common and extension resolution options, W3C
status projection and `Vary: Accept`. It has no OpenAPI feature or document.

NeoPRISM revision `d4608fe3d662ebaf425c4a6380f6d241cffe74c3`
(Apache-2.0) is the donor-shape oracle. Its
`lib/did-resolver-http` uses optional `utoipa 5.4.0`, macros and OpenAPI derives
in its DID Core crate. It documents a dynamic route but says query options are
unsupported. The SDK must adapt that intent rather than copy stale behavior or
move OpenAPI annotations into chain-neutral DID Core.

## Normative sources

- [OpenAPI Specification 3.1.0](https://github.com/OAI/OpenAPI-Specification/tree/42a9e3d4eddade52363a5c4fac852e80681c2fe5)
  is the pinned source revision.
- [W3C DID Resolution](https://github.com/w3c/did-resolution/tree/71a50058090417f9947b9f13985fc8b561a4ad59)
  is pinned at revision `71a50058090417f9947b9f13985fc8b561a4ad59`.
- RFC 9110 governs representation negotiation and `Vary` behavior.
- ADR 0090 and ADR 0091 define the implemented router, negotiation, projection
  and query-option contracts that documentation must not exceed.

The OpenAPI document is descriptive evidence for the implemented GET binding.
It does not create a new protocol, transport or compatibility promise.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `utoipa` typed OpenAPI model without macros | 5.5.0 / tag `2492086d40ad2b488b00db39724d45a92cb7863a` | `adopt` | Current stable release, OpenAPI 3.1 model, Rust 1.75 floor, dual MIT/Apache-2.0 license and only one new resolved package when defaults are disabled. | A breaking 5.x defect, advisory, maintenance loss, unsupported target or smaller maintained typed model with better evidence. |
| NeoPRISM Utoipa pattern | 5.4.0 / `d4608fe` | `oracle` | Proves consumer demand and composition, but default macros and DID Core annotations would widen coupling and its contract omits current query behavior. | NeoPRISM adopts the SDK document and exposes an additional generic composition requirement. |
| Handwritten `serde_json::Value` document | repository-local | `not-adopt` | Avoids one package but gives up typed OpenAPI construction and shifts schema-shape maintenance into unchecked string keys. | Utoipa becomes unavailable or materially broadens its runtime/dependency surface. |
| Utoipa derive/path macros | 5.5.0 default feature | `not-adopt` | Adds `utoipa-gen`, `syn`, `quote` and macro expansion although the workspace needs one fixed operation. | The adapter grows enough operations that handwritten typed builders become less auditable than generated definitions. |

## Compatibility and dependency evidence

The new `openapi` feature is off by default. Default and no-default consumers
retain their existing public API and resolved runtime behavior. Feature users
receive one additive function returning `utoipa::openapi::OpenApi`; exposing
that type is deliberate in this already Axum-coupled outer adapter and lets
hosts merge documents without an SDK-specific wrapper. This facade boundary is
limited to the optional outer adapter. The fixed router
signature and wire responses do not change. Rollback removes the feature,
function and dependency with no data or transport migration.

`utoipa 5.5.0` declares Rust 1.75, below the SDK's Rust 1.98.1 MSRV/etalon. With
`default-features = false`, its direct dependencies are `serde`, `serde_json`
and `indexmap`; all are already in the resolved workspace lock. The resulting
direct and resolved dependency cone adds only `utoipa` itself and excludes
`utoipa-gen`. No native code, build script or network runtime is introduced.
The feature stays host-only with the existing adapter; it makes no WASM/mobile
support claim.

## Security, privacy and maintenance evidence

The document is built from constants and contains no request, DID, resolver,
secret or PII data. It performs no input parsing, I/O, network access or dynamic
path interpolation. Deterministic structural tests prevent the document from
claiming unimplemented methods, representations, options or status outcomes.

Source inspection of registry releases 5.4.0 and 5.5.0 found no `unsafe`, FFI
or native-linking use in Utoipa Rust sources. Default macros are disabled. The
5.5.0 tag points to a GitHub-verified commit, was released 2026-05-04, and its
license and provenance are recorded as MIT OR Apache-2.0 and tag `2492086d`.
Supply-chain policy and the lockfile checksum
remain authoritative. Security posture is limited to documentation integrity;
the adapter's existing request bounds and redacted errors are unchanged.

## Rejected or deferred candidates

Handwritten JSON and Utoipa macros are `not-adopt` for the reasons above.
OpenAPI annotations in `identus-did`, a generated UI, dynamic path rewriting,
POST and dereferencing descriptions are deferred because those would add
coupling or advertise behavior not implemented by this slice.

Protocol and draft currency are explicit: OpenAPI 3.1.0 is a final published
specification, while the pinned W3C DID Resolution document remains a Candidate
Recommendation Draft and must be reviewed again before publication.

## Open questions and blockers

No blocker prevents implementation. Method-specific query extensions remain
described but cannot be enumerated. Hosts that nest the fixed router own any
path-prefix rewrite when merging this document; the SDK does not accept an
unbounded dynamic path.

Stop rather than merge if default/no-default graphs gain Utoipa, macro/native
code becomes reachable, the document advertises unimplemented behavior, DID
Core gains an OpenAPI edge, serialization is nondeterministic or any request
data can enter the document.

## Evidence commands

Commands run before implementation include `scripts/factory doctor`, `rg` and
Git inspection, `cargo search utoipa`, `cargo info utoipa@5.4.0 --verbose`,
`cargo info utoipa@5.5.0 --verbose`, registry source/manifest inspection,
`git ls-remote` for the Utoipa and OpenAPI revisions, and locked Cargo tree
inspection. Nix, nextest and cargo-deny are unrun because Nix is unavailable in
this shell. Hosted fast and weekly supply-chain/target checks remain unrun until
push or schedule.
