# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation has one runtime consumer: the DID document
`publicKeyMultibase` property. Its current validator accepts any non-empty
printable ASCII string up to 16 KiB, including unknown prefixes and strings
that do not decode. The public accessor exposes the exact string. Future
did:key work is planned but does not yet exist as a method parser or resolver.

The implementation must improve the current consumer without claiming future
multicodec or key semantics.

## Normative sources

- [Controlled Identifiers 1.0](https://www.w3.org/TR/controller-document/#multibase)
  defines the `u` base64url-no-pad and `z` base58-btc common normative
  encodings and notes weaker interoperability for other entries.
- [did:key Method v0.9](https://w3c-ccg.github.io/did-key-spec/#did-key-identifier-syntax)
  defines `z` and `u` fingerprints containing a multicodec key identifier
  followed by raw public-key bytes.
- [Multibase registry](https://github.com/multiformats/multibase) distinguishes
  final, draft, experimental, deprecated and reserved encodings.
- [bs58 release source](https://github.com/Nullus157/bs58-rs/tree/7d3c9282d2595612e5474df93dd0e017db9b684f)
  is the exact commit behind the assessed published tag.

Controlled Identifiers 1.0 is the final generic verification-material source.
did:key v0.9 is a method-specific draft/oracle and does not activate key
semantics in this change.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `bs58` plus existing `base64` | 0.5.1 / `7d3c9282`; 0.22.1 | `adopt` | Reuses both required final base algorithms, adds one package, supports alloc without std and keeps SDK prefix/canonicality policy cohesive. | Reconsider bs58 on advisory, target break, maintenance risk or a narrower maintained equivalent; widen encodings only for a named normative consumer. |
| `multibase` | 0.9.3 / `cdeda567` | `not-adopt` for this boundary | Correct generic API but nine-name incremental cone imports unused draft/experimental algorithms and build macros even with defaults disabled. | Upstream feature-slices z/u engines, or multiple named consumers require enough registry encodings to justify the cone. |
| Local Base58 implementation | none | `not-adopt` | Reimplements a closed conversion algorithm and expands security/conformance ownership. | Only if maintained candidates fail targets/security and no compatible alternative exists. |
| Printable-ASCII validation | `develop@b88b5ffe` | `not-adopt` as production behavior | Does not prove a multibase value and accepts invalid public key carriers. | Restore only if standards or compatibility evidence requires an explicitly opaque extension property. |

## Compatibility and dependency evidence

The exact new dependency is `bs58 0.5.1`, defaults disabled, `alloc` only.
Its minimal normal cone is one package. `base64 0.22.1` is already in the
workspace lock and normal graphs. A combined no-std probe compiled on Rust
1.98.1 and `wasm32-unknown-unknown`.

The comparison lock resolved 19 packages total because it deliberately
included both approaches. `multibase 0.9.3` accounted for fourteen package
names in its standalone normal/build tree and nine names absent from the
current workspace lock. Its Base45 dependency is no longer an MSRV blocker
under Rust 1.98.1, but passing the compiler gate does not make unused
algorithms cohesive.

This change intentionally narrows accepted known material to canonical `z`
and `u` values no larger than 4 KiB. Existing valid examples remain exact;
placeholder fixtures that are not valid Base58 must be replaced with real
encoded bytes. The public
type, accessor and JSON member remain unchanged. Rollback is one dependency
and validator revert.

## Security, privacy and maintenance evidence

`bs58 0.5.1` has crates.io checksum
`bf88ba1141d185c399bee5288d850d63b8369520c1eafc32a0430b5b6c287bf4`,
annotated tag object `54fd9a73d03b0d62921b27d47e5a0288485cba58`,
release commit `7d3c9282d2595612e5474df93dd0e017db9b684f`, and
packaged `src/lib.rs` SHA-256
`eee508c54fc64a0ffb58d6db610559671e3a3d23f6a98a9a003f399701ab4217`.
The licenses are MIT OR Apache-2.0.

The repository is not archived; its latest commit is dated 2024-03-19. The
codec is small and mature, but this is a maintenance caveat, not an activity
claim. Exact pinning, RustSec and target gates plus an explicit replacement
trigger are required.

The crate contains a scoped unsafe conversion for encoding directly into a
mutable `str`. The owned `String` implementation converts through
`Vec<u8>` and `String::from_utf8`; the SDK uses only owned string/vector
paths and exposes no target trait or upstream error. The SDK adds no unsafe
code. A fresh comparison-lock scan with the repository's pinned Nix
`cargo-audit` loaded 1,242 advisories and reported no vulnerability.

For comparison, `multibase 0.9.3` has crates.io checksum
`7e0e4a371cbf1dfd666b658ba137763edb23c45beb43cfe369b5593cd6b437b6`,
release commit `cdeda567ce45c8d37ced0fd4eac7f7bac5a445c3` and
packaged `src/lib.rs` SHA-256
`8774a315f35de850552d1b56ce1eee3c67fa8e2a6a6276f4e9f705f878995fd9`.

License and provenance are therefore anchored to exact registry artifacts and
release revisions rather than a moving branch. The direct and resolved
dependency cone is one new package for the selected composition, compared
with nine new package names for the rejected generic candidate. Supply-chain
evidence consists of the exact checksum/source comparison, maintenance and
unsafe inspection, license policy, RustSec scan and the mandatory integrated
Cargo/Nix gates.

Public and wire compatibility remains deliberately narrow: the existing
string accessor and JSON spelling are unchanged for accepted values. The
facade boundary discards decoded bytes and upstream errors after validation;
no dependency type becomes part of the SDK API.

A release-mode local resource probe measured Base58 decode plus canonical
re-encode at approximately 1 ms for 1,024 encoded bytes, 27 ms for 4,096 and
332 ms for 16,383. The original 16 KiB property ceiling therefore permits
disproportionate work when repeated inside the 256 KiB document envelope.
The implementation ceiling is 4 KiB and executes before codec allocation.
This diagnostic is machine-specific evidence, not a portable latency promise.

## Rejected or deferred candidates

`multibase 0.9.3` and a local Base58 implementation are rejected for this
boundary for the reasons in the candidate matrix. Additional registry prefixes
are deferred until a named protocol consumer pins their status and behavior.
Multicodec/key semantics and the draft did:key method remain deferred to a
method-specific specification.

## Open questions and blockers

No research blocker prevents the bounded implementation. Implementation must
prove canonical behavior for both prefixes, non-empty decoded material, the
4 KiB precheck, unchanged valid serialization, stable redacted errors, exact
locked cone and all supported targets. Any dependency-feature drift, public
upstream type leak, unbounded allocation, acceptance of padding/unknown
prefixes, or key-semantic claim is a stop condition.

## Evidence commands

- Repository searches located the single current validation seam and every
  multibase-shaped fixture.
- `cargo search`, `cargo info`, crates.io archives and GitHub API/tag
  inspection established versions, features, checksums, commits, licenses and
  maintenance dates.
- `cargo tree --no-dedupe` measured the combined normal/build cone and the
  current lock comparison identified one versus nine incremental names.
- Source inspection located the only candidate unsafe block and proved the
  planned owned paths do not dispatch through the mutable-string target.
- A release-mode `std::time::Instant` probe measured worst-case non-zero
  Base58 decode/re-encode at 1 KiB, 4 KiB and the former 16 KiB ceiling.
- A Rust 1.98.1 no-std host/WASM probe and pinned Nix RustSec scan passed.
- Integrated host/mobile/WASM, focused/full, dependency, documentation and Nix
  evidence remains an implementation gate and is not pre-claimed here. Those
  are the exact commands and unrun checks at research time; final evidence
  must distinguish executed checks from intentionally unrun host-incompatible
  derivations.
