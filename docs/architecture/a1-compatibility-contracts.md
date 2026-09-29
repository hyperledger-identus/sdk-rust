# A1 compatibility contract blueprint

**Status:** implementation blueprint for issue #504; schemas are not yet
implemented

**Architecture:** ADR
[0173](../adr/0173-sequence-a1-through-versioned-evidence-contracts.md)

## Shared rules

All records are UTF-8, repository-owned, offline-validatable, explicitly
versioned, and deterministic under sorted serialization. Stable IDs use
lowercase dot-separated capability prefixes and kebab-case record names. A
semantic schema change increments its schema version. Unknown fields fail
closed until the owning schema explicitly allows them.

Every repository source uses a full 40-character Git revision and relative
path. Every standards source uses an exact publication/profile version and
section. Payloads use SHA-256 and have an authorship, license, and
redistribution decision. No record may contain secrets, personal data,
production credentials, or inaccessible private fixtures.

## Contract 1 — cross-language vector catalog (#420)

Planned canonical path:
`docs/conformance/cross-language-vector-catalog.toml`.
Redistributable payloads live below
`docs/conformance/fixtures/<capability>/<packet-version>/`.

Required catalog fields:

| Group | Required fields |
| --- | --- |
| identity | `schema_version`, `catalog_version`, `status`, `owner_issue` |
| vector identity | stable `id`, `packet`, `capability`, `authority` |
| provenance | source kind, repository/standard, revision/version, path/section, authorship, license, redistribution |
| immutable behavior | input/payload path and SHA-256, operation, expected output or stable error code, profile scope |
| constraints | public-data declaration, resource/boundary class, target applicability, known limitations |
| evidence | exact Rust selector, optional language selectors, owning issue, supersession/replacement |

Authority values are `normative`, `identus-contract`, `consumer-regression`,
`implementation-regression`, and `exploratory`. Only the first three constrain
compatibility, and a consumer regression cannot override the first two.

Planned validator: `scripts/check-cross-language-vectors.py`. It rejects
duplicate IDs, invalid hashes/revisions, missing redistribution decisions,
unavailable local payloads, unclassified authority, selectorless active
vectors, and invalid supersession chains. Mutation tests exercise each field
and dangling external-record references.

## Contract 2 — canonical adapter/error mappings (#505)

Planned canonical path:
`docs/architecture/language-adapter-mappings.toml`.

Required mapping fields:

| Group | Required fields |
| --- | --- |
| identity | `schema_version`, stable `id`, `capability`, `owner_issue`, state |
| canonical side | Rust crate/module/type or error code, semantic version origin, canonical field/outcome description |
| language side | language/package/symbol, exact source revision, legacy/current shape, mapping direction |
| compatibility | additive/fixed/behavioral/deprecated/breaking class, lossless/lossy flag, version window, deprecation phase |
| proof | vector IDs, exact Rust and language selectors, unsupported cases |
| lifecycle | migration action, replacement, observability, rollback, removal gate |

Rust is always the canonical side. A mapping may be bidirectional, but a
legacy language shape cannot cause a donor-only field or error string to enter
the Rust core. Lossy mappings require explicit unsupported/failure behavior.

Planned validator: `scripts/check-language-adapter-mappings.py`. The first
records map DID/DID URL values and `did.invalid_did`/
`did.invalid_did_url`; they do not implement JavaScript.

## Contract 3 — consumer-visible change ledger (#422)

Upgrade existing
`docs/architecture/identus-platform-change-ledger.toml` to schema version 2.
Preserve its current empty state: governance implementation itself is not a
consumer-visible runtime change.

Required change fields add stable `id`, affected mapping/vector/quality IDs,
source and target packages, old/new behavior, migration action and window,
first/last compatible versions, release-note class, deprecation/legacy-bug
policy, rollback, exact tests/artifacts, issue and PR.

Planned validator: `scripts/check-platform-change-ledger.py`. It validates
cross-record references and produces deterministic Markdown input for release
notes/migration guides. #422 begins after #420 and #505 stable IDs land.

## Contract 4 — risk-routed quality evidence (#501)

Planned canonical path:
`docs/architecture/quality-evidence-declarations.toml`.

Each capability/slice declares all four classes:

| Class | Minimum exact evidence |
| --- | --- |
| property | invariant, generator/domain, selector, case/seed policy, target |
| fuzz | harness, corpus/seed, bounds, sanitizer/target route, retained artifact |
| benchmark | operation, fixture, environment/tool identity, statistic, sample minimum, threshold or measurement-only rationale |
| differential | named oracle/consumer revision, vector IDs, normalization, selector, accepted deviations |

Each class has outcome `required`, `satisfied`, or `not-applicable`.
`not-applicable` requires a risk-based rationale and reviewer/issue. A generic
CI URL without selectors and artifact identity is invalid.

Planned validator: `scripts/check-quality-evidence-declarations.py`. The DID
seed declaration references #420; design may proceed in parallel but #501
cannot close before that ID exists.

## Combined integrity and delivery

Issue #504 adds one combined check after all four validators exist. It verifies
that every cross-reference resolves, each active mapping/change has its
required vectors and quality declaration, authority precedence is respected,
and the four schemas agree on capability and packet versions.

The combined check joins existing factory gates but does not replace domain
validators such as Apollo parity, OID4VC conformance, error goldens, or release
receipts. Those may be referenced through adapters or migrated deliberately.

## First implementation PR sequence

1. #420: schema, validator, mutations, DID packet and Rust loader/test.
2. #505: mapping schema, validator, mutations and DID value/error records.
3. #501: quality schema, validator, mutations and DID declaration.
4. #422: ledger v2, validator/renderer and cross-reference mutations.
5. #504: combined integrity receipt and milestone closeout.
6. Stop. #492 is the separate SDK-TS consumer PR.

Each PR updates its OpenSpec task, posts factory metrics to its issue/PR, and
may merge independently when its declared blockers are closed and protected CI
is green.
