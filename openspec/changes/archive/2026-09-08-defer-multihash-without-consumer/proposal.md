## Why

The dependency portfolio currently instructs agents to adopt `multihash`
because the crate is technically cohesive, but the SDK has no runtime consumer
of multihash bytes. The only apparent consumer was an inaccurate source comment:
`did:key` uses multibase-encoded multicodec key bytes, not multihash. Acting on
the current record would add policy and migration cost without deleting useful
code or delivering consumer behavior.

## What Changes

- Require every new production dependency to name a current SDK capability or
  consumer and the concrete correctness, maintenance or code-reduction payoff.
- Reclassify `multihash 0.19.5` from `adopt` to `conditional-adopt` until a
  focused DID-method issue proves a normative multihash requirement.
- Correct the inaccurate `did:key` documentation and immutable upstream
  provenance without changing the existing public value or its behavior.
- Record the future activation gate for algorithm codes, digest lengths,
  capacity, canonical varints, wire representation and migration behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `dependency-research-readiness`: adds a consumer-payoff gate before a
  technically suitable crate can become a production dependency.

## Impact

This is a specification and decision correction. It changes no Cargo
dependency, public signature, serialization, runtime behavior, target claim or
consumer repository. Future agents cannot infer multihash adoption from
technical fit alone; a named method profile must activate a new issue first.
