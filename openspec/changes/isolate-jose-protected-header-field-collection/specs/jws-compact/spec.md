# JWS protected-header field ownership

## ADDED Requirements

### Requirement: Protected-header field collection has private bounded ownership

The SDK MUST keep the closed protected-header member vocabulary and partial
raw-field state behind private owners while the Serde visitor performs only
bounded entry iteration and delegates finalization.

#### Scenario: A supported protected header is collected

- **WHEN** a complete bounded JSON object contains supported unique members
- **THEN** each typed value is retained exactly and one required algorithm plus
  at most one key reference is finalized into the same validated public header.

#### Scenario: Multiple member and finalization faults coexist

- **WHEN** duplicate, unknown, invalid-value, missing-algorithm, or ambiguous
  key-reference faults coexist
- **THEN** the same current static error is returned according to source-order
  collection followed by required-algorithm and key-reference finalization
- **AND** no header value enters diagnostics or becomes usable.

#### Scenario: Collection remains bounded and protocol neutral

- **WHEN** the private owners classify, decode, and finalize header fields
- **THEN** existing decoded-header, string, JWK, certificate-chain, attestation,
  trust-chain, duplicate, and closed-vocabulary rules remain authoritative
- **AND** no new algorithm, trust/profile meaning, dependency, allocation class,
  ambient I/O, dynamic dispatch, synchronization, metric waiver, or
  forwarding-only helper chain is added.
