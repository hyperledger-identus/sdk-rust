## ADDED Requirements

### Requirement: Isolated safe DID Registration profile

The DID capability SHALL provide a chain-neutral Rust lifecycle profile based
on DIF DID Registration draft commit
`ec1bf38f7860b361eb6692c02f742b9dfc48291b`. It SHALL NOT claim direct DIF
JSON/HTTP compatibility, expose raw secret exchange, or import method, ledger,
wallet, persistence, transport, cache, or runtime policy.

#### Scenario: draft volatility remains replaceable

- **WHEN** a method implements the portable registration port
- **THEN** a future DIF wire adapter SHALL be independently replaceable without
  changing that method's resolver or dereferencer implementation

### Requirement: Bounded immutable registration requests

Create, update, deactivate, continue, and cancel SHALL be distinct immutable
request variants. Every request SHALL carry a bounded idempotency key and exact
validated method; continuation and cancellation SHALL carry a method-scoped
opaque job. Update SHALL preserve a non-empty ordered list of document
operations.

#### Scenario: invalid mutation input never dispatches

- **WHEN** a request has an empty/excessive operation list, contradictory
  method/DID/job identity, malformed identifier, or excessive public data
- **THEN** construction SHALL fail with a stable redacted registration error
  before a registrar is invoked

#### Scenario: update order remains method-owned

- **WHEN** standard and method-specific update operations are combined
- **THEN** their exact order SHALL reach the selected adapter without generic
  diffing, reordering, resolution, or an atomicity claim

### Requirement: Opaque secret and custody modes

Internal, external, and client-managed secret modes SHALL contain no private
key, seed, password, decrypted payload, or arbitrary secret bag. Internal mode
SHALL require storage or return of an opaque handle; external mode SHALL use an
opaque custody handle; client-managed mode SHALL exchange only bounded public
action data.

#### Scenario: destructive internal secret policy fails closed

- **WHEN** internal mode would neither store generated material nor return an
  opaque handle
- **THEN** request construction SHALL reject it rather than permit silent loss
  of DID control

#### Scenario: diagnostics never reveal custody values

- **WHEN** requests, results, jobs, actions, handles, or validation failures are
  formatted for diagnostics or bridged to the SDK error surface
- **THEN** opaque identifiers, payloads, documents, public-data contents, and
  all private material SHALL be absent

### Requirement: Coherent terminal and non-terminal states

A finished or failed registration result SHALL have no job. An action or wait
result SHALL have one method-scoped job. Action state SHALL correlate the
job's expected action id exactly; wait state SHALL carry no expected action and
MAY provide an advisory wait no greater than 24 hours.

#### Scenario: contradictory draft examples are rejected

- **WHEN** an action/wait result omits a job or a terminal result retains one
- **THEN** the safer SDK profile SHALL reject it as an invalid state even if a
  draft example depicts that combination

#### Scenario: completed identity matches its request

- **WHEN** a result is validated for create, update, deactivate, continue, or
  cancel
- **THEN** its DID and job method SHALL match the request and create completion
  SHALL contain a DID

### Requirement: Explicit continuation and action correlation

An action continuation SHALL contain exactly one response whose id equals the
job's expected action id. A wait continuation SHALL contain no response. The
generic layer SHALL NOT sign, decrypt, redirect, poll, sleep, or spawn retries.

#### Scenario: stale or injected action response fails closed

- **WHEN** a continuation response is absent, unexpected, or uses another
  action id
- **THEN** request construction SHALL reject it before adapter dispatch

### Requirement: Deterministic idempotency and honest cancellation

Registrar implementations SHALL bind an idempotency key to one canonical
request. Identical replay SHALL return the same logical job/outcome; a different
request with that key SHALL fail as conflict. Dropping a Rust future SHALL NOT
mean the operation was cancelled. Cancellation SHALL be an explicit best-effort
request and SHALL NOT claim rollback of irreversible work.

#### Scenario: retry cannot duplicate a mutation silently

- **WHEN** an uncertain create/update/deactivate attempt is retried with its
  original idempotency key
- **THEN** the adapter SHALL resume or return the original logical outcome
  rather than submit a second mutation

#### Scenario: cancellation reports observed truth

- **WHEN** cancellation arrives after an adapter has submitted irreversible
  work
- **THEN** the adapter SHALL return its actual terminal or continuing state and
  SHALL NOT fabricate a cancelled rollback

### Requirement: Bounded public registration data

Public registration data SHALL be bounded and secret-free. Open option, update,
action, response, registration-metadata, and document-metadata maps SHALL be public-only and bounded to 64 properties per
map, 128 collection items, 32 levels, 4,096 nodes, and 64 KiB strings. Raw
public JSON entry points SHALL be at most 512 KiB and SHALL reject
private-material-shaped members recursively.

#### Scenario: native and JSON public data share one boundary

- **WHEN** excessive, malformed, reserved, control-bearing, or secret-shaped
  public data is supplied natively or through JSON
- **THEN** both entry points SHALL reject it with equivalent redacted reason

### Requirement: Object-safe registrar and exact method dispatch

The DID capability SHALL define a `Send + Sync` object-safe asynchronous
`DidRegistrar` over the validated request/result contract. A method binding MAY
add a registrar independently of its resolver and dereferencer. The immutable
registry SHALL dispatch by exact request/job method with no fallback, prefix,
allocation, mutation, or chain knowledge.

#### Scenario: independent methods share the lifecycle seam

- **WHEN** PRISM- and Midnight-shaped registrars are bound and invoked through
  `Arc<dyn DidRegistrar>`
- **THEN** each exact method SHALL receive only its original request and return
  its existing valid result without either method type entering the core

#### Scenario: missing capability is precise

- **WHEN** a method is unknown or a known method has no registrar
- **THEN** the registry SHALL return a valid terminal `methodNotSupported` or
  `featureNotSupported` result respectively

### Requirement: Runtime and cache isolation

Generic registration SHALL perform no ambient network, filesystem, clock,
randomness, persistence, signing, cache, global-registry, chain, or executor
operation. Successful update/deactivate results SHALL expose their exact DID so
an outer coordinator MAY invalidate all resolution cache variants explicitly.

#### Scenario: performance remains observable

- **WHEN** representative exact registry dispatch is repeatedly exercised in
  release mode
- **THEN** throughput SHALL be recorded without a machine-specific CI threshold
