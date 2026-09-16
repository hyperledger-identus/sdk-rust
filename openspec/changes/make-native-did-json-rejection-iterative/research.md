# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The current implementation at exact revision
`508917b0416147668b9d12949eec987f0b82946b` validates DID JSON through
borrowed bounded walks, but rejected caller-owned `serde_json::Value` trees use
serde_json's recursive destruction. The public/native audit found these owned
entry paths:

| Family | Native owned-JSON entry paths | Rejection owner |
| --- | --- | --- |
| Document | `VerificationMethod::new`; `Service::new` endpoint/extensions; `DidDocumentBuilder::context` and `extensions` through `build` | `document.rs` |
| Resolution problem/metadata | `DidResolutionError::new`; `DidResolutionMetadata::new`; `DidUrlDereferencingMetadata::new`; `DidDocumentMetadataBuilder::extensions` through `build` | `resolution.rs` |
| Dereferenced content | `DereferencedContent::new`; `DidUrlContentMetadata::new` | `resolution.rs` |
| Query options | `ResolutionOptions::new`/builder; `DereferencingOptions::new`/builder | `query.rs` |
| Registration | `RegistrationPublicData::new` | `registration.rs` |

Once accepted, every retained value is already bounded by the existing DID
depth/node/member/string contracts. Typed values subsequently composed into a
larger document/result therefore do not carry hostile depth. Wire-slice entry
points retain their byte and streaming depth/node/member gates and are not the
source of this native cleanup gap.

## Candidate decisions

| Candidate | Decision | Evidence and reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Crate-private generic rejection guard plus iterative `Value` dismantler | `adopt` | Matches the reviewed `identus-crypto` JWK technique, needs no unsafe/dependency, starts guarding before any validation, and centralizes the only traversal algorithm. | A standard-library or serde_json facility provides guaranteed iterative destruction. |
| Constructor-specific iterative walkers | `reject` | Duplicates security-sensitive traversal and invites family drift. | Type-specific ownership cannot project JSON roots into the shared primitive. |
| `serde_stacker` | `reject` | Helps recursive serde calls, not destruction of an already-owned native `Value`; does not own the failing boundary. | It adds a documented iterative-drop facility. |
| `stacker` or larger thread stacks | `reject` | Native/unsafe dependency, platform coupling, and finite stack growth mask rather than remove the unbounded recursive cleanup path. | Never for this requirement; only a distinct bounded recursion decision could reconsider it. |
| Add a shallower semantic JSON limit | `reject` | Existing accepted limits are already part of the contract; changing them would break compatible values without solving early rejection cleanup. | A separate standards/consumer requirement changes accepted DID JSON budgets. |

## Compatibility and dependency evidence

No Cargo manifest, dependency version/cone, feature, MSRV, target, public type,
constructor signature, serde form, wire encoding, error variant, or accepted
limit changes. The internal guard owns a value only until validation succeeds;
success returns the exact original allocation. Failure drains arrays/objects
through a bounded explicit `Vec<Value>` worklist. No raw value is logged or
formatted.

The direct and resolved dependency cone is unchanged. `identus-did` already
directly depends on workspace `serde_json = "1"`; the lock resolves
`serde_json 1.0.150` from the crates.io index with checksum
`e8014e44b4736ed0538adeecded0fce2a272f22dc9578a7eb6b2d9993c74cfb9`.
No feature is added or changed. The facade boundary remains SDK-owned public
DID types and errors; no serde_json implementation helper becomes public.

## Security, privacy and maintenance evidence

The attacker controls nesting depth but existing byte/node semantics control
accepted values. The iterative cleanup visits each rejected node once, moves
children rather than clones them, and performs no recursion or unsafe code.
The worklist may allocate proportionally to hostile breadth already allocated
by the caller; outer allocation remains explicitly caller-owned. Errors remain
stable and redacted. The implementation is portable standard Rust plus the
already-required serde_json dependency, so the existing Linux fast and native
weekly target policies remain unchanged.

Supply-chain evidence is unchanged: no new package, build script, native code,
unsafe exception, license, advisory surface, or registry source enters the
lock. Maintenance stays inside the DID crate; release/publication remains
prohibited and this security hardening creates no support or certification
claim. Rust 1.98.1 remains the temporary etalon, not a new MSRV promise.

## Normative sources

- Issue #297 and accepted ADR 0125 at repository base
  `508917b0416147668b9d12949eec987f0b82946b`.
- Public primary source URL: [serde_json `Value` documentation](https://docs.rs/serde_json/1.0.150/serde_json/value/enum.Value.html).
- `crates/crypto/src/jwk.rs` iterative rejected-extension guard at the same
  repository base, used as internal design evidence only; Apache-2.0.
- `serde_json::Value` from the already locked workspace dependency; no new
  source or fixture is imported.
- Complete source search over `crates/did/src/{document,resolution,query,registration}.rs`
  and their public constructors/builders on 2026-09-17.

License and provenance remain the repository Apache-2.0 source plus the
existing serde_json MIT/Apache-2.0 registry package at the locked checksum. No
donor code or fixture is copied. Protocol or draft currency is unchanged: the
existing W3C DID Core/Resolution and DID Registration profile versions stay
exactly as pinned by their prior slices.

## Rejected or deferred candidates

The table rejects dependency-based stack growth, duplicated walkers, and
changed semantic limits. Abandoned infallible builders and direct construction
of public JSON-bearing enum variants remain ordinary caller-owned values rather
than validation rejections; this slice covers every path that returns a DID
validation `Err`. If a later audit demonstrates a recursive destructor outside
those rejection paths, `SDK-LIM-007` broad disclosure must be restored and a
focused issue opened.

## Open questions and blockers

No blocker remains. The exact public constructor inventory is closed for the
current source revision; implementation must restore broader disclosure if a
new owned-JSON rejection path is found during review.

Rollback is atomic: remove the guard/integrations/tests and restore the former
`SDK-LIM-007` native depth obligation. No accepted value, public API, wire data,
or consumer migration is involved.

## Evidence commands

Exact commands run before implementation: `scripts/factory doctor`, repository
`rg` source/signature inventory, issue/ADR/spec/constraint review,
`cargo tree -p identus-did`, and the crypto JWK pattern inspection. Commands
unrun before implementation are the new focused hostile-depth tests, normal DID
tests, format, strict Clippy, workspace tests/build/docs, compatible Nix
closure, final factory/OpenSpec checks, exact-diff review, and protected
exact-head CI; all are tasks, not inferred evidence.
