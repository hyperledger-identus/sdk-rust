# SDK governance evidence delta specification

## ADDED Requirements

### Requirement: Generated presentation support is inventoried

The human-readable and machine-readable bootstrap inventories SHALL continue
to classify `identus-presentations` as an implemented experimental
credential-semantics package. They SHALL identify bounded request, candidate,
disclosure-plan, generated-artifact and receipt-input contracts delivered by
issues #79, #81 and #83 while explicitly excluding proof execution, protocol
wire/transport, receipt outcome/persistence/policy, lifecycle state, FFI,
chain and product behavior.

#### Scenario: Consumer inspects generated presentation support

- **WHEN** a consumer inspects `identus-presentations` in the bootstrap
  inventory
- **THEN** the entry identifies the validated opaque artifact and value-free
  receipt-input boundary without implying a complete presentation protocol,
  proof implementation or wallet receipt service
