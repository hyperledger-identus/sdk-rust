# Review: bounded OID4VCI Credential Offer grants

## Pre-implementation semantic review — 2026-09-06

### Scope and architecture

- **Pass:** the owner remains the chain-neutral, unpublished
  `identus-oid4vci` crate; the transition consumes a validated core offer and
  does not activate the quarantined umbrella crate or mutate a consumer.
- **Pass:** typed known grants do not choose or execute a flow. Metadata,
  authorization, token exchange, replay, trust, consent, storage, chain
  behavior, FFI, adoption, publication, and release remain excluded.
- **Pass:** unknown grant names/members and exact JSON remain lossless, so this
  bounded slice does not close the protocol's extension point.

### Standards and compatibility

- **Pass:** the contract matches OpenID4VCI Final section 4.1.1: absent/empty
  grants defer selection, multiple grants remain caller-selected, both known
  grant values are objects, and Pre-Authorized Code is required in its grant.
- **Pass:** a present `tx_code` is an object even when empty; optional
  `numeric`/`text`, integer length, and 300-character description rules are
  represented without confusing requirements with the later user input.
- **Pass:** RFC 8414 issuer syntax is applied to Authorization Server hints,
  while the Final metadata membership and multiple-server rules are deferred
  until metadata is available.
- **Pass:** Lace's `tx_code: null` stays visible as legacy-negative evidence;
  no released compatibility exists and no consumer is silently widened.

### Security and privacy

- **Pass:** issuer state and Pre-Authorized Code are attacker-controlled and
  bearer-adjacent. Positive limits, zeroizing ownership, explicit accessors,
  and redacted diagnostics address allocation and accidental-log threats.
- **Pass:** validation makes no freshness, single-use, replay, phishing,
  endpoint agreement, or trust claim. Section 13.6 mitigations remain future
  orchestration/issuer responsibilities.
- **Pass:** known-grant, Transaction Code, and scalar/integer type confusion
  fail before a grant-validated public value exists.

### Resource and portability review

- **Pass:** separate positive limits avoid breaking transport/core limit APIs;
  300 Unicode scalar values and 1,200 UTF-8 bytes are independently enforced.
- **Pass:** the selective scanner remains inside existing transport byte,
  depth, node, and duplicate-member ceilings and avoids a generic value tree.
- **Pass:** the dependency cone remains unchanged; no async, HTTP, crypto,
  runtime, platform, chain, or product dependency enters.

### Provenance and isolation

- **Pass:** no donor source or fixture is copied. Official and independently
  reconstructed values suffice for this grant-shape slice.
- **Pass:** Oxid and Lace revisions/status/digests are recorded as read-only
  evidence; other consumer/donor dirty state is pre-existing and untouched.

## Decision

The contract is semantically ready for implementation. No blocking finding or
protected decision remains. Structural factory validation is separate evidence.

## Post-implementation review

Pending implementation and an independent fresh review pass.
