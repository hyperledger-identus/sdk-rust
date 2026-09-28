# Design

## Module map

`json.rs` remains the crate-private facade and owns only module declarations,
re-exports, the cursor scanner, lexical/container traversal, root-object
admission, common bounded scalar/object mechanics, and transport-level
`validate_json`.

Four descendants own protocol-specific fields and parsing:

| Module | Owned entry points and grammar |
| --- | --- |
| `json/offer.rs` | Credential Offer core, grants, authorization/pre-authorized alternatives, Transaction Code, configuration-ID arrays |
| `json/metadata.rs` | Credential Issuer Metadata, Authorization Server Metadata, authorization-server/configuration/grant-type collections |
| `json/token.rs` | Token Response, Authorization Details, Token Error, Credential Nonce, authorization-detail helper decoding |
| `json/credential.rs` | Credential Endpoint payload error, immediate/deferred responses, issued credential values |

Every moved record/function remains `pub(crate)` only where its current caller
requires it; scanner mechanics remain private to the `json` module tree.

## Shared ingress invariant

One root-object helper creates the scanner, applies the exact depth/node
limits, admits and counts the root object, invokes one protocol parser, skips
trailing whitespace, and rejects incomplete input with the caller-supplied
existing static error. `validate_json` retains its current transport behavior.
The scanner alone owns string/number/literal syntax, duplicate-name mechanics,
container traversal, and common bounded scalar parsing.

Protocol parsers continue to own required fields, collection cardinality and
uniqueness, per-field limits, exact-value retention, and error selection. The
split does not normalize superficially similar error or object rules.

## Compatibility and cleanup

Callers keep importing through `crate::json`; private re-exports preserve the
current names. Decoded untrusted strings and retained response values remain
zeroizing. Rejected recursive values continue to be traversed iteratively by
the scanner. No serde-derived intermediary or unbounded `Value` tree is added.

## Characterization and ratchet

The existing integration suite is the pre-move characterization corpus: it
covers all eleven entry families and malformed, duplicate, depth/node,
cardinality, retained-byte, numeric, redaction, and branch-exclusion cases.
It runs at the planning head before any move and at the implementation head.

Code-health evidence must show semantic ownership rather than merely smaller
files: `json.rs` becomes the scanner/ingress core, each descendant has one
protocol change axis, no production file exceeds the 1,000-line attention
threshold, and the combined call cluster does not gain duplicate scanners,
root admission, limit policy, or forwarding-only layers.

## Verification

Review compares public API/callers, Cargo metadata, error golden hashes,
function bodies, and pre/post test results. Focused OID4VCI tests and strict
Clippy precede workspace/factory/Nix gates. Fuzz/weekly slow evidence remains
the production-promotion line; no release or downstream adoption is part of
this change.
