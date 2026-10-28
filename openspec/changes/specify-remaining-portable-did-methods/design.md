# Design

## Independent crate graph

```text
identus-did-key ──> identus-did + identus-crypto
identus-did-jwk ──> identus-did + identus-crypto
identus-did-web ──> identus-did + injected fetch port
```

The method crates do not depend on one another. Their delivery sequence reduces
risk; it is not a runtime dependency chain. Generic crates never depend on a
method crate. A shared multicodec abstraction is deferred until delivered code
demonstrates identical semantics and errors in at least two owners.

## Profile boundaries

`identus-did-key` owns strict base58-btc identifier parsing, the selected
multicodec/key-length matrix, deterministic Multikey document projection, and
method-local resolution. It reuses SDK public-key validation and never accepts
private material.

`identus-did-jwk` owns bounded base64url/JSON decoding, public JWK validation,
the fixed `#0` projection, and `use`-conditioned relationships. It preserves the
input DID and treats canonical generation as optional convenience rather than
an identity-uniqueness claim.

`identus-did-web` separates a pure DID-to-HTTPS transform and exact document-ID
validator from retrieval. Its port accepts an already validated request and a
bounded response contract. The crate cannot choose an HTTP client, executor,
DNS/TLS policy, redirects, proxy, cache, or product trust.

## Evidence architecture

Each child owns normative and negative vectors, property invariants, a bounded
parser fuzz target, deterministic benchmarks, host and supported target builds,
and differential oracle comparisons. `did:web` additionally owns an in-process
fake-fetch E2E that proves transform, retrieval, bounded parse, exact ID binding,
and redacted error behavior without an external network.

The Rust DTO and error contracts remain idiomatic SDK-owned surfaces. Language
compatibility, if needed, is a thin adapter with a separate migration window.

## Backlog handoff

Planning issue #555 closes after this packet integrates. `IDR-040` points to
#557. Delivery of #557 advances it to #558; delivery of #558 advances it to
#559. Only #559 can evaluate whether the row's remaining-method scope is
delivered. This creates one active owner without duplicate umbrella issues.

## Stop boundary

This change modifies governance, planning, and research only. It adds no Cargo
member, source, fixture, dependency, target claim, release artifact, network
adapter, binding, or downstream mutation.
