## ADDED Requirements

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
