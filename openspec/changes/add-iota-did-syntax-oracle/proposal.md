# Add an isolated IOTA Identity DID syntax oracle

## Why

The SDK owns a bounded DID and DID URL parser, but its conformance corpus should
also be exercised against a mature independent Rust implementation. ADR 0069
already classifies IOTA Identity as an oracle; issue #235 narrows that decision
to syntax only so the dependency cannot become a second production DID model.

## What changes

- Add a separately locked, unpublished fixture for exact `identity_did 1.5.1`.
- Compare an attributable DID/DID URL corpus against `identus-did` and record
  intersections and normatively adjudicated divergences.
- Add an isolation/drift check and complete dependency, target, security,
  maintenance and supply-chain evidence.
- Record whether retaining the oracle has enough value to justify its ongoing
  maintenance cost.

## What does not change

No production dependency, public API, wire contract, support target, DID method,
downstream repository, release or certification state changes.

## Authority

- Issue #235, child of #164.
- ADR 0069 and the SSI repository-disposition policy.
- W3C DID Core 1.0 and RFC 3986 remain normative; candidate behavior is not.
