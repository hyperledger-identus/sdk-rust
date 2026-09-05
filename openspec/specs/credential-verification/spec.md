# credential-verification Specification

## Purpose

Define bounded, format-neutral evidence from credential verification without
performing verification or conflating evidence validity with product trust.

## Requirements
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

### Requirement: Verification execution is asynchronous and object-safe

The SDK SHALL define an `#[identus::port]`-marked `CredentialVerifier`
capability whose `verify` method returns a boxed `Send` future borrowing the
verifier and request inputs. The port SHALL be object-safe and usable through
`Arc<dyn CredentialVerifier>` without selecting an executor, async macro,
network stack or concrete format dependency.

The future success value SHALL be the existing canonical
`VerificationReport`. The port-owning credential crate SHALL depend on
`identus-derive` only for the build-time port marker.

#### Scenario: unrelated async adapters share one capability

- **WHEN** dummy Midnight-shaped and unrelated-format verifiers are stored as
  trait objects and invoked
- **THEN** both SHALL return canonical reports through the same runtime-neutral
  future contract without importing either adapter implementation

### Requirement: Verification requests expose least authority without copying artifacts

`CredentialVerificationRequest<'a>` SHALL borrow the validated format, payload
and optional detached proof from a `CredentialEnvelope`. It SHALL expose no
credential private material, clone no artifact byte buffer, and render only
the format plus payload/proof lengths in Debug output.

#### Scenario: verifier cannot receive holder private material

- **WHEN** an envelope containing payload, detached proof and private material
  is converted to a verification request
- **THEN** the request SHALL reference the first two verification artifacts,
  SHALL provide no private-material accessor, and SHALL not render any artifact
  contents

### Requirement: Completed evidence is distinct from operational failure

`CredentialVerificationResult` SHALL contain a canonical report on completed
evidence evaluation. Malformed structure, issuer/key mismatch, invalid proof,
temporal invalidity, unacceptable status evidence and schema failure SHALL be
represented as Failed report stages; an unperformed check SHALL be NotChecked.

The error channel SHALL be a data-free `CredentialVerificationError` with only
`UnsupportedFormat`, `Unavailable` and `Internal` classes. These SHALL map to
static `credential.verification_*` errors: unsupported SHALL use
`ErrorKind::Unsupported`, and unavailable/internal SHALL use
`ErrorKind::Internal`. No operational error SHALL use
`ErrorKind::VerificationFailed` or carry format, payload, proof, endpoint,
cause or product-trust data.

#### Scenario: invalid proof is a successful execution result

- **WHEN** a verifier completes and determines that credential proof evidence
  is invalid
- **THEN** its future SHALL resolve successfully with an Invalid report whose
  proof stage is Failed rather than an operational error

#### Scenario: dependency outage is not fabricated as invalid evidence

- **WHEN** a verifier cannot execute because an injected dependency is
  unavailable
- **THEN** its future SHALL return `Unavailable` without claiming that any
  credential stage failed or that the credential is trusted or untrusted

### Requirement: Exact-format verifier registry is bounded and immutable

The SDK SHALL provide a builder that binds each exact validated
`CredentialFormat` to one `Arc<dyn CredentialVerifier>`, rejects duplicate
formats, and rejects more than 64 entries. Building SHALL freeze an immutable,
clone-cheap registry. The registry SHALL expose entry count, emptiness, exact
support lookup and deterministic lexical iteration over bound format names.

Duplicate and capacity failures SHALL use zero-data `CredentialError` variants
with static `credential.*` invalid-input bridges. No rejected format value
SHALL enter an error.

#### Scenario: dummy format proves the extension point

- **WHEN** two independent validated dummy formats are bound and the registry
  is cloned
- **THEN** each exact format SHALL dispatch to its sole verifier and both
  registry values SHALL expose the same deterministic binding set

#### Scenario: ambiguous composition fails before runtime

- **WHEN** a builder registers the same exact format twice or attempts a
  sixty-fifth binding
- **THEN** setup SHALL fail with the corresponding static duplicate or capacity
  error and SHALL not replace an existing verifier

### Requirement: Registry dispatch is exact and artifact-allocation-light

`CredentialVerifierRegistry` SHALL implement `CredentialVerifier`. It SHALL
select only by exact validated format spelling, return `UnsupportedFormat` for
an unbound format, and SHALL NOT inspect payload/proof prefixes, apply fallback
order, retry another verifier or copy credential artifact buffers. Registry
setup allocation SHALL be bounded by the 64-entry ceiling and exact lookup
SHALL be `O(log n)`.

An ignored release diagnostic SHALL poll a ready dummy verifier through the
production registry and report throughput without a machine-dependent timing
threshold.

#### Scenario: declared format controls one deterministic dispatch

- **WHEN** two requests carry identical payload bytes but distinct declared
  formats
- **THEN** each SHALL reach only its exact bound verifier, independent of
  payload contents

#### Scenario: unknown format fails closed without probing

- **WHEN** a syntactically valid but unbound format is submitted
- **THEN** dispatch SHALL return `UnsupportedFormat` without invoking a bound
  verifier or reading artifact bytes
