# ADR 0082: defer multihash until a method consumes it

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#155](https://github.com/hyperledger-identus/sdk-rust/issues/155)
- **Supersedes:** ADR 0061's production-adoption disposition for `multihash`
- **Decision authority:** IDR-040 and the consumer-payoff dependency gate

## Context

ADR 0061 classified `multihash 0.19.5` as an immediate production candidate.
The crate is technically attractive: it is a small, `no_std`-compatible
structural codec, does not implement hashing or select algorithms, declares
Rust 1.81 and has a two-package minimal normal dependency cone.

The adoption premise was nevertheless wrong. `identus-did::Multihash` is an
unused, infallible bytes newtype introduced to exercise the bytes-newtype
derive. Its documentation called it the value underlying `did:key`, but the
[did:key method](https://w3c-ccg.github.io/did-key-spec/#did-key-identifier-syntax)
defines its fingerprint as multibase encoding of a multicodec public-key type
followed by raw public-key bytes. A
[multihash](https://github.com/multiformats/multihash#format) instead contains
an unsigned-varint hash-function code, unsigned-varint digest length and the
digest bytes. Repository and consumer searches found no current runtime value
with those multihash semantics.

Adding the crate now would not replace code or fix reachable behavior. It would
force the SDK to invent allowed hash codes, digest sizes, capacity, canonical
varint, representation and migration policies before a DID method requires
them.

## Decision

`multihash 0.19.5` is **conditional-adopt**, not adopted or rejected. No Cargo
dependency or structural validation is added until a focused DID-method issue:

1. pins a normative profile and proves that its value is multihash rather than
   multicodec-prefixed key bytes;
2. names the current SDK capability or consumer and the implementation or
   correctness risk that reuse replaces;
3. defines allowed codes and digest lengths, maximum capacity, canonical
   varints, text/wire representation, stable errors and migration behavior;
4. refreshes exact version/features, compiler, target, dependency-cone,
   license, maintenance, advisory, unsafe and native-code evidence; and
5. keeps the crate private behind an Identus-owned facade.

The existing public `Multihash` placeholder remains source- and wire-compatible
for now. Its documentation becomes explicit that it owns opaque bytes and does
not guarantee multihash structure, method semantics or a standards wire
format. A future consumer must decide validation and migration explicitly; it
cannot silently reinterpret the existing lowercase-hex serde representation.

This issue also establishes a general dependency rule: technical fitness is
necessary but not sufficient. A production dependency needs concrete consumer
payoff.

## Provenance retained for reconsideration

- crate: `multihash 0.19.5`, MIT;
- crates.io checksum:
  `577c63b00ad74d57e8c9aa870b5fccebf2fd64a308a5aee9f1bb88e4aea19447`;
- annotated tag object: `08383e219747d4b8c14f40f94d3a2340a5cc3229`,
  unsigned;
- release commit: `e2044a2e3aa27c2a08d3bad492fccd4babf10310`;
- minimal normal cone: `multihash` plus `unsigned-varint`;
- declared compiler floor: Rust 1.81.

These are dated research inputs, not permanent security or compatibility
approval. The activating issue must repeat the checks against its actual
lockfile and supported targets.

## Consequences

- The SDK avoids a speculative dependency and premature algorithm policy.
- AI agents receive an objective stop/go rule instead of equating a good crate
  with a product requirement.
- Exact candidate research remains reusable when a real method appears.
- The placeholder name remains imperfect until a consumer-driven API decision,
  but its neutral documentation prevents a false did:key claim without causing
  unrelated 0.0.x churn.

## Alternatives rejected

### Adopt because the crate is narrow

This satisfies technical architecture criteria but produces no current
consumer behavior and creates policy/migration obligations.

### Implement structural validation locally

This recreates a standards parser while preserving the same absence of a
consumer. It also increases code ownership without reducing dependency risk.

### Remove the public placeholder now

Removal would turn a decision correction into a public API change. It can be
considered with real usage evidence or an explicit pre-release cleanup issue.

## Verification and rollback

This decision changes documentation and dependency governance only. Existing
constructors, formatting, serde, tests and Cargo resolution remain unchanged.
Revert this focused PR to restore the former wording; no data or runtime
migration is involved. A future implementation must update this ADR and the
conditional disposition in issue #155 before adding the crate.
