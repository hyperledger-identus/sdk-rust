# Design

## Versioned clean-room fixture packet

Create `crates/oid4vci/tests/fixtures/interop-v1/` with one manifest and small
positive/negative subdirectories. Fixture values are authored for this SDK
from the Final contract and use visibly synthetic domains and secret-shaped
values. They are not textual transformations of Oxid or Portal files.

The positive packet exercises a coherent Pre-Authorized Code wallet journey:
by-value offer transport, semantic/grant validation, issuer/server metadata
matching, Transaction Code ownership, deterministic Token Request, request-
bound Token Response, JWT Credential Request construction and immediate
Credential Response binding. Runtime test signing uses the existing public
JOSE test seam and a fixed synthetic signature.

The negative packet exercises three relevant historical incompatibility
classes: extra offer query data, null Transaction Code and a singular
Credential Response envelope. Issuer-side singular proof and JSON Token
Request vectors are not retained because no wallet-core public API consumes
them.

## Closed provenance manifest

The manifest is an object with a fixed schema/version, suite identity,
authorship/license statement, normative source, reference-only consumer
evidence and a non-empty vector list. Each vector records a stable ID, path,
SHA-256, Final section, transformation, expected result and public API.

The integration test rejects unknown top-level/vector fields, duplicate IDs or
paths, absolute/escaping/symlink/non-regular paths, non-lowercase digests,
missing provenance fields, consumer-copy claims and digest drift. It validates
the manifest before using any vector.

## Public-API execution

Tests load fixture bytes only after provenance validation. Positive inputs are
processed through public `identus-oid4vci` and `identus-jose` APIs; assertions
cover exact request media/method/body, selected issuer/server/configuration
lineage, response branch and immediate credential count/value. Negative inputs
assert stable public error variants rather than private parser details.

No test reaches into crate-private functions, starts a network listener,
imports a consumer crate or selects a chain credential format.

## Consumer compatibility review

Add a durable Markdown assessment under `docs/conformance` with a table for
each pinned generic behavior:

- Oxid strict Final controls: expressed by positive vectors;
- Oxid rejection of the legacy Portal offer/response classes: expressed by
  negative vectors;
- Portal Final-shaped issuer outputs: structurally consumable where they are
  wallet-side and generic;
- issuer-side proof validation, replay/origin policy and Midnight format
  semantics: downstream or outside the wallet-core boundary.

The assessment distinguishes immutable source/design evidence from current
human approval and live E2E interoperability.

## Program closeout

After all evidence passes, change the matrix's cross-consumer row from
`missing` to `implemented`, update the report to zero missing rows, mark
IDR-023 delivered with #376 as its immutable evidence owner, and record this
as the fortieth bounded delivery. Partial and unsupported rows remain intact.
Issue #7/M4 may then close as a bounded wallet-core milestone without implying
publication, certification or downstream adoption.

## Delivery decomposition

The reviewed implementation spans 27 paths and 1,330 changed text lines
(1,303 additions and 27 deletions), so the repository's decomposition trigger
requires an explicit cohesion decision. The bulk is test evidence: a 507-line
strict public-API integration test, a 130-line closed manifest, ten deliberately
small fixture payloads, and the planning/conformance records required to make
the evidence independently auditable. No runtime Rust source, Cargo manifest,
lockfile, dependency, public API or workflow changes.

Splitting the manifest from its validating/executing test would create an
unverified evidence window. Splitting the conformance matrix, backlog and
assessment would make the milestone claim disagree with the executable suite.
Those parts therefore land atomically. Consumer adoption, live-network
interoperability and format-specific behavior remain independent future slices
and are deliberately excluded.

## Rejected alternatives

Copying licensed Oxid wrappers would preserve ambiguous Portal-derived bytes.
Adding a generic runtime fixture engine would expand a test-only concern into
public API. A Markdown-only compatibility claim would not catch drift. Running
consumer repositories would cross repository boundaries and still would not
establish a reusable SDK fixture license.
