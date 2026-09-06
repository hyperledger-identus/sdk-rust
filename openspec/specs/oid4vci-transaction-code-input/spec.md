# oid4vci-transaction-code-input Specification

## Purpose
TBD - created by archiving change add-oid4vci-transaction-code-input. Update Purpose after archive.
## Requirements
### Requirement: Transaction Code input presence is bound explicitly

The SDK SHALL consume one `CredentialOfferWithPreAuthorizedServer`, one
optional caller-owned `String`, and one `TransactionCodeInputLimits` to create
a `CredentialOfferWithPreAuthorizedTokenInput` only when all requirements in
this capability succeed.

When the offered Pre-Authorized Code grant contains a `tx_code` object,
including `{}`, input SHALL be present. When the grant omits that object, input
SHALL be absent. Missing required input and unexpected input SHALL fail with
distinct static errors. The transition SHALL NOT invent, normalize, trim,
prompt for, compare, serialize, or submit a value.

Success SHALL prove only offer/input presence agreement. It SHALL NOT prove
Transaction Code correctness, issuer or server trust, safe out-of-band
delivery, client eligibility, request readiness, replay safety, or successful
issuance.

#### Scenario: required input advances the state

- **WHEN** the bound offer contains a `tx_code` object and the caller supplies
  one valid bounded owned value
- **THEN** the consuming transition returns a prepared input state that owns
  the predecessor and records input presence

#### Scenario: absent requirement accepts only absent input

- **WHEN** the bound offer omits `tx_code` and the caller supplies no input
- **THEN** the consuming transition succeeds with input presence false

#### Scenario: presence disagreement fails closed

- **WHEN** required input is absent or unrequested input is supplied
- **THEN** no prepared input state is returned and the mismatch has a distinct
  static error

### Requirement: Transaction Code material is independently bounded and erased

`TransactionCodeInputLimits` SHALL accept only a positive maximum decoded
UTF-8 byte length and SHALL default to 256 bytes. A supplied `String` SHALL be
wrapped in zeroizing owned storage before validation. An empty or oversized
value SHALL fail with a distinct fieldless error, and both failure paths SHALL
erase the owned allocation on drop.

Accepted input SHALL remain byte-for-byte opaque. The SDK SHALL NOT enforce
advertised `input_mode` or `length` in this state; those fields and
`description` SHALL remain available through the predecessor as issuer UI
guidance, while the Authorization Server remains responsible for validating
the actual code.

The raw accepted value SHALL have no public accessor and no Serde contract.
The success state SHALL expose only whether input is present. Later in-crate
request construction MAY borrow it through a crate-private boundary.

#### Scenario: exact byte boundary succeeds without content interpretation

- **WHEN** a non-empty opaque input occupies exactly the configured UTF-8 byte
  maximum
- **THEN** it is accepted without trimming, normalization, mode checks, or an
  extra content copy

#### Scenario: invalid limits or content fail after zeroizing ownership

- **WHEN** the configured maximum is zero or supplied input is empty or exceeds
  the maximum, including by multibyte UTF-8 content
- **THEN** construction fails with the corresponding static error and no raw
  input enters the error

### Requirement: Prepared input state remains redaction-safe and portable

`CredentialOfferWithPreAuthorizedTokenInput` SHALL own and expose its
`CredentialOfferWithPreAuthorizedServer` predecessor by reference, expose
input presence only, and implement a data-free Debug representation. Every new
failure SHALL map to a stable static `oid4vci.*` code/message and SHALL echo no
Transaction Code, Pre-Authorized Code, issuer, endpoint, description, or
retained JSON through Debug, Display, the core error bridge, or serialization.

The transition SHALL add no new dependency, manifest, lockfile, feature,
parser, or existing-limit change and SHALL preserve Rust 1.85, browser-WASM,
Android ARM64, and iOS ARM64 portability. HTTP, async, runtime, crypto, DID,
storage, chain, product, and consumer code SHALL remain outside its dependency
cone.

#### Scenario: sensitive success and failure diagnostics are data-free

- **WHEN** predecessor values and supplied input contain distinct canaries and
  the success state plus every new error surface are formatted
- **THEN** no canary is present and the input is not serializable

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the state transition passes without dependency-cone, feature,
  target, or downstream drift
