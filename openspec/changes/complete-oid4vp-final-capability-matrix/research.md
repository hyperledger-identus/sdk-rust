# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The crate implements bounded by-reference invocation, effect-free Request URI
envelopes and response binding, compact signed JAR verification, DCQL
validation/selection, `direct_post` success/error construction, and
`direct_post.jwt` construction through an injected JWE port. These are real
capabilities, but the current README and issue graph do not prove complete
Final coverage.

The existing OpenID4VCI closeout establishes a useful repository pattern: one
CSV inventory, one standard-library checker, one concise report, and explicit
gap owners. Reusing that pattern reduces factory and reviewer complexity.

## Normative sources

The authority is OpenID for Verifiable Presentations 1.0 Final, published
2025-07-09, retrieved with current errata on 2026-10-28:
https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.

The matrix covers wallet-applicable capability groups from sections 5 through
15 and explicitly records Appendix A Digital Credentials API, Appendix B
format profiles, and Appendix C SIOPv2 as separate or out-of-scope profiles.
RFC 9101 and the Final document's referenced OAuth/JOSE RFCs remain normative
for signed Request Object claims and response semantics.

## Current capability findings

| Area | Finding | Disposition |
| --- | --- | --- |
| Invocation and Request URI | Bounded by-reference GET/POST profile exists | implemented |
| Signed Request Object | Signature, type, client and wallet-nonce correlation exist | partial: audience/time/replay are #544 |
| Client identifier and verifier metadata | Syntax is retained; trust/scheme proof is not bound | missing: #545 |
| DCQL and claim paths | Bounded SDK facade and deterministic selection exist | implemented for the generic JSON inventory profile |
| Plain direct post | Request-bound success/error construction exists | implemented |
| Encrypted direct post | Engine-neutral ECDH-ES/A128GCM construction exists | partial: production engine is separately owned |
| Redirect response modes | No bounded redirect response constructor exists | missing: #546 |
| Transaction/format metadata | No generic request-to-presentation port exists | missing: #547 |
| Wallet/verifier metadata publication and attestation | Not part of the current wallet-core profile | unsupported/adjacent |
| DC API, format engines, SIOPv2 combination | Separate capabilities | not applicable here |

## Candidate decisions

| Candidate | Exact version or revision | Decision | Reason |
| --- | --- | --- | --- |
| Existing OpenID4VCI CSV/checker pattern | `develop@63b23f7ebce3bab7741873b0147e7943a12ba73b` | `adopt` | Already reviewed, offline, deterministic, and suited to a flat protocol inventory. |
| New standard-library OID4VP checker | repository-authored at issue #541 | `retain-local` | It validates repository evidence and roadmap state, not reusable protocol behavior. |
| External conformance service | none | `not-adopt` | Network execution would be nondeterministic and mix transport/trust claims into traceability. |
| Donor SDK DTO/error models | SDK-TS and SDK-Swift revisions pinned by the platform inventory | `oracle` | Useful for adapter canaries, but not normative for Rust API or error design. |

## Compatibility and dependency evidence

The current implementation is `identus-oid4vp` at
`63b23f7ebce3bab7741873b0147e7943a12ba73b`. This evidence change adds no Cargo
dependency and leaves the direct and resolved dependency cone unchanged. The
checker uses Python 3 standard-library modules only. Exact version and features
for production crates are unchanged; Rust 1.98.1 remains the primary compiler
and Rust 1.89.0 remains the release-line MSRV.

Existing host, `wasm32-unknown-unknown`, iOS, and Android target evidence is
unaffected because no production Rust code or dependency changes. The new Rust
integration tests compile in the existing crate feature matrix. Public and
wire compatibility is behavior-neutral: no exported item, serialization,
error category, response body, or persisted representation changes.

The OpenID source is public standards-track material. Repository fixtures and
test adapters are independently authored under Apache-2.0. No donor bytes are
copied. SDK-TS/Swift provenance is used only through the pinned platform
inventory. There is no added package checksum, license, advisory, unsafe,
native-code, build-script, network, or supply-chain surface.

## Security, privacy and maintenance evidence

The matrix and checker process paths and bounded CSV text checked into the
repository. They execute no untrusted protocol payload, network, native code,
or dependency. Test fixtures contain public deterministic keys and synthetic
claims only. Diagnostics name stable row IDs and repository paths, never
credentials, private keys, presentation values, or consumer data.

Maintenance is one flat CSV schema plus a standard-library validator mirroring
the existing OID4VCI pattern. Release posture is unchanged: these are
engineering artifacts and cannot authorize certification, publication, target
support, or downstream adoption. Rollback deletes the additive checker,
matrix, report, and tests without API, wire, storage, or migration work.

## Consumer and adapter research

SDK-TS is the most current donor for capability discovery and SDK-Swift is the
second portability oracle. Neither is normative for Rust DTOs, errors, or
architecture. The evidence slice therefore uses independently authored
adapter-shaped fixtures that translate representative TypeScript and Swift
input shapes into existing public Rust APIs. No donor bytes, generated source,
repository mutation, or compatibility promise is required.

## Decision

Adopt the existing OpenID4VCI closeout shape with a dedicated OpenID4VP
validator. A row may claim `implemented` only when repository-local public
implementation, canonical specification, and executable test evidence exist.
Incomplete required rows require a focused open issue. The checker is offline
and deterministic; it never fetches normative or consumer repositories.

IDR-024 remains `in_progress` while missing required rows exist. IDR-031 can
move to `delivered` because issue #514 and its signed implementation are merged,
but its draft-13 limitation remains explicit.

## E2E and knowledge evidence

One clean-room reference test will execute representative invocation through
JAR verification, DCQL selection, and plain/protected response construction.
Two adapter canaries will deserialize independent TypeScript- and Swift-shaped
fixtures at the test boundary and drive the same Rust-native protocol API.
The tests prove adapter feasibility and protocol lineage, not foreign DTO or
error compatibility, HTTP interoperability, verifier trust, presentation
authenticity, or official certification.

## Rejected or deferred candidates

Markdown-only inventories drift too easily. Runtime network conformance would
mix transport and trust into the protocol core. Copying SDK-TS/Swift DTOs would
violate the normalized Rust architecture. Treating every optional Final
feature as mandatory would confuse a framework specification with a profile.

## Open questions and blockers

There are no blockers for the evidence slice. The matrix is expected to keep
IDR-024 in progress; issues #544 through #547 own the unresolved protocol
features. A production JWE adapter remains separately owned and is not a
precondition for truthful matrix publication.

## Evidence commands

```text
scripts/check-oid4vp-conformance.py
python3 -m unittest scripts/tests/oid4vp-conformance.py
cargo test -p identus-oid4vp --all-features
cargo test -p identus-oid4vp --no-default-features
scripts/factory check
scripts/factory backlog-live
```

Portable, dependency, Nix, and workspace gates run before delivery. Exact
consumer repositories are not required to execute this SDK-owned evidence.

Unrun during planning: implementation tests, portable target compilation,
Nix gates, fuzzing, benchmarks, and CLI review. They run after exact-head
preflight where proportionate. Live OpenID certification and consumer
repository mutation are intentionally not run.

## Reconsideration triggers

Reconsider the flat CSV model if Final errata can no longer be represented
without nested conditional profiles, if an official offline OpenID conformance
manifest becomes available under compatible terms, or if two independent
consumers require a versioned machine schema outside this repository.
