# SDK governance evidence delta specification

## ADDED Requirements

### Requirement: Presentation request activation is inventoried

The machine-readable and human-readable bootstrap inventories SHALL classify
`identus-presentations` as an implemented, experimental
credential-semantics package after issue #79. They SHALL identify its bounded
request/query/claim/challenge/candidate contracts and SHALL continue to exclude
DCQL/protocol wire, lookup and selection policy, consent, proof execution,
presentation artifacts/receipts, lifecycle state, storage, FFI, and product or
chain behavior.

#### Scenario: Consumer inspects presentation support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies #79 and its request/query/candidate boundary
  without implying that a complete presentation protocol or wallet flow exists

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
The `identus-presentations` entry SHALL identify its bounded semantic
request/query/claim/challenge/candidate contracts while explicitly excluding
protocol wire, candidate lookup/ranking, consent, proof execution,
artifact/receipt, lifecycle, storage, FFI, chain, and product behavior.

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
- **THEN** the inventory identifies #79 as an experimental first slice without
  implying protocol, selection, consent, proof, lifecycle, or storage support
