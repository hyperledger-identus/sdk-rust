## Why

Backlog item `IDR-005` requires a single chain-neutral DID resolution result
boundary after the DID syntax and document model delivered by issues #36 and
#37. NeoPRISM, midnight-identity, Lace and Oxid currently expose overlapping
but incompatible metadata, error and result records, which forces every
consumer to translate the same W3C contract independently.

GitHub issue #39 defines this bounded child of parent #5. It follows the W3C
DID Resolution v1 Candidate Recommendation Draft dated 28 August 2026, whose
error wire format is now RFC 9457-shaped. Resolution/dereferencing algorithms,
ports and HTTP bindings remain IDR-006 work under issue #10.

## What Changes

- Add immutable resolution and dereferencing result envelopes to `identus-did`.
- Add bounded media-type, resolution-datetime and opaque version-id values.
- Add bounded RFC 9457-style resolution errors with URI-valued types, all nine
  standard W3C classifications and explicit legacy-keyword migration.
- Type the common DID document metadata and preserve method extensions.
- Preserve at-risk dereferenced JSON content through a bounded open value with
  typed DID document, verification method, service and URI projections.
- Enforce success, failure and deactivation states plus requested/document DID
  equality and same-method canonical/equivalent identifiers.
- Reuse the existing document JSON resource policy for all open metadata.
- Pin current-standard and downstream-shaped tests plus a release diagnostic.
- Record the result ownership and standards-volatility boundary in ADR 0010.

## Capabilities

### Modified Capabilities

- `did-core`: extend the DID syntax/document capability with transport-free DID
  resolution and DID URL dereferencing result values.

## Impact

- **Issue:** #39, child of #5 (`IDR-005`).
- **API:** additive types in the unpublished `identus-did` package.
- **Dependencies:** no new third-party package and no lockfile change expected.
- **Consumers:** one neutral model for PRISM and Midnight producers and
  Lace/Oxid consumers; no downstream repository is modified.
- **Sequencing:** traits, async policy, method dispatch, HTTP and caching remain
  issue #10 / `IDR-006`.
- **Rollback:** revert issue #39's pull request. No published API, persisted SDK
  storage or downstream migration is involved.
