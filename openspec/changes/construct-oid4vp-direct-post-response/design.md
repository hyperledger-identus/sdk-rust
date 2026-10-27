# Design

## Consuming transition

```text
VerifiedRequestObject
        |
        v
ValidatedAuthorizationRequest
  - signed client/JAR lineage
  - HTTPS response_uri
  - optional state
  - validated DCQL response rules
        |
        | + caller-selected bounded presentations
        v
DirectPostResponse
  - target URI
  - application/x-www-form-urlencoded
  - zeroizing body
```

Construction consumes the validated request so the same evidence cannot be
silently rebound to several response destinations or incompatible result maps.
The returned value is an inert envelope, not permission or capability to send.

## Retained response rules

DCQL validation records an ordered private SDK-owned query rule containing the
identifier and `multiple` flag. It also retains the bounded credential-set
matrix: each set's `required` flag and every option's query identifiers. When
no credential sets exist, all query rules must be answered. Otherwise, every
required set must have at least one option for which all referenced queries are
answered; optional sets impose no coverage requirement. Results for any known
query remain allowed, including additional satisfied optional sets. This
metadata is derived during the existing bounded scan, not reparsed from raw
request bytes and not obtained from caller input. It remains private except for
narrow validation helpers.

## Presentation and result values

`DirectPostPresentation` owns either a bounded string presentation or a
bounded encoded JSON object. Constructors validate type and JSON work before
retention. `DirectPostQueryResult` owns one bounded query identifier and a
non-empty bounded presentation list. Diagnostics reveal only kind, count, and
byte length.

The constructor first validates result count, identifiers, duplicates,
credential-set satisfaction, and cardinality. It then serializes a
deterministic `vp_token` object in validated DCQL query order. String values
receive correct JSON escaping; validated object bytes are whitespace-minified
without normalizing numeric or string lexemes and embedded only after bounded
streaming validation. Temporary owned strings and retained encoded buffers are
wrapped in zeroizing owners.

## State and form output

Authorization routing retains an optional bounded OAuth state after verifying
a non-empty visible-ASCII value. Existing limit construction remains source
compatible; a dedicated state limit has a conservative default and an explicit
override builder.

The response encodes `vp_token=<percent-encoded-json>` followed by
`&state=<percent-encoded-state>` when present. The encoder implements the
HTML-form/RFC behavior required by the protocol, including `+` for space and
uppercase percent escapes. Output is deterministic and checked incrementally
against a final body limit before growth.

## Errors and verification

New errors distinguish invalid limits, unknown/duplicate/missing query result,
empty or excessive presentation list, cardinality mismatch, invalid or
oversized presentation, response JSON overflow, and form-body overflow. Error
messages are static.

Official examples and clean-room cases cover string/object presentations,
state, absent-set all-query coverage, required alternative sets, optional sets,
and multiple cardinality. Negative/boundary tests,
round-trip form decoding, redaction checks, property tests, fuzz targets,
benchmarks, public-contract fixtures, target builds and dependency checks bind
the implementation. No runtime network test belongs to this slice.

## Slice-size decomposition note

The delivery crosses the 12-path attention threshold because one externally
visible transition must remain bound to its request state, DCQL rules, error
contract, resource inventory, executable vector, property/fuzz/benchmark
evidence and generated governance views. Production behavior remains confined
to `identus-oid4vp`; the other paths are tests, corpus, harnesses or required
knowledge records. Splitting those records into later PRs would temporarily
advertise an ungoverned public surface or leave its E2E lineage non-executable.
HTTP execution, presentation production and verification, protected response
modes and consumer adoption remain independently decomposed successors.
