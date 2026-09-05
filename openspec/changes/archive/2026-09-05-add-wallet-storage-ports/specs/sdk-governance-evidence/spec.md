## MODIFIED Requirements

### Requirement: Implemented API inventory remains honest

The human-readable inventory SHALL identify the current public API families,
feature surfaces and stabilization issue for every `implemented` package. It
SHALL identify `identus-conformance` as verification-only and every placeholder
as unsupported. It SHALL label all implemented surfaces experimental,
unreleased and subject to focused component contracts. The
`identus-credentials` entry SHALL identify its bounded envelope,
metadata/schema descriptors, status binding/freshness/query/evidence contracts,
canonical staged verification evidence, async verifier port and bounded exact
format registry while explicitly excluding claim values, trust decisions,
concrete verifier/schema/status execution, holder bindings, status proof
payloads, concrete formats, protocols, display/localization, wire codecs and
storage. The `identus-presentations` entry SHALL identify its bounded semantic
request, candidate, disclosure-plan, generated-artifact, receipt-input and
protocol lifecycle contracts while explicitly excluding protocol
wire/transport, candidate lookup/ranking, selection/consent/authorization
policy, proof execution/verification, verifier acceptance, receipt
outcome/persistence/policy, storage, FFI, chain and product behavior. The
`identus-wallet` entry SHALL identify bounded shared storage vocabulary plus
the five associated-type storage ports under #89 while explicitly excluding a
wallet product, SDK-owned records, concrete storage, encryption, codecs,
transactions, synchronization, custody, policy, protocols, chains and FFI.

#### Scenario: Consumer inspects current foundations

- **WHEN** a consumer considers integrating core, derive, crypto, DID,
  credential/presentation semantics, wallet storage ports or the entropy
  adapter
- **THEN** the inventory identifies the real surface and owner issue while
  warning that no immutable release or compatibility commitment exists

#### Scenario: Placeholder name resembles a planned component

- **WHEN** a placeholder name overlaps a roadmap concept
- **THEN** the inventory states that the name and dependency graph remain
  unaccepted until a focused issue replaces the placeholder

#### Scenario: Consumer inspects current credential foundations

- **WHEN** a consumer considers integrating `identus-credentials`
- **THEN** the inventory identifies #71, #73, #75, #77 and #87 as delivered
  experimental slices without implying concrete verification or trust support

#### Scenario: Consumer inspects current presentation foundations

- **WHEN** a consumer considers integrating `identus-presentations`
- **THEN** the inventory identifies #79, #81, #83 and #85 as delivered
  experimental slices without implying protocol wire/transport, selection,
  consent or authorization policy, proof execution/verification, verifier
  acceptance, receipt service or storage support

#### Scenario: Consumer inspects wallet storage foundations

- **WHEN** a consumer considers integrating `identus-wallet`
- **THEN** the inventory identifies #89 as an experimental port contract and
  does not imply a production adapter, storage format, wallet or custody

## ADDED Requirements

### Requirement: Wallet storage activation is inventoried

The human and machine inventories SHALL reclassify `identus-wallet` from a
quarantined placeholder to implemented experimental orchestration only when
the issue #89 port surface, bounds, redaction and tests exist. The inventory
SHALL record runtime `identus-core` plus build-time `identus-derive` as the
complete dependency cone and SHALL continue to deny publication.

#### Scenario: Inventory validates the activated package

- **WHEN** the bootstrap inventory validator inspects `identus-wallet`
- **THEN** its source shape, classification, public families, issue ownership
  and dependencies agree with the implemented storage-port contract
