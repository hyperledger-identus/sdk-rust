# Design

## Catalog and packet separation

`docs/conformance/cross-language-vector-catalog.toml` owns source, packet, and
vector metadata. Payloads live below
`docs/conformance/fixtures/<capability>/<packet-version>/`. The first packet is
`did.syntax.v1` at `docs/conformance/fixtures/did/v1/vectors.json`.

The TOML top level contains one schema/catalog identity plus closed arrays for
sources, packets, and vectors. A source is either an immutable standard or a
repository at a full Git revision. A packet binds its path and SHA-256,
authorship, license, redistribution, public-data state, and target set. A
vector points to exactly one packet case and records authority, provenance,
operation, expectation class, boundary/resource class, selectors, ownership,
limitations, and lifecycle.

## Portable DID seed packet

Each JSON case has a stable ID, operation (`did.parse` or `did-url.parse`), a
closed input representation, and a closed expected representation. Inputs are
either `{ "literal": "..." }` or a deterministic ASCII
prefix/repeat/suffix form. Expected success objects expose canonical string
components; expected failures expose a stable Rust error code and require
input redaction.

The seed includes:

- valid basic, colon-separated, percent-preserving DID values;
- valid DID URLs with path/query/fragment and explicit empty query/fragment;
- separately classified shared donor positive and negative cases;
- invalid scheme, method, empty segment, percent escape, Unicode, space, and
  DID-URL-delimiter cases;
- malformed DID URL escape/space cases; and
- exact and over-limit DID/DID URL byte boundaries.

## Validation

`scripts/check-cross-language-vectors.py` uses only the Python standard
library. It rejects unknown fields, invalid identifiers, duplicates, malformed
revisions/hashes, unsafe paths, missing/altered packets, non-public data,
unsupported licenses/redistribution, invalid authority, inconsistent case and
expectation metadata, selectorless active vectors, incomplete coverage, and
cyclic/dangling supersession or external references.

Mutation tests copy the records into a temporary tree and independently alter
mandatory relationships. The Rust conformance test loads the checked-in JSON,
materializes bounded inputs, parses through `identus_did::{Did, DidUrl}`, and
asserts canonical components or stable redacted failures. This dev dependency
does not enter production crate edges.

## Factory integration and evolution

The validator, mutation test, catalog, packet, and explanatory document join
the central factory contract. Validation is deterministic and offline. Schema
v1 is closed; a semantic change requires a new schema/packet version. Stable
vector IDs survive deprecation and use explicit `supersedes`/`replaced_by`
links. Existing domain catalogs remain independent until a reviewed migration.

## Stop boundary

This change stops after the metadata contract and Rust proof. It does not add
language adapter code or consume the packet in another repository. #505 maps
language shapes, while #492 later performs the first consumer canary.
