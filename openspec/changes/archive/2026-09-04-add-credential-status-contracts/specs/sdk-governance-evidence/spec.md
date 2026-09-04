# SDK governance evidence delta specification

## ADDED Requirements

### Requirement: Credential status activation is inventoried

The human-readable bootstrap inventory SHALL identify the bounded open status
binding, freshness, query, and evidence contracts added under issue #77 as part
of the experimental `identus-credentials` surface. The status slice SHALL
continue to exclude wire codecs, status proof payloads, clocks, registries,
reader/writer/verifier ports, format adapters, retrieval/verification behavior,
and trust or credential-usability decisions.

#### Scenario: Consumer inspects credential status support

- **WHEN** a consumer inspects `identus-credentials` in the bootstrap inventory
- **THEN** the entry identifies #77 without implying that the SDK retrieves,
  verifies, interprets, or acts on credential status

## MODIFIED Requirements

### Requirement: Implemented API inventory remains honest

The human-readable inventory SHALL identify the current public API families,
feature surfaces and stabilization issue for every `implemented` package. It
SHALL identify `identus-conformance` as verification-only and every placeholder
as unsupported. It SHALL label all implemented surfaces experimental,
unreleased and subject to focused component contracts. The
`identus-credentials` entry SHALL identify its bounded envelope, metadata/schema
descriptors, status binding/freshness/query/evidence contracts, and staged
verification evidence while explicitly excluding claim values, trust,
verifier/schema/status execution, holder bindings, status proof payloads,
concrete formats, protocols, display/localization, wire codecs, and storage.

#### Scenario: Consumer inspects current foundations

- **WHEN** a consumer considers integrating core, derive, crypto, DID or the
  entropy adapter
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
