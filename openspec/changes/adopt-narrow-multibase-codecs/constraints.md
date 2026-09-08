# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/156
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective: both codecs are chain-neutral and private
inside the generic DID crate. `SDK-COMPAT-004` and `SDK-COMPAT-005`
remain effective because the integrated graph must pass exact Rust 1.98.1 and
supported targets. `SDK-SEC-001` remains effective: SDK code adds no unsafe
block, while the dependency's unreachable mutable-string target is attributed
and reviewed. `SDK-SEC-003` remains effective because the 4 KiB recognized
carrier ceiling executes before allocation. `SDK-DELIVERY-001` is satisfied by
issue #156 and this specification-first change.

## Introduced or changed constraints

No repository-wide constraint value changes. The recognized
`publicKeyMultibase` property now permits only canonical `z` and `u`
forms no larger than 4 KiB with non-empty decoded payloads. Any future prefix
requires its own named consumer, normative source and dependency/conformance
review.

## Introduced or changed limitations

- Valid multibase registry encodings other than `z` and `u` are rejected.
- Carrier validation does not interpret multicodec, key type, curve, length or
  cryptographic validity.
- Recognized carriers above 4 KiB are rejected to bound Base58's measured
  quadratic decode/re-encode cost; a larger post-quantum or composite profile
  requires its own size and latency evidence.
- `bs58 0.5.1` has no declared MSRV and its source repository has not changed
  since March 2024; exact Rust/target/security gates are the evidence.
- The dependency contains a scoped unsafe mutable-`str` output path that the
  SDK does not call or expose.
- did:key, did:peer, Multikey and secret-key material remain outside the
  effective capability.

## Consumer and product impact

Consumers keep the same public `Option<&str>` accessor and exact JSON for
valid values. Inputs previously accepted solely because they were printable
may now fail as invalid known verification material. This deliberate
pre-release tightening prevents malformed key carriers from entering later
verification stages.

## Activation and rollback

The decision activates only when issue #156's implementation PR passes all
local and hosted gates and merges into `develop`. Reverting that PR restores
the loose validation and removes `bs58`; no accepted-value migration exists.
No claim extends to `main`, release or downstream adoption.

## Evidence

The research record covers the current consumer, final/draft normative
sources, exact artifacts, features, cone, maintenance, licenses, unsafe reach,
resource bounds, target plan, compatibility, facade, rollback and explicit
stop conditions.
