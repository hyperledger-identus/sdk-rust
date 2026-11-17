# sdk-ts-did-document Specification

## Purpose

Define the private, additive SDK-TS DID Document facade backed by the bounded
canonical Rust model. The contract covers immutable capability-oriented
TypeScript projection, lossless normalized JSON, stable redacted failures,
shared browser/Node/React evidence, CI isolation, and rollback. It excludes
DID resolution, dereferencing, registration, method effects, networking,
storage, key custody, credentials, agent orchestration, React Native/native
bindings, publication, stable support, production claims, and mdoc.

## Requirements
### Requirement: Raw documents are validated by Rust before projection

The provider SHALL pass raw JSON directly to the bounded Rust DID Document
parser before any TypeScript JSON parse. Duplicate object names, malformed
JSON, private JWK material, ambiguous verification material, duplicate sets or
identifiers, and resource-policy violations SHALL be rejected by Rust with a
redacted stable error.

#### Scenario: duplicate input names are supplied

- **WHEN** raw document JSON repeats a decoded property name at any core or
  extension boundary
- **THEN** parsing returns `did.invalid_document`, exposes no caller-controlled
  name or value, and never constructs a TypeScript document

### Requirement: The public document is immutable and capability-oriented

The package SHALL expose strict frozen plain values for DID Documents,
contexts, controllers, aliases, verification methods, tagged relationships,
services, endpoints, and bounded extensions. Historical SDK component names,
generated WASM classes, memory, initializer, lifecycle, and raw error types
SHALL NOT appear in reviewed public exports or declarations.

#### Scenario: a full document crosses the boundary

- **WHEN** Rust accepts a document containing every core relationship, public
  verification material, services, and extensions
- **THEN** the caller receives a deeply immutable typed projection and exact
  normalized JSON with no generated ownership surface

### Requirement: Wire distinctions and extensions are preserved

The facade SHALL preserve absent versus present values, scalar versus array
cardinality, relationship reference versus embedded form, URI/map/mixed service
endpoint form, and bounded unknown extension values. Convenience projection
SHALL NOT claim original whitespace/order or RFC 8785 canonicalization.

#### Scenario: equivalent cardinality forms are parsed

- **WHEN** two valid documents use scalar and array forms respectively
- **THEN** each normalized JSON and typed value retains its selected wire form
  while both remain valid under the same Rust semantics

### Requirement: Unsafe JavaScript numbers never become lossy values

The adapter SHALL NOT silently round accepted JSON integers outside the
JavaScript safe integer range. It SHALL retain lossless normalized JSON and
return the reviewed unsupported-projection error when a typed DTO cannot be
constructed without numeric corruption.

#### Scenario: an extension contains an unsafe integer

- **WHEN** Rust accepts a document extension integer outside the JavaScript
  safe range
- **THEN** typed projection fails with the closed adapter code, no rounded value
  is returned, and diagnostics contain no document data

### Requirement: Generated ownership and errors are closed exactly once

Each generated document success or error SHALL be freed exactly once on normal
projection, validation failure, observation failure, and mapped rejection. The
public error SHALL contain only a stable code and constant message with no
caller input, Rust detail, generated object, or stack.

#### Scenario: projection fails after Rust success

- **WHEN** the generated document shape violates the reviewed binding schema
- **THEN** the raw object is freed once, a binding-contract error is returned,
  and no partially mutable value escapes

### Requirement: One document contract executes on every applicable target

One provider-neutral TypeScript assertion body SHALL execute every active
document vector unchanged through Node ESM, Node CommonJS, browser ESM, React
SSR, and hydrated React client providers. React SHALL reuse target providers
and SHALL NOT implement document semantics.

#### Scenario: SSR output hydrates in the browser

- **WHEN** React SSR parses a representative document through Node and the
  client hydrates through the browser provider
- **THEN** both phases observe the same normalized document value and the
  client-only hydration signal proves browser execution

### Requirement: Donor tests and deviations are fully ledgered

Every discovered donor document journey SHALL have an immutable identity and
one of `shared`, `adapted`, `platform-owned`, `superseded`, or `excluded`, with
rationale, replacement evidence, owner, and reconsideration trigger. Donor
defects SHALL NOT become portable compatibility requirements.

#### Scenario: a donor method-specific resolver test is encountered

- **WHEN** the inventory classifies a PRISM, peer DID, DIDComm, custody, or
  external-agent journey outside generic document semantics
- **THEN** it records the owning capability and trigger instead of silently
  skipping or copying the behavior

### Requirement: Document quality evidence is source-bound and local-first

The train SHALL provide immutable vectors, deterministic artifacts, strict
types, public-operation coverage, negative/resource tests, fresh document fuzz
and differential evidence, and measurement-only resource results bound to the
exact implementation source. Historical syntax evidence SHALL NOT be relabeled
as document evidence.

#### Scenario: local document evidence completes

- **WHEN** every focused and full local gate passes
- **THEN** the TS1 document row records only local partial target evidence and
  retains external adoption, accepted budgets, support, and publication gaps

### Requirement: CI isolation and rollback remain fail-closed

Rust core SHALL complete independently before fail-closed impact planning and
isolated target jobs. Document paths SHALL select every applicable target.
Rollback SHALL select the alternate provider before initialization; Rust or
projection failure SHALL NOT invoke a second parser.

#### Scenario: one document target fails

- **WHEN** Rust core succeeds and a selected document target fails, times out,
  is cancelled, or emits a stale receipt
- **THEN** core evidence remains green and visible while the terminal `fast`
  verdict fails

### Requirement: Runtime and product expansion remains excluded

The document package SHALL remain private, unpublished, opt-in, and
non-default. It SHALL NOT claim resolution, dereferencing, registration,
method/network effects, React Native/native evidence, external SDK-TS adoption,
stable API, support, certification, production use, or mdoc work.

#### Scenario: all repository-local document targets pass

- **WHEN** browser, Node.js, and React Web evidence is complete
- **THEN** the capability remains partial until separately authorized consumer,
  support, budget, publication, and native-target decisions are accepted
