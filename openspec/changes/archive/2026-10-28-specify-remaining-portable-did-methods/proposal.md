# Specify the remaining portable DID methods

## Why

Peer and PRISM method capabilities now have bounded SDK-owned implementations,
but the canonical `IDR-040` row still groups `did:key`, `did:jwk`, and
`did:web` behind one planning issue. Those methods have different security,
effect, and dependency boundaries. Treating them as one implementation slice
would either couple effect-free methods to networking or make a third-party SSI
facade the SDK's architecture by accident.

## What changes

- Pin the current normative drafts and independent Rust implementations used as
  conformance oracles.
- Define strict first profiles for `did:key` and `did:jwk`, each reusing the
  existing DID and cryptographic value types without a broad SSI dependency.
- Define `did:web` as pure URL transformation and document validation around a
  small injected fetch port; concrete networking remains an adapter concern.
- Record separate implementation issues and sequence them from the smallest
  effect-free surface to the effect-bearing method.
- Move the canonical backlog owner from planning issue #555 to first delivery
  issue #557 while preserving `IDR-040` as in progress.

## Capability

### Modified capability

- `ssi-upstream-program`: require independent profiles, owners, and effect
  boundaries for the remaining portable DID methods.

## Non-goals

This change adds no crate, production API, dependency, protocol implementation,
fixture, target claim, release promise, downstream adapter, or consumer change.
It does not define private-key generation, custody, HTTP execution, DNS/TLS
policy, redirects, caching, product trust, or language-SDK DTO compatibility.

## Delivery

Planning issue #555 is decomposed into ordered implementation issues #557
(`did:key`), #558 (`did:jwk`), and #559 (`did:web`). Each child requires its own
OpenSpec contract and preflight receipt before implementation. The canonical
backlog advances ownership after each child is delivered rather than claiming
the method family complete from this planning change.
