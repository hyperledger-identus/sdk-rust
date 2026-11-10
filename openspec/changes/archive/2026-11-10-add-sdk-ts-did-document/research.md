# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-10
Source retrieval date: 2026-11-10
Research blockers: none

## Problem and existing implementation

`identus-did::DidDocument` is an immutable validated W3C DID Core structural
model. Untrusted JSON enters through `from_json_slice` or `from_json_str`, which
enforce a 256 KiB byte ceiling, duplicate decoded object-name rejection, wire
depth/node/member/live-name budgets, then semantic validation. The model
preserves scalar-versus-array cardinality for contexts, controllers, and
service types; reference-versus-embedded verification relationships; URI,
map, and mixed-set service endpoint forms; and bounded open extensions.

Verification methods retain suite-defined public properties, reject registered
private JWK members, reject simultaneous JWK and multibase carriers, and bound
all open strings/trees. Services, relationships, controllers, aliases, method
IDs, and service IDs enforce their documented uniqueness and collection rules.
Rejected hostile JSON is cleaned iteratively, including early-error paths. All
document failures bridge to the redacted public code `did.invalid_document`.

The current private `identus-wasm-did` facade exposes only DID/DID URL values
at binding API version 1. The minimal document addition is an owned generated
view containing exact document ID and deterministic normalized JSON produced
only after Rust validation. TypeScript builds convenience DTOs from that
Rust-produced value and frees the raw view exactly once.

The current implementation therefore already owns the generic DID Document
grammar, validation, serialization, resource policy, and redacted error
boundary. The missing capability is a narrow WASM/TypeScript projection that
does not create a second parser or change the Rust model.

## Normative sources

The primary source URL is W3C DID Core 1.0:
<https://www.w3.org/TR/did-core/>. It is a W3C Recommendation dated
2022-07-19, with the current errata linked by the Recommendation. The protocol
or draft currency decision is to implement that Recommendation and applicable
published errata only; DID Core 1.1 drafts and method-specific specifications
are deferred until a separately versioned contract selects them.

The canonical implementation source is the repository-local
`identus-did::DidDocument` at base
`f7e777b8556ba4c82bdc272faf2551e7bfb9d8f8`. The compatibility donor is
`hyperledger-identus/sdk-ts@4bf86ebf69d5e96616a148e4c973f831f95fa38e`
and is evidence rather than normative authority.

## Compatibility and dependency evidence

The donor is `@hyperledger/identus-sdk@8.1.4` at immutable revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`. Its effective document route is
`Domain.DIDDocument`; the model source content hash is
`59a787c3c26a0ce2ce9b0eacc64f3e3938f663aa31ace3ee20e91306f234bf67`.
The donor is compatibility evidence, not normative authority.

The donor `fromJSON(any)` does not bound or validate input, loses top-level
controller and aliases, drops unresolved references, mutates embedded-object
prototypes, leaves top-level methods as raw objects, permits private JWK
members and ambiguous verification material, and leaks caller values through
generic errors. `cloneWithNewDID` rewrites IDs but not method controllers. The
new contract SHALL NOT reproduce those defects or historical Apollo, Castor,
Mercury, Pluto, Pollux, or Edge Agent public names.

The adapted generic donor journeys are the PRISM document parse assertion
(`PrismDID.test.ts`, content hash
`18a25ef8e1d5a7586f02b98a932d919b8efe73f24d8c97da916a835f3cebd330`)
and its generic service projection. Method creation/resolution tests are
excluded to their method capabilities; resolver, DIDComm routing, signing-key,
and secret-custody tests are platform-owned. The donor's fifteen cloud-agent
Cucumber features do not exercise the generic value contract and remain
ledgered external exclusions.

License and provenance are recorded for both inputs: SDK-Rust is Apache-2.0;
the pinned SDK-TS donor revision is Apache-2.0 repository content and is not
copied wholesale. Adapted assertions retain immutable source paths, content
hashes, and transformation/disposition records. W3C text is referenced, not
vendored.

The supported Rust MSRV remains the repository-pinned Rust 1.98.1 toolchain;
this change does not lower or raise it. The direct and resolved dependency cone
is unchanged for portable crates: `identus-wasm-did` already depends on
`identus-did`, generated web/node artifacts feed the existing private
TypeScript package, and React reuses those providers. No portable crate gains
WASM, npm, TypeScript, React, network, storage, or product dependencies.

Supply-chain evidence consists of the unchanged Cargo/npm lockfiles, existing
repository source-distribution and dependency-policy checks, exact source
revisions, closed generated-artifact inventories, and the full local flake
gate. A dependency or lockfile change is not planned; if implementation proves
one necessary, research and the dependency evidence will be reopened before it
is accepted.

## Candidate decisions

The candidate comparison selects canonical Rust validation, normalized JSON,
strict immutable TypeScript values, explicit lossy-number rejection, and reuse
of the existing private target graph. All alternatives are recorded below.

## Rejected or deferred candidates

| Candidate | Decision | Reason |
| --- | --- | --- |
| Parse raw input in TypeScript first | `not-adopt` | Loses duplicate keys and bypasses the Rust wire boundary. |
| Expose generated document classes | `not-adopt` | Leaks allocation, binding shape, and lifecycle into the public contract. |
| Rust-validated normalized JSON plus typed DTO | `adopt` | Keeps Rust authoritative while offering strict idiomatic immutable values. |
| Normalize every scalar/array to arrays | `not-adopt` | Erases wire cardinality required by the portable model and round-trip evidence. |
| Silently coerce large JSON integers | `not-adopt` | JavaScript would corrupt accepted extension values beyond its safe integer range. |
| Reject unsafe integer DTO projection explicitly | `adopt` | Preserves normalized JSON and records a bounded adapter deviation without corruption. |
| Copy donor class methods and resolver behavior | `not-adopt` | Mixes method/network/platform policy into a portable document slice. |
| Reuse existing target package and CI graph | `adopt` | Adds a capability without a second artifact or duplicated lifecycle implementation. |

The reconsideration trigger for a rejected candidate is new normative W3C
behavior, a proven inability of the Rust model to retain required wire
distinctions, or measured target evidence showing the owned projection cannot
meet the private train's safety/resource contract. External support,
publication, React Native, resolver, method, or agent requirements trigger a
new slice rather than silently expanding this one.

## Public mapping

The package adds `DidDocumentValue`, `DidVerificationMethodValue`, a tagged
reference/embedded `DidVerificationRelationshipValue`, `DidServiceValue`,
typed service endpoint/context forms, and bounded immutable JSON values.
`DidDocumentValue` includes deterministic normalized JSON plus typed fields and
separate extension maps. Optionality and scalar/array cardinality remain
observable. The provider adds `parseDidDocument(json)`; there is no resolver,
network request, mutator, method execution, or implicit fallback.

The WASM contract maps structural rejection to `did.invalid_document` and
unsafe JavaScript integer projection to a separately versioned adapter error.
Neither code, message, observation, nor generated error contains caller data.
Normalized JSON is deterministic semantic JSON from the Rust model; it is not
the original source bytes and does not claim RFC 8785 canonicalization.

## Security, privacy and maintenance evidence

One immutable vector packet covers minimal/full documents, every relationship
class, controllers/aliases, JWK and multibase methods, all service endpoint
shapes, cardinality, extensions, deterministic serialization, duplicate keys,
private key material, malformed shapes, unsafe numbers, byte/depth/node/member
ceilings, and redaction. One provider-neutral assertion body runs unchanged in
Node ESM/CommonJS, Chromium, and React SSR/hydration. Rust-owned negative and
resource evidence supplements the donor's sparse generic tests.

A new document fuzz target proves no panic, accepted-input deterministic
reparse, bounds, public-only material, and redacted errors. TypeScript uses an
iterative bounded validator/freezer to avoid recursive stack exhaustion. Size,
startup, ordinary/near-limit latency, normalized-output size, and RSS remain
measurement-only until a later support decision accepts budgets.

Maintenance, release and security posture remain deliberately conservative:
Rust owns validation and redaction; the package is private, unpublished,
non-default, and additive; generated allocation is closed exactly once; and no
caller data enters diagnostics or observations. The rollback is removal of the
unreleased document provider, mapping, packet, and target assertions while the
already-merged DID syntax canary remains unchanged. Publication, support, or
release activation needs a separate protected decision.

## Evidence commands

Exact commands and unrun checks are tracked in the train receipt. The planned
local sequence is:

```text
./scripts/factory research-ready add-sdk-ts-did-document
./scripts/factory constraints-ready add-sdk-ts-did-document
./scripts/factory validate add-sdk-ts-did-document
./scripts/factory preflight add-sdk-ts-did-document --issue 502 --write
cargo test -p identus-wasm-did
cargo test -p identus-did
./scripts/factory check
./scripts/factory ready add-sdk-ts-did-document
nix flake check
```

Target-package, Node ESM/CommonJS, Chromium, React SSR/hydration, fuzz,
differential, artifact, coverage, resource, and mutation commands will be
recorded exactly as the implementation exposes them. At research time all
implementation and closeout commands are unrun checks; no result is inferred.

## Open questions and blockers

There are no blockers to the repository-local private train. External SDK-TS
consumer wiring, accepted budgets, default/support/publication decisions, and
React Native remain separate gates under issues #492 and #502.
