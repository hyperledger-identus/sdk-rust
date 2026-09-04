# SDK governance evidence delta specification

## ADDED Requirements

### Requirement: Presentation disclosure-plan support is inventoried

The machine-readable and human-readable bootstrap inventories SHALL continue
to classify `identus-presentations` as an implemented, experimental
credential-semantics package. They SHALL identify the bounded
request/query/candidate and validated disclosure-plan contracts delivered by
issues #79 and #81 while explicitly excluding protocol wire, candidate lookup
or ranking, selection and consent policy, proof execution, presentation
artifacts or receipts, lifecycle state, storage, FFI, chain and product
behavior.

#### Scenario: Consumer inspects presentation disclosure-plan support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies the structural proof-generation input without
  implying that the SDK chooses disclosures or implements a complete
  presentation protocol or wallet flow
