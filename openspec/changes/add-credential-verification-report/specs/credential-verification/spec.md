## ADDED Requirements

### Requirement: Fixed policy-neutral verification stage taxonomy

The SDK SHALL define exactly six `VerificationStageName` values in canonical
order: structural, issuer/key, proof, temporal, status, and schema. Each SHALL
round-trip its stable string. Trust, acceptance, and product policy SHALL NOT
be represented as verification stages.

#### Scenario: stage taxonomy is complete and ordered

- **WHEN** a consumer inspects the public stage list and each string spelling
- **THEN** it SHALL contain the six contracted stages exactly once in canonical
  order and SHALL contain no trust stage

### Requirement: Bounded machine-readable verification reason codes

The SDK SHALL provide an owned `VerificationReasonCode` accepting 1–128
lowercase ASCII bytes. The first byte SHALL be alphanumeric; later bytes SHALL
be lowercase alphanumeric or `.`, `_`, `-`, `:`. Validation SHALL precede the
single successful allocation, preserve accepted spelling, and reject all
other values without retaining or rendering them in an error.

#### Scenario: namespaced codes preserve spelling

- **WHEN** representative proof, status, schema, and adapter-neutral codes are
  parsed
- **THEN** accepted values SHALL preserve their exact spelling

#### Scenario: unsafe reason values fail closed

- **WHEN** a reason is empty, oversized, uppercase, non-ASCII, whitespace or
  punctuation-bearing outside the grammar
- **THEN** parsing SHALL return the static invalid-reason error without echoing
  the rejected value

### Requirement: Verification stage invariants are construction-time facts

A `VerificationStage` SHALL contain a name, Passed/Failed/NotChecked status,
and an optional reason code. Passed SHALL forbid a reason. Failed and
NotChecked SHALL require one. No public constructor SHALL produce a stage that
violates these rules.

#### Scenario: stage and reason combinations are validated

- **WHEN** all status/reason-presence combinations are constructed
- **THEN** only Passed-without-reason, Failed-with-reason, and
  NotChecked-with-reason SHALL succeed

### Requirement: Complete canonical report derives its outcome

`VerificationReport` SHALL contain exactly six stages in canonical order and
SHALL derive, not accept, its `VerificationOutcome`. Any Failed stage SHALL
produce Invalid. Otherwise any NotChecked stage SHALL produce Indeterminate.
Otherwise the outcome SHALL be Valid. Reordered or duplicate arrays SHALL be
rejected. Lookup by stage name SHALL use its canonical index.

#### Scenario: aggregate precedence is deterministic

- **WHEN** reports contain all passed, one not-checked, one failed, or both a
  failed and not-checked stage
- **THEN** outcomes SHALL be Valid, Indeterminate, Invalid, and Invalid
  respectively

#### Scenario: incomplete and non-canonical reports are unrepresentable

- **WHEN** callers construct the fixed-size input or submit a reordered or
  duplicate stage array
- **THEN** array size SHALL make missing/extra stages a compile-time mismatch
  and runtime validation SHALL reject reorder/duplication

### Requirement: Verification evidence is independent of trust

The verification report SHALL contain no trust, acceptance, issuer allow-list,
or wallet policy field and SHALL make no claim that Valid means accepted. Its
documentation SHALL state that evidence validity and relying-party trust are
independent decisions.

#### Scenario: validity and trust vary independently

- **WHEN** fixtures pair Valid with an untrusted issuer and Invalid with a
  trusted issuer outside the report
- **THEN** both combinations SHALL remain representable without modifying the
  verification outcome

### Requirement: Verification construction errors use the stable SDK boundary

Every new verification construction error SHALL map to `IdentusError` with
capability `credential`, kind `InvalidInput`, a static `credential.*` code,
and static public text. No rejected reason or runtime diagnostic SHALL cross
the bridge.

#### Scenario: all new errors bridge without caller data

- **WHEN** every reason/stage/report construction error is converted and
  formatted
- **THEN** the capability, kind, code, and text SHALL match the contract and
  SHALL contain no caller-controlled value

### Requirement: Report construction has a bounded allocation-light path

Report construction SHALL inspect at most six stages, allocate no report
collection, and store stages canonically in a fixed array. Stage lookup SHALL
be direct by canonical index. A manual release-mode diagnostic SHALL record
construction throughput without enforcing a machine-dependent time threshold.

#### Scenario: performance path is observable without flaky gating

- **WHEN** the ignored release diagnostic is run explicitly
- **THEN** it SHALL construct complete reports through the production API and
  print elapsed/throughput data without changing correctness acceptance
