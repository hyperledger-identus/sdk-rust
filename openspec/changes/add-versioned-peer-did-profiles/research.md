# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation of SDK-Rust has bounded generic DID values and
documents but no peer DID package. SDK-TS is the active Identus consumer
implementation for numalgo 2. Available Rust crates cover different subsets
and bring public facade and dependency ownership that does not fit the SDK.
Consumer evidence therefore requires a narrow Rust facade rather than copying
any donor boundary.

## Normative sources

- DIF peer DID draft
  `decentralized-identity/peer-did-method-spec@c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8`.
- DID Methods WG proposal source
  `decentralized-identity/did-methods@90850c3e700c4714afd7e91816c4530e4631ebb3`.
- SDK-TS compatibility donor
  `hyperledger-identus/sdk-ts@4bf86ebf69d5e96616a148e4c973f831f95fa38e`.
- Affinidi `did-peer` 0.7.5, repository tag commit
  `3bca5b6358d05b07773ee6445834427b0411d083`.
- `equs-vcx-did-peer` 0.1.0, repository tag commit
  `7c4b1f849027f18408dd42bcc044503db061b7ef`.

Earlier planning text said `w3c-cg/did-methods`; the pinned revision is in
`decentralized-identity/did-methods`. This change corrects provenance without
changing pinned content. Primary source URLs are
https://github.com/decentralized-identity/peer-did-method-spec/tree/c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8,
https://github.com/decentralized-identity/did-methods/tree/90850c3e700c4714afd7e91816c4530e4631ebb3,
and
https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e.
Each pinned source revision is immutable input to this profile decision.

The DIF draft defines numalgo 2 as dot-delimited purpose-coded multibase keys
plus an optional base64url service block, and numalgo 4 as a short multihash
over the exact multibase-encoded JSON input document plus an optional long-form
document segment. The WG proposal nominates only numalgo 4 for current
standardization. SDK-TS remains compatibility evidence; its DTOs, errors and
historical component names are not Rust architecture.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| SDK-owned `identus-did-peer` | `adopt` | Exact generic DID integration, explicit profiles, finite work and target-neutral ownership. | A maintained narrow crate matches both profiles and every SDK hard gate. |
| `did-peer` 0.7.5 | `oracle` | Maintained numalgo-2 behavior, but Affinidi DTO/secret/WASM coupling, no numalgo 4 and panic sites. | Its public boundary and dependency posture become a strict subset of the SDK contract. |
| `equs-vcx-did-peer` 0.1.0 | `oracle` | Numalgo-4 coverage, but a broad VCX type cone and no declared MSRV. | A narrow maintained release declares compiler/target policy and removes VCX coupling. |
| SDK-TS peer implementation | `conditional-adopt` | Required numalgo-2 consumer vectors. | Donor behavior conflicts with pinned protocol evidence. |
| Exact JSON input-document bytes | `conditional-adopt` | Numalgo 4 hashes encoded JSON and the draft defines no cross-language canonical JSON algorithm. | A future spec defines canonical JSON. |
| Generic unversioned peer API | `not-adopt` | Would conceal normative/compatibility divergence. | Both profiles converge under a superseding specification. |

No oracle becomes a production dependency. Adapted fixtures require exact
source path, license and transformation records.

## Compatibility and dependency evidence

`identus-did-peer` depends inward on `identus-did` plus existing bounded
base64/base58/SHA-256 facilities. Generic DID never depends on the method crate.
Numalgo 4 validates a JSON object without root `id`, checks typed local
identifiers/references, encodes it, and hashes the exact encoded-document text.
Parsing verifies exact bytes before JSON allocation. Long-form resolution
contextualizes local references; short-form resolution requires caller-supplied
hash-matching long-form material and never opens I/O.

Numalgo 2 consumes explicit public multicodec keys and services. Purpose-coded
elements are validated before decode; key bytes, codecs, counts, services and
aggregate work are finite. Resolution projects canonical `identus-did` values.
The compatibility module name prevents accidental use as the current profile.

The direct and resolved dependency cone is captured from the implementation
lockfile. Expected direct versions and features are workspace `base64 0.22`,
`bs58 0.5.1` with `alloc`, `serde/serde_json 1`, and `sha2 0.10`; no oracle
package is reachable. Public and wire compatibility are profile-specific. The
public facade boundary uses SDK-owned Rust values and stable errors. MSRV is
1.89 and target evidence covers native, `wasm32-unknown-unknown`, iOS and
Android library builds.

## Security, privacy and maintenance evidence

All encoded bytes, decoded bytes, JSON nodes/depth, keys, services, element
counts and projected items have public finite limits. Algorithm/profile dispatch
precedes base decoding. Numalgo-4 digest comparison is constant-time over the
fixed 32 bytes. Public errors omit DID text, keys, services and document bytes.
The crate owns no secrets, entropy, persistence, network, executor, clock,
logging or global state.

Supply-chain evidence is a lockfile and feature-cone receipt proving both
oracle crates absent. Reachable unsafe and native code must remain within
already-governed generic dependencies. Maintenance belongs to the SDK-Rust
peer-method owner; release remains disabled until quality and authority gates
pass. The security posture is fail-closed parsing, finite work, redacted errors
and no custody or effects. Rollback removes the additive crate before release.

## Rejected or deferred candidates

Direct adoption of either oracle, one unversioned API, numalgos 0/1/3, private
key generation, DIDComm behavior, relationship storage and consumer mutation
are rejected or deferred. A generic method bundle is rejected because it
couples unrelated protocols and release cycles.

## Open questions and blockers

No planning blocker remains. Implementation must settle conservative default
limits inside generic DID/document ceilings and stop if an official vector
cannot be reproduced without weakening the pinned profile. Draft protocol
currency is explicit: a new normative revision is a reconsideration trigger,
not an automatic upgrade.

## Evidence commands

Exact planning commands are `scripts/factory research-ready
add-versioned-peer-did-profiles`, `scripts/factory constraints-ready
add-versioned-peer-did-profiles`, `scripts/factory validate
add-versioned-peer-did-profiles`, and `scripts/factory preflight
add-versioned-peer-did-profiles --issue 523 --write`. Unrun checks at planning
time are Rust tests, Clippy, rustdoc, target builds, fuzzing, benchmarks,
dependency reports, independent review and hosted CI.

Claude is preferred when available. If Claude is missing, unauthenticated,
quota-limited or session-limited, Codex CLI runs immediately against the same
exact commit. A reviewer limit is never a delivery blocker.
