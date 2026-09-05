# Design: narrow the DID crypto feature cone

## Context and inventory

Issue #101 starts from
`develop@ed6cbed292e27de4096adb2cb81622591ce1c7c5`. The root workspace
dependency deliberately disables `identus-crypto` defaults so consumers can
select exact capabilities. `crates/did/Cargo.toml` locally overrides that rule
with `features = ["default"]`, activating Ed25519, X25519, secp256k1, P-256,
hashing, encodings, JWK/thumbprints, COSE and derivation.

An exhaustive source and test search found no `identus_crypto` import in
`crates/did`. The DID document model stores `publicKeyJwk` as a bounded JSON
map, rejects registered private members and requires a non-empty `kty`; it
intentionally does not select curves, validate points or verify signatures.
The only crypto-related values in DID tests are inert JSON strings used to
prove representation-neutral round trips. The development-only URI oracle is
`uriparse`, not crypto.

The edge was retained mechanically when issue #98 disabled crypto defaults at
the workspace root; it did not correspond to an implemented DID capability.
Current JOSE code performs the later algorithm/key conversion through its own
explicit Ed25519/P-256 crypto dependency after DID dereferencing.

## Decisions

### D1 — remove the unused dependency

Delete `identus-crypto` from the DID manifest. Do not replace it with `jwk` or
any curve feature: the DID crate calls none of those APIs, and a narrower unused
edge would still violate dependency minimization. `identus-did` continues to
depend internally on `identus-core` and the `identus-derive` proc macro only.

### D2 — retain a structural JWK boundary

`VerificationMethod::public_key_jwk` continues to expose the recognized JSON
object without interpreting its cryptosuite. DID document construction keeps
its public-only and resource-bound checks. Algorithm-aware consumers convert
that object into their own typed key at the operation boundary; JOSE already
does so while binding the protected algorithm.

This keeps DID Core vocabulary extensible to cryptosuites the SDK does not yet
implement and prevents DID parsing from silently becoming cryptographic proof.

### D3 — add no empty feature taxonomy

The DID crate currently has no optional production behavior, so it will retain
no `[features]` table. Its default and `--no-default-features` surfaces are the
same by construction. Introducing curve-named passthrough features would imply
that DID owns algorithm enablement and increase consumer configuration without
changing code.

### D4 — make the absence of the edge executable

Extend the existing manifest conformance test to assert that the exact current
internal dependencies of `identus-did` are `identus-core` and
`identus-derive`. This catches accidental reintroduction of crypto or another
workspace crate without spawning Cargo or duplicating manifest parsing.

The layer rule remains permissive enough for future evidence-backed domain
dependencies. A future focused issue must update the exact assertion when it
adds a real dependency.

### D5 — use existing platform gates and focused graph evidence

The existing workspace default, MSRV, browser-WASM, Android ARM64 and iOS ARM64
gates already compile `identus-did`. A dedicated Nix feature surface would
duplicate those builds because the crate has no features. This change instead
runs focused default/no-default checks and tests, records both Cargo trees, and
runs the full existing gate set. The graph receipt is evidence for this
unreleased change, not a permanent build-time or binary-size promise.

## Risks and trade-offs

- A downstream crate that accidentally relied on `identus-did` to activate
  crypto defaults will have to request its own crypto capabilities. That is the
  desired Cargo ownership model and affects no API of the unpublished SDK.
- Structural JWK validation does not prove curve points or signature fitness.
  The existing API and documentation already make this distinction; this
  change preserves it rather than weakening validation.
- Exact internal dependency assertions require an intentional spec/test edit
  when the DID crate gains a legitimate dependency. This is useful review
  friction at the reusable domain boundary.

## Rollback and migration

Before publication, rollback is a manifest/test/documentation revert. There is
no public API, serialized state, protocol or downstream migration. Consumers
should already request the crypto features they use directly.
