# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

Current implementation: at pinned source revision `a7af3fa2f3afca89b88af0b499e3cf5150760360`, invocation parsing rejects
`transaction_data`, authorization routing does not retain it, and DCQL rejects
every non-empty `meta` object. The private DCQL engine understands format as a
string but cannot establish Final format-metadata or transaction binding.

## Normative sources

OpenID4VP 1.0 Final, published 2025-07-09, is normative:
https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html.
Sections 5.1 and 8.4 require each transaction-data value to be an exact
base64url-encoded JSON object with a type and non-empty credential IDs, reject
unknown or malformed mandatory semantics, and bind the value or a type-defined
representation into the corresponding presentation. Section 8.5 defines
`invalid_transaction_data`. DCQL section 6.1 makes `format` and object-valued
`meta` mandatory and format-specific. Sections 10.1 and 11.1 define
`vp_formats_supported` as format-keyed capability metadata. Appendix B defines
the `dc+sd-jwt`, `mso_mdoc`, `jwt_vc_json`, and `ldp_vc` identifiers and their
format-specific metadata/binding behavior.

The exact encoded transaction-data string is retained because format profiles
can bind that representation rather than decoded bytes. Foreign Identus SDKs
are differential oracles only; Rust DTOs and errors remain canonical.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| SDK-owned `CredentialFormat` discriminant | `adopt` | Gives stable known formats and bounded extensions without importing a proof engine. |
| caller-injected format capability registry | `adopt` | Keeps metadata and binding rules cohesive with their credential crates. |
| SDK-owned transaction envelope | `adopt` | Enforces common Final structure, exact encoding, bounds, references, and redaction once. |
| opaque arbitrary JSON accepted by core | `not-adopt` | Cannot reject unknown mandatory semantics or prove format compatibility. |
| implement SD-JWT/mdoc proofs here | `not-adopt` | Violates crate ownership and couples OID4VP to format engines. |
| general OpenID/OAuth crates | `not-adopt` | No existing candidate exposes this consumed-state, bounded, format-port contract. |

## Compatibility and dependency evidence

The direct and resolved dependency cone is inspected with `cargo tree -p
identus-oid4vp --edges normal --depth 2`; the design adds no dependency or
feature. `serde_json`, `base64`, `zeroize`, and the private DCQL engine remain
at exact Cargo.lock versions. Cargo deny, RustSec, source-distribution, Nix,
and the release train own license, provenance, supply-chain, and package evidence.
Rust 1.89 remains MSRV and Rust 1.98.1 the etalon; all/no-default and portable
WASM/iOS/Android target checks remain required. The Identus facade prevents
private engines and foreign DTO/error types from becoming public or wire contracts.

Public and wire compatibility expands before publication. Rollback removes
the port and retained state while preserving prior empty-meta routing.

## Security, privacy and maintenance evidence

Encoded/decoded bytes, item counts, JSON depth/nodes/members, names, values,
credential references, registered formats/types, and callback work are bounded.
Duplicates, padding, malformed base64url/JSON, unknown transaction types,
unknown mandatory format metadata, missing references, incompatible selected
inputs, and adapter failures fail closed with static redacted errors. Exact
encoded values and metadata never appear in `Debug`, `Display`, errors, or metrics.

The API is a pre-publication consumer-visible expansion. Existing empty-meta,
no-transaction callers retain a default path. Rollback removes the new port and
state while preserving prior routing. No dependency, unsafe code, runtime,
network, filesystem, global registry, native library, or background work is added.

## Quality evidence

Implementation must provide clean-room request-to-selection E2E evidence;
generic, SD-JWT-shaped and mdoc-shaped port canaries; malformed/oversized and
redaction vectors; properties over exact encoding and reference linkage; fuzz
coverage; near-limit benchmarks; all/no-default, rustdoc, Clippy, portable,
factory/Nix, and exact-head Claude-or-Codex CLI review evidence.

Maintenance and security ownership remains with OID4VP maintainers; release is
still gated and the protocol/draft currency is pinned to Final 2025-07-09.

## Rejected or deferred candidates

Bundled format proof engines, global registries, arbitrary unvalidated JSON,
foreign SDK DTOs/errors, transport, persistence, consent policy, and product
trust are rejected or deferred. No donor code or fixture is copied; clean-room
cases are Apache-2.0 repository work.

## Reconsideration triggers

Reconsider if Final errata changes transaction hashing/linkage, a credential
crate needs a richer no-JSON handoff, two independent adopters require the same
format implementation, or an external crate satisfies MSRV, portability,
bounds, dependency-cone, license, and SDK-facade rules.

## Open questions and blockers

None. Concrete format proof APIs and product consent remain downstream.

## Evidence commands

Planning uses repository inspection, the pinned Final source, `cargo tree -p
identus-oid4vp --edges normal --depth 2`, `scripts/factory research-ready`,
`constraints-ready`, `validate`, `check`, and preflight. Tests, portable
builds, property/fuzz/benchmark evidence, Nix, hosted CI, and CLI review are
explicitly unrun at planning time.
