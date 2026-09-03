## Why

`DidDocument::from_json_slice` currently hands bounded bytes directly to
`serde_json`, whose map representation cannot retain duplicate object names.
That permits ambiguous resolver input to be interpreted differently by other
JSON implementations and prevents the SDK from proving which value was signed,
validated, or displayed. The existing dependency-free RFC 3986 parser also
needs independent differential evidence before downstream method adapters rely
on it as a shared wire boundary.

## What Changes

- Reject duplicate names in every object nested in raw DID document JSON before
  typed deserialization can collapse them, including escaped-equivalent names.
- Add a reusable crate-private streaming scanner with explicit nesting, node,
  object-member, and live-key-byte limits under the existing 256 KiB envelope.
- Add one redaction-safe document error reason while preserving the stable
  public `did.invalid_document` contract.
- Add deterministic URI/document property suites and compare URI acceptance
  with NeoPRISM-aligned `uriparse` 0.6.4 as a development-only oracle.
- Record intentional parser-profile differences plus release throughput and
  resource-shape evidence without normalizing URIs or adding runtime policy.
- Keep sanitizer-backed DID/DID-URL cargo-fuzz work in #35 and resolution
  envelope adoption in #41.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `did-core`: strengthens raw DID document ingestion to reject ambiguous JSON,
  makes its scanner work bounds explicit, and requires deterministic
  differential/generative evidence for the URI and document boundary.

## Impact

- Affected code: `crates/did` JSON ingestion, document error taxonomy, and DID
  conformance tests; one private scanner module is reusable by later #41 work.
- Public compatibility: unique-name document JSON and all native APIs remain
  compatible; raw duplicate-name JSON changes from last-value-wins acceptance
  to a deterministic error before the first SDK release.
- Dependencies: production dependencies remain unchanged; exact
  `uriparse` 0.6.4 is test-only and matches the NeoPRISM etalon.
- Repository boundary: no chain, product, runtime, network, filesystem,
  JSON-LD, normalization, cryptographic, FFI, release, or downstream change.
