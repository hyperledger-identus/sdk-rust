# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/155
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective: any future multihash facade must stay generic
and chain-neutral. `SDK-DELIVERY-001` is satisfied by issue #155 and this
spec-first lifecycle. Rust 1.98.1 and all current security, resource and target
constraints remain unchanged because no dependency or behavior is added.

## Introduced or changed constraints

No repository-wide constraint value changes. Dependency research gains a
consumer-payoff rule: technical fitness alone cannot authorize a production
dependency. A focused issue must name the current capability or consumer and
the implementation or correctness risk being replaced.

## Introduced or changed limitations

The existing `Multihash` value remains an unvalidated opaque-byte placeholder.
Its lowercase-hex formatting and serde are existing SDK behavior, not a
multihash or DID-method wire-format promise. The SDK does not currently provide
multihash structural validation or algorithm policy.

## Consumer and product impact

There is no source, binary, wire or runtime consumer impact. Agents gain a
clear stop condition that prevents speculative integration. A future DID
method must open a separate issue and define its normative and migration
requirements before implementation.

## Activation and rollback

The decision activates when issue #155's documentation-only PR merges into
`develop`. Reverting that PR restores only the prior research wording; no
runtime rollback or data migration is required.

## Evidence

Normative did:key and multihash formats, exact crate provenance, repository and
consumer searches, the existing public API, compiler/cone history, security
ownership and the future activation gate are recorded in `research.md`.
