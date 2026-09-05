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
- **THEN** the inventory identifies #71, #73, #75, #77 and #87 as delivered
  experimental slices without implying concrete verification or trust support

#### Scenario: Consumer inspects current presentation foundations

- **WHEN** a consumer considers integrating `identus-presentations`
- **THEN** the inventory identifies #79, #81, #83 and #85 as delivered
  experimental slices without implying protocol wire/transport, selection,
  consent or authorization policy, proof execution/verification, verifier
  acceptance, receipt service or storage support

### Requirement: Credential verification activation is inventoried

The human-readable bootstrap inventory SHALL identify the canonical staged
verification evidence added under issue #73 and the async execution port plus
bounded exact-format registry added under issue #87 as parts of the
experimental `identus-credentials` surface. The verification surface SHALL
continue to exclude trust decisions and concrete parsing, proof, DID, status,
schema, clock, network, retry, storage, format, protocol and product behavior.

#### Scenario: Consumer inspects verification support

- **WHEN** a consumer inspects `identus-credentials` in the bootstrap inventory
- **THEN** the entry identifies #73 and #87, distinguishes generic dispatch
  from concrete verification, and does not imply a relying-party trust decision
