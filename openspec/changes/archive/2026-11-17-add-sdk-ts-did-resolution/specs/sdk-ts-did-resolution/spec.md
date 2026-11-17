## ADDED Requirements

### Requirement: Rust normalizes every request before host invocation

The provider SHALL pass the DID or DID URL and serialized option map through
the bounded Rust validators before invoking a host effect. The effect SHALL
receive a frozen plain request containing only Rust-normalized values. Invalid
identifiers, options, reserved extensions, and resource-policy violations SHALL
prevent invocation.
The public API SHALL accept structured options only. It SHALL preserve absence
versus explicit false, including `noCache`, and SHALL reject cycles, prototype
keys, unsafe numbers, and structural budgets before closed serialization.

#### Scenario: invalid options are supplied

- **WHEN** a request contains an invalid media type, version time,
  verification relationship, reserved extension, or over-budget value
- **THEN** Rust returns the stable invalid-options failure, the host is not
  invoked, generated ownership is closed once, and no caller data is logged

### Requirement: Host effects are explicit, asynchronous, and policy-owned

The package SHALL invoke at most one configured async `resolve` or
`dereference` host capability per operation. The host SHALL return raw UTF-8
result JSON text. Transport, endpoint, fetch/HTTP, headers, authentication,
outer allocation, decompression, timeout, cancellation, retry, cache, SSRF, and
network-observation policy SHALL remain caller-owned.
An already-aborted supplied signal SHALL reject before invocation; after
invocation cancellation is advisory to the host and the facade SHALL NOT claim
to stop an effect that ignores the signal.

#### Scenario: a host adapter uses fetch

- **WHEN** the adapter receives a response
- **THEN** it applies its caller-owned outer policy and returns `response.text()`
  rather than a parsed object, preserving duplicate names and numeric bytes for
  canonical Rust validation

### Requirement: Resolution results are bounded and correlated by Rust

Raw resolution result text SHALL be validated by
`DidResolutionResult::from_json_str_for` before TypeScript parses it. Rust SHALL
enforce raw-wire budgets, typed branch validity, document identity, canonical
and equivalent DID method constraints, metadata semantics, and stable redacted
rejection.

#### Scenario: a host returns a different document

- **WHEN** a success result document ID differs from the requested DID
- **THEN** Rust rejects the envelope as an invalid resolution result, no typed
  result escapes, and neither the raw DID nor document appears in diagnostics

### Requirement: Dereferencing retains its independent result contract

Raw dereferencing result text SHALL pass the bounded current
`DidUrlDereferencingResult` validator before projection. The facade SHALL
preserve success/error branch distinctions, operation metadata, content,
content metadata, typed known projections, and bounded open JSON without
claiming method, authorization, representation, or transport semantics.

#### Scenario: a contradictory result is returned

- **WHEN** dereferenced content and error metadata coexist or both are absent
- **THEN** Rust rejects the envelope with the stable invalid-dereferencing-
  result failure and TypeScript constructs no partial value

### Requirement: Public results are immutable and capability-oriented

The package SHALL expose strict deeply frozen plain options, requests, results,
metadata, RFC 9457 problem details, document/content values, and bounded JSON.
Historical component names, generated WASM types, memory, initializer,
lifecycle, raw host values, and explicit public `any` SHALL NOT appear in
reviewed exports or declarations.
W3C/RFC 9457 problem data SHALL use the distinct
`DidResolutionProblemValue` name and SHALL NOT be confused with thrown local
SDK errors. Native non-JSON dereferencing content is unsupported in this train.

#### Scenario: a successful result crosses the boundary

- **WHEN** Rust accepts a representative result
- **THEN** the caller receives a frozen discriminated result with normalized
  JSON and reusable immutable DID Document projection, with no generated owner

### Requirement: Wire distinctions and numeric fidelity are preserved

The facade SHALL preserve success, operation-error, and deactivation states;
absent versus present metadata; problem-detail fields; document/content forms;
and bounded extensions. It SHALL NOT silently round integers outside the
JavaScript safe range. Normalized JSON is semantic deterministic output and
SHALL NOT claim original formatting or RFC 8785 canonicalization.

#### Scenario: an extension contains an unsafe integer

- **WHEN** Rust accepts a result extension integer that cannot be represented
  exactly by the public TypeScript JSON value
- **THEN** projection fails with the closed unsupported-lossless-projection
  code and no rounded result is returned

### Requirement: Operation errors and host failures remain distinct

A valid W3C result containing problem metadata SHALL resolve to a typed result.
A rejected host promise or invalid result envelope SHALL reject with distinct
stable SDK-owned codes. Messages and observations SHALL never contain DID/DID
URL values, endpoints, option/result bodies, problem details, host throwables,
or stacks.

#### Scenario: the host promise rejects

- **WHEN** the injected effect throws or rejects
- **THEN** the provider returns the constant host-execution failure, records
  only redacted route/result metadata, and does not retry or select a fallback

### Requirement: Generated ownership is closed before asynchronous suspension

Each generated request/result/error SHALL be copied and freed exactly once.
Request ownership SHALL close before awaiting the arbitrary host promise, and
result ownership SHALL close before public completion, projection rejection,
or observer rejection.

#### Scenario: an observer rejects after projection

- **WHEN** the configured observation callback throws
- **THEN** every generated value has already been freed once, the stable
  observation failure is returned, and no second host/provider is invoked

### Requirement: One conformance contract executes on every applicable target

One provider-neutral TypeScript assertion body SHALL execute every active
resolution/dereferencing vector unchanged through Node ESM, Node CommonJS,
browser ESM, React SSR, and hydrated React client providers. React SHALL reuse
target providers and SHALL NOT implement resolution semantics.

#### Scenario: SSR output hydrates in the browser

- **WHEN** SSR resolves through the Node provider and the client hydrates
  through the browser provider
- **THEN** both observe the same normalized typed outcome and a client-only
  signal proves browser execution without serializing host policy

### Requirement: Donor journeys and deviations are completely ledgered

Every discovered pinned SDK-TS resolver source/test journey SHALL have an
immutable identity and one of `shared`, `adapted`, `platform-owned`,
`superseded`, or `excluded`, with rationale, replacement evidence, owner, and
reconsideration trigger. Method and transport policy SHALL NOT be copied into
the generic portable contract.

#### Scenario: a PRISM or peer resolver test is encountered

- **WHEN** it mixes method construction, cryptography, network policy, or
  document mutation with resolution
- **THEN** the generic async outcome is adapted and the remaining behavior is
  assigned to its method/platform owner rather than silently skipped

### Requirement: Resolution quality evidence is source-bound and local-first

The train SHALL provide immutable vectors, strict declarations, deterministic
artifacts, property/fuzz/differential/negative/resource evidence, mutation
guards, target receipts, and measurement-only performance results bound to the
exact source. Syntax/document evidence SHALL NOT be relabeled as resolution
evidence.

#### Scenario: local evidence completes

- **WHEN** every focused and full local gate passes
- **THEN** the TS1 resolution row records only the exact proved private target
  state and retains external adoption, transport, method, budget, support,
  publication, native-target, and release gaps

### Requirement: CI isolation and rollback remain fail-closed

Rust core SHALL complete before fail-closed impact planning and isolated target
jobs. Relevant core/binding/mapping/vector/package paths SHALL select every
applicable target. A target failure SHALL preserve completed core and sibling
evidence while failing the existing terminal verdict. Rollback SHALL choose the
alternate provider before initialization; an operation SHALL NOT fall back
after Rust, host, projection, or observer failure.

#### Scenario: one resolution target fails

- **WHEN** Rust core succeeds but a selected target fails, times out, is
  cancelled, skipped, or emits a stale receipt
- **THEN** core evidence remains green and reviewable while `fast` fails and
  the resolution capability remains undelivered for that target

### Requirement: Runtime and product expansion remains excluded

The resolution package SHALL remain private, unpublished, opt-in, and
non-default. It SHALL NOT claim concrete transport/method/cache policy, React
Native/native evidence, external SDK-TS adoption, stable API, support,
certification, production use, release, or mdoc work.

#### Scenario: all repository-local targets pass

- **WHEN** browser, Node.js, and React Web evidence is complete
- **THEN** the capability remains partial until separately authorized consumer,
  method/transport, budget, support, publication, native-target, and release
  decisions are accepted
