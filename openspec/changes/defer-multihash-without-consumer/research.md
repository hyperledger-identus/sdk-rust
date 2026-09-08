# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation, `identus-did::Multihash`, is an infallible
`Vec<u8>` newtype with lowercase-hex
`Display` and serde. Repository and read-only consumer searches found no
runtime use beyond its module and tests. Its documentation nevertheless calls
it the canonical bytes identifier underlying `did:key`, creating the false
consumer premise behind issue #155 and ADR 0061's preliminary adoption choice.

No Cargo dependency or structural multihash parser exists today. The correction
can therefore be completed without implementation or compatibility change.

## Normative sources

- [did:key Method v0.9](https://w3c-ccg.github.io/did-key-spec/#did-key-identifier-syntax)
  specifies a multibase encoding of a multicodec public-key type concatenated
  with raw public-key bytes.
- [Multihash format](https://github.com/multiformats/multihash#format)
  specifies unsigned-varint hash-function code, unsigned-varint digest length
  and digest bytes of exactly that length.
- [`multihash 0.19.5` documentation](https://docs.rs/multihash/0.19.5/multihash/)
  describes a bare `no_std`-compatible data structure with no hashing policy.
- The pinned source revision for release tag `v0.19.5` is
  `e2044a2e3aa27c2a08d3bad492fccd4babf10310`; the published crate checksum is
  `577c63b00ad74d57e8c9aa870b5fccebf2fd64a308a5aee9f1bb88e4aea19447`.

Protocol or draft currency is method-specific: did:key v0.9 proves that method
is not a multihash consumer, while no other DID-method draft is selected by
this decision.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `multihash` | 0.19.5 / `e2044a2e` | `conditional-adopt` | Strong structural-codec fit, but no current SDK consumer or local implementation is replaced. | A focused DID-method issue pins a profile that carries multihash and defines all code, size, canonical and wire policies. |
| Existing `identus-did::Multihash` | `develop@61210a86` | `retain-placeholder` | Removing it would create unrelated API churn; expanding it would silently reinterpret existing arbitrary-byte/hex behavior. | A named consumer decides validation, migration and compatibility explicitly. |
| Implement a local codec now | none | `not-adopt` | Recreates a standard parser without a consumer and without method policy. | Same named-consumer trigger; compare against the crate then. |

## Compatibility and dependency evidence

The exact version and features assessed are `multihash 0.19.5` with default
features disabled. Its declared MSRV is Rust 1.81. The direct and resolved
dependency cone in the minimal normal graph contains two packages:
`multihash` and `unsigned-varint`. Earlier target evidence passed the
repository's supported host and portable targets. The exact Rust 1.98.1
workspace floor removes no remaining compiler blocker. Those facts establish
technical eligibility, not product need. Public and wire compatibility are
unchanged because this change adds no dependency and changes no public API,
serialized bytes, feature, target or compiler value.

If activated later, the facade boundary keeps the dependency private behind an
Identus-owned value. The activating issue must remeasure the actual lockfile cone and run
host, WASM, Android, iOS, advisory, license and unsafe checks at its own head.

## Security, privacy and maintenance evidence

The candidate does not hash data or choose algorithms. That is desirable
cohesion, but it also means the SDK would have to own code allow-lists, digest
sizes and resource limits. The multicodec table includes codes unsuitable for
every cryptographic context, so structural validity cannot imply policy or
trust. Premature adoption would turn arbitrary existing bytes and hex serde
into an ambiguous migration boundary.

License and provenance are the MIT-licensed upstream release and published
crate checksum pinned above. Unsafe and native-code evidence found no native
dependency in the minimal surface; a production integration must repeat the
reachable unsafe scan. Supply-chain evidence is limited to the exact checksum,
source revision and the parent portfolio's dated advisory query; it is not a
current audit pass. The maintenance, release and security posture is therefore
technically eligible but deliberately time-bounded. A future integration must
refresh maintenance and advisory evidence rather than treat this dated record
as a permanent security approval.

## Rejected or deferred candidates

Production adoption and local reimplementation are both deferred. Removing or
deprecating the existing public placeholder is also deferred because this issue
authorizes a decision correction, not an API break. `did:key` cannot activate
multihash work because its normative key fingerprint uses multicodec, not
multihash.

Rollback is a documentation-only revert. No dependency, persisted data or
consumer migration would need to be removed.

## Open questions and blockers

No blocker remains for this documentation/specification correction. The
method-specific code policy, digest size, capacity, representation and
migration questions are deliberate entry conditions for a future issue, not
assumptions to resolve speculatively now.

## Evidence commands

Exact commands and unrun checks are distinguished below.

- Repository searches covered `sdk-rust`, Apollo, NeoPRISM,
  `midnight-identity`, Lace ID Portal and Oxid without finding a runtime
  multihash consumer.
- The did:key method and multihash format were re-read on 2026-09-08.
- Published crate checksum and exact release commit were reconciled with issue
  #155; the earlier report SHA is explicitly corrected by this change.
- `cargo tree`, host and portable-target results from the parent dependency
  portfolio remain recorded as historical candidate evidence; this change does
  not claim to rerun dependency gates for a dependency it does not add.
- Factory, OpenSpec, text and exact-diff checks remain tasks and are not claimed
  complete until recorded in the final verification receipt.
