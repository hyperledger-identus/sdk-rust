# sdk-ts-prism-did-method Specification

## Purpose
TBD - created by archiving change add-sdk-ts-prism-did-method. Update Purpose after archive.
## Requirements
### Requirement: Canonical and long-form PRISM identifiers remain explicit

The generated and TypeScript APIs SHALL distinguish canonical and
self-certifying long-form PRISM identifiers. They SHALL derive and validate
both forms through `identus-prism-did` and SHALL NOT duplicate suffix or
long-form parsing in TypeScript.

#### Scenario: a valid long-form identifier is parsed

- **WHEN** a caller supplies a bounded valid long-form PRISM identifier
- **THEN** Rust returns immutable long and canonical forms whose embedded
  initial state is hash-correlated to the canonical suffix

### Requirement: Creation accepts public material only

Long-form creation SHALL accept bounded public verification keys, explicit
usages, services, and contexts. It SHALL NOT accept private keys, seeds,
mnemonics, derivation paths, entropy, signing callbacks, custody handles, or a
generic operation-kind selector.

#### Scenario: secret-shaped creation input is attempted

- **WHEN** structured input contains private material or an unsupported
  curve, usage, service, or context
- **THEN** validation fails before retention and no caller bytes appear in
  errors, observations, generated values, or logs

### Requirement: Long-form initial state resolves locally

The facade SHALL project a valid long-form identifier's embedded initial state
without invoking a host effect. It SHALL verify the suffix linkage and use the
canonical Rust DID Document projector.

#### Scenario: a long-form identifier is resolved

- **WHEN** the embedded create state is valid and within resource limits
- **THEN** the immutable active document is returned locally with no network,
  chain, persistence, cache, retry, timestamp, or fallback behavior

### Requirement: Canonical resolution uses an injected exact-history effect

Canonical resolution SHALL require an injected cancellable effect that returns
only a bounded ordered array of unpadded base64url exact signed-operation
bytes. The effect SHALL NOT return a DID Document, replacement DID, trust
decision, timestamp, ledger coordinate, or policy result.

#### Scenario: the host supplies a signed history

- **WHEN** the effect returns an ordered signed create/update/deactivate
  history for the requested canonical DID
- **THEN** Rust decodes, correlates, verifies, reduces, and projects it once,
  and TypeScript cannot substitute a document or identifier

### Requirement: Reduction is authenticated, correlated, and atomic

Rust SHALL reject mismatched create operations, repeated creates, broken
previous-operation hashes, invalid signatures or authorization, invalid action
application, post-deactivation operations, and resource excess. Failure SHALL
return no partial state or document.

#### Scenario: a history operation is not authorized

- **WHEN** an update signature or authorizing key is invalid
- **THEN** the complete reduction fails with a stable operation/state category
  and no earlier partial result escapes

### Requirement: TypeScript values are strict, immutable, and reusable

The package SHALL expose frozen capability-named plain values that reuse the
existing `DidValue` and `DidDocumentValue` projections. Generated WASM owners,
memory, raw pointers, historical SDK-TS component names, and explicit public
`any` SHALL NOT cross the package boundary.

#### Scenario: generated projection succeeds

- **WHEN** a validated generated result is returned
- **THEN** TypeScript copies it into reviewed immutable values and frees every
  generated owner exactly once before returning

### Requirement: Failures are stable and redact caller material

The adapter SHALL preserve closed syntax, wire, operation, and state error
categories plus host-unavailable, host-failure, abort, observer, and
binding-contract categories. Messages and observations SHALL NOT contain DIDs,
operations, keys, services, documents, generated errors, or stacks.

#### Scenario: generated API identity drifts

- **WHEN** a generated result or error has an unknown API version, field,
  shape, form, state, or code
- **THEN** projection fails closed as a binding-contract error after exact
  ownership cleanup

### Requirement: Host effects are finite and cancellation is terminal

TypeScript SHALL bound history count, per-operation encoded bytes, aggregate
bytes, depth, nodes, strings, cycles, prototype keys, unknown fields, and
unsafe numbers before serialization. Cancellation SHALL be checked before and
after the host effect and SHALL NOT trigger retry or fallback.

#### Scenario: cancellation occurs during host resolution

- **WHEN** the abort signal becomes cancelled before the history is reduced
- **THEN** the operation returns the stable abort category, performs no Rust
  reduction, and retains no host result

### Requirement: Donor journeys have complete immutable dispositions

Every discovered pinned SDK-TS PRISM production/test journey SHALL have exact
source identity and one adapted, platform-owned, conformance-only,
superseded, or excluded disposition. NeoPRISM evidence SHALL remain read-only
and SHALL NOT transfer Cardano, persistence, indexing, or service composition.

#### Scenario: a donor test includes key custody and resolution

- **WHEN** a source journey mixes public PRISM outcomes with key generation,
  signing, storage, networking, or agent policy
- **THEN** only the portable method outcome is adapted and the remainder is
  assigned to its independent owner

### Requirement: One conformance body executes on applicable targets

One provider-neutral assertion body SHALL execute applicable PRISM vectors
unchanged through Node ESM, Node CommonJS, browser ESM, React SSR, and hydrated
React client providers. React SHALL contain no PRISM semantics and React Native
SHALL remain explicitly inapplicable.

#### Scenario: SSR output hydrates in the browser

- **WHEN** SSR uses the Node provider and the client hydrates through the
  browser provider
- **THEN** both observe identical normalized PRISM outcomes and a client-only
  signal proves real browser execution

### Requirement: PRISM adapter evidence is source-bound and local-first

The train SHALL provide immutable vectors, mappings, strict declarations,
deterministic artifacts, negative/resource/differential/property/fuzz/mutation
evidence, target receipts, and measurement-only performance results bound to
exact source. Core PRISM evidence SHALL be referenced rather than relabeled as
TypeScript adapter evidence.

#### Scenario: local evidence completes

- **WHEN** every applicable focused and full local gate passes
- **THEN** the compatibility table records only the exact proved private
  Browser, Node.js, and React Web state and retains external adoption, native,
  support, publication, release, and mdoc gaps

### Requirement: CI isolation and rollback remain fail-closed

Rust core SHALL complete before fail-closed impact planning and isolated
package, browser, Node, and React jobs. Relevant PRISM/core/binding/vector/
mapping/package paths SHALL select all applicable targets. Rollback SHALL
select a provider without PRISM capability before initialization.

#### Scenario: one selected target fails

- **WHEN** Rust core succeeds but a selected target fails, times out, is
  cancelled, skipped, stale, or missing
- **THEN** sibling evidence remains independently visible while the terminal
  verdict fails and no support claim is made

### Requirement: Runtime and product expansion remains excluded

The package SHALL remain private, unpublished, opt-in, and non-default. It
SHALL NOT claim Cardano/VDR access, indexing, persistence, private-key custody,
signing, agent runtime, React Native/native evidence, external SDK-TS adoption,
stable support, certification, production use, release, or mdoc work.

#### Scenario: all repository-local targets pass

- **WHEN** Browser, Node.js, and React Web evidence is complete
- **THEN** the capability remains partial until separately authorized
  consumer, native, accepted-budget, support, publication, and release
  decisions are accepted

