# Local review

Codex CLI reviewed the complete change against `origin/develop` after Claude
CLI was unavailable. The required fallback prevented an external session limit
from blocking delivery.

## Resolved findings

- Malformed non-ASCII numalgo-2 purpose elements no longer panic.
- Numalgo-4 rejects secret-shaped and duplicate JSON members before retention;
  its `Debug` output is redacted.
- Both profiles guarantee that accepted created or parsed DIDs resolve locally;
  service URIs and post-contextualization alias bounds are validated.
- Numalgo-4 parsing enforces the decoded byte ceiling without a conservative
  base58 estimate that could reject values produced by creation.
- The fuzz target treats bounded construction rejection as an ordinary outcome.
- Profile-specific raw DID and fixed multihash ceilings reject oversized input
  before delimiter scanning or base58 decoding.
- Numalgo-2 rejects oversized retained service strings before serialization,
  and numalgo-4 rejects private JWK members before public retention, including
  explicitly JWK-named extension objects that omit `kty`.
- Numalgo-4 parsing retains the exact validated identity-defining JSON bytes,
  so accepted noncanonical inputs recreate the same long and short forms.
- Both profiles preflight aggregate DID-document projection growth before
  expanding repeated DIDs or local references and return `ResourceLimit`.
- Numalgo-2 retains the pinned compatibility profile's bounded one-or-many
  endpoint-map shape; JWK context is preserved through nested arrays.
- Numalgo-2 preflights aggregate DID length before base64-encoding and appending
  each compact service.
- Both profile parsers match the complete algorithm selector boundary, so
  selectors such as `20` and `40` remain `UnsupportedAlgorithm` rather than
  being misclassified as malformed payloads for algorithms 2 and 4.
- Numalgo-2 maps `V` keys only to authentication, accepts omitted compact
  endpoint options without inventing them on re-encoding, and supports the
  Identus one-array encoding for multiple services in one `S` element.
- Numalgo-2 applies one shared 4,096-node compact-service JSON budget during
  creation and parsing, preserving create/parse closure for every accepted
  service array; truncated elements retain the malformed-encoding category.
- The exact 16 KiB decoded input ceiling round-trips. The profile ceiling is
  intentionally below the generic DID limit because large base58 conversion is
  super-linear; the measured exact-boundary test completes in about 2.3 seconds
  in the debug profile on the local Apple Silicon host.

## Rejected finding

The review suggested fingerprint-derived numalgo-2 verification method IDs.
The pinned DIF specification at
`decentralized-identity/peer-did-method-spec@c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8`
requires ordered `#key-N` identifiers starting at one, and the pinned
`did-peer` 0.7.5 oracle implements the same rule. The design text was corrected
to state ordered-position derivation; the implementation remains unchanged.
