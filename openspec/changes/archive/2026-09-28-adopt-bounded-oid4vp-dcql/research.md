# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation at pinned source revision
`73274255307eb31825a4da3788043c0ff39b6a43` keeps unpublished
`identus-oid4vp` owns bounded by-reference ingress, runtime-neutral Request URI
retrieval, signed compact JAR verification, and exact client/wallet-nonce
correlation. `VerifiedRequestObject` retains a completely scanned bounded JSON
payload but deliberately does not interpret DCQL. It is the named production
consumer required by ADR 0156.

The existing research fixture pins exact `siros-dcql 0.3.0`. On 2026-09-29,
crates.io still reports 0.3.0 as latest, BSD-2-Clause, Rust 1.82, pure Rust, and
the spike retains its exact lock/checksum and 12-package registry cone.

## Normative sources

OpenID4VP 1.0 Final with current errata sections 5, 6, and 7 requires exactly
one of `dcql_query` or a scope representing DCQL, non-empty unique credential
and claim identifiers using alphanumeric/underscore/hyphen, required non-empty
credentials, required `meta`, non-empty claims/values/set options, valid
references, and bounded consumer processing. Section 6.4 defines holder-binding,
claim, credential-set, and combination selection. The 2026-09-29 publication
at `https://openid.net/specs/openid-4-verifiable-presentations-1_0.html` is the
normative protocol source.

## Candidate decisions

| Candidate | Decision | Evidence |
| --- | --- | --- |
| Exact `siros-dcql 0.3.0` | `conditional-adopt` | Cohesive path/selection/set engine; no I/O/unsafe; portable spike; small cone. |
| Candidate public types/parser | `not-adopt` as SDK boundary | Unbounded collections/strings, verifier-bearing diagnostics, missing `meta` and invalid identifiers tolerated. |
| Local engine | `not-adopt` unless adapter duplicates candidate | Would independently maintain roughly 1,421 production source lines and the subtle Section 6.4/7 rules. |
| Broad OID4VC frameworks | `oracle` only | ADR 0156 records coupling, MSRV, provenance, and bounded-contract mismatch. |

Adoption becomes final only if the strict facade remains substantially smaller
than the engine behavior avoided. The engine stays a private dependency; no
candidate type, diagnostic, feature, or support promise crosses the crate API.

## Compatibility and dependency evidence

The exact version is `siros-dcql = "=0.3.0"` with no feature activation. Its
MSRV is 1.82, below the SDK 1.89 floor. The spike passed primary/MSRV and
WASM/iOS/Android target checks. It has two direct dependencies (`serde` and
`serde_json`) and a 12-package resolved dependency cone; only the candidate
package name is incremental to the root graph. The additive public API and wire
input remain Identus-owned behind the facade boundary. Rollback is deletion of
an unpublished capability.

## Security, privacy and maintenance evidence

The facade validates request bytes through the already bounded JAR parser and
adds independent limits for serialized query bytes, credential queries,
claims, path components, values, sets/options/references, identifier/format/key
bytes, credential inventory, query-by-credential and claim-by-credential work,
and emitted combinations. It rejects the candidate's known tolerance gaps
before execution and validates cross-references.

Credential claim resolution remains caller-owned through an Identus trait. A
private adapter converts only bounded SDK path components and maps every
caller/candidate failure to static categories. Results retain identifiers and
selected paths only; `Debug`, errors, component metadata, and metrics expose
counts and static enums rather than verifier, credential, or claim values.

Candidate source forbids unsafe code and contains no native code or I/O. The
BSD-2-Clause license and crates.io provenance are compatible; Cargo-deny is the
supply-chain gate. Its narrow maintenance and release posture is acceptable
behind a replaceable private seam. Protocol/draft currency is fixed to
OpenID4VP 1.0 Final plus current errata.

## Rejected or deferred candidates

Full Affinidi, Impierce, and Spruce stacks remain `oracle`. Candidate public
models and a duplicate local engine are `not-adopt`. Reconsideration trigger:
upstream loses maintenance/security viability, target support, Final semantics,
or the facade duplicates most candidate behavior.

## Open questions and blockers

None for this slice. Format-specific `meta` and trusted-authority semantics are
not silently accepted: the initial exact-format policy rejects non-empty
format metadata or trusted-authority constraints until separately injected
policy is designed. Full Authorization Request validation remains a later
state and cannot be inferred from a `ValidatedDcqlQuery`.

## Evidence commands

Completed commands: `cargo search siros-dcql --limit 5`,
`cargo info siros-dcql@0.3.0`, exact locked research metadata/source inspection,
and the repository SIROS checker. Final root dependency, license/advisory,
primary, MSRV, target, tests, Clippy, docs, and factory commands are unrun checks
until implementation and remain mandatory tasks.
