# SDK governance evidence delta specification

## ADDED Requirements

### Requirement: Presentation lifecycle support is inventoried

The human-readable and machine-readable bootstrap inventories SHALL continue
to classify `identus-presentations` as an implemented experimental
credential-semantics package. They SHALL identify the bounded lifecycle phase,
terminal-outcome and transition contract delivered by issue #85 alongside
issues #79, #81 and #83, while explicitly excluding protocol wire/transport,
candidate lookup/ranking, selection/consent/authorization policy, proof
execution/verification, verifier acceptance, storage, FFI, chain and product
behavior.

#### Scenario: Consumer inspects presentation lifecycle support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies the allocation-free state vocabulary and guard
  without implying a complete protocol engine, durable session or success
  receipt

## MODIFIED Requirements

### Requirement: Implemented API inventory remains honest

The human-readable inventory SHALL identify the current public API families,
feature surfaces and stabilization issue for every `implemented` package. It
SHALL identify `identus-conformance` as verification-only and every placeholder
as unsupported. It SHALL label all implemented surfaces experimental,
unreleased and subject to focused component contracts. The
`identus-credentials` entry SHALL identify its bounded envelope,
metadata/schema descriptors, status binding/freshness/query/evidence contracts,
and staged verification evidence while explicitly excluding claim values,
trust, verifier/schema/status execution, holder bindings, status proof payloads,
concrete formats, protocols, display/localization, wire codecs, and storage.
The `identus-presentations` entry SHALL identify its bounded semantic request,
candidate, disclosure-plan, generated-artifact, receipt-input and protocol
lifecycle contracts while explicitly excluding protocol wire/transport,
candidate lookup/ranking, selection/consent/authorization policy, proof
execution/verification, verifier acceptance, receipt
outcome/persistence/policy, storage, FFI, chain and product behavior.

#### Scenario: Consumer inspects current foundations

- **WHEN** a consumer considers integrating core, derive, crypto, DID, the
  credential/presentation semantics, or the entropy adapter
- **THEN** the inventory identifies the real surface and owner issue while
  warning that no immutable release or compatibility commitment exists

#### Scenario: Placeholder name resembles a planned component

- **WHEN** a placeholder name overlaps a roadmap concept
- **THEN** the inventory states that the name and dependency graph remain
  unaccepted until a focused issue replaces the placeholder

#### Scenario: Consumer inspects current credential foundations

- **WHEN** a consumer considers integrating `identus-credentials`
- **THEN** the inventory identifies #71, #73, #75, and #77 as delivered
  experimental slices without implying the deferred capabilities are supported

#### Scenario: Consumer inspects current presentation foundations

- **WHEN** a consumer considers integrating `identus-presentations`
- **THEN** the inventory identifies #79, #81, #83 and #85 as delivered
  experimental slices without implying protocol wire/transport, selection,
  consent or authorization policy, proof execution/verification, verifier
  acceptance, receipt service or storage support

### Requirement: Generated presentation support is inventoried

The human-readable and machine-readable bootstrap inventories SHALL continue
to classify `identus-presentations` as an implemented experimental
credential-semantics package. They SHALL identify bounded request, candidate,
disclosure-plan, generated-artifact and receipt-input contracts delivered by
issues #79, #81 and #83 plus accepted later presentation slices while
explicitly excluding proof execution, protocol wire/transport, receipt
outcome/persistence/policy, FFI, chain and product behavior.

#### Scenario: Consumer inspects generated presentation support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies the validated opaque artifact and value-free
  receipt-input boundary alongside accepted later lifecycle support without
  implying a complete presentation protocol, proof implementation or wallet
  receipt service

### Requirement: Presentation request activation is inventoried

The machine-readable and human-readable bootstrap inventories SHALL classify
`identus-presentations` as an implemented, experimental credential-semantics
package and SHALL retain issue #79 as the request/query/candidate foundation.
The current entry SHALL also identify later accepted presentation slices and
SHALL continue to exclude protocol wire/transport, lookup and selection
policy, consent/authorization, proof execution/verification, receipt
outcome/persistence/policy, storage, FFI, and product or chain behavior.

#### Scenario: Consumer inspects presentation support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies #79 and its request/query/candidate foundation
  alongside accepted later slices without implying that a complete
  presentation protocol or wallet flow exists

### Requirement: Presentation disclosure-plan support is inventoried

The machine-readable and human-readable bootstrap inventories SHALL continue
to classify `identus-presentations` as an implemented, experimental
credential-semantics package. They SHALL identify the bounded
request/query/candidate and validated disclosure-plan foundations delivered by
issues #79 and #81 plus accepted later presentation slices, while explicitly
excluding protocol wire/transport, candidate lookup or ranking, selection and
consent/authorization policy, proof execution/verification, receipt
outcome/persistence/policy, storage, FFI, chain and product behavior.

#### Scenario: Consumer inspects presentation disclosure-plan support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies the structural proof-generation input without
  hiding accepted later result/lifecycle boundaries or implying that the SDK
  chooses disclosures or implements a complete presentation protocol or
  wallet flow
