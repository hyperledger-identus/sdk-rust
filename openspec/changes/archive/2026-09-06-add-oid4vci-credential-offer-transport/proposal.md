# Add bounded OID4VCI credential-offer transport

## Why

Oxid and Lace ID Portal independently construct or route the same
`openid-credential-offer` invocation, but their accepted query surfaces have
drifted. The first OID4VCI protocol slice should establish the standards-bound
transport before semantic offer fields, grants, metadata, networking, or
orchestration are introduced.

## What changes

- Add an experimental, unpublished `identus-oid4vci` crate in the
  protocol-semantics ring while retaining the quarantined
  `identus-openid4vc` umbrella marker.
- Parse the OID4VCI 1.0 Final by-value and by-reference Credential Offer URI
  shapes into distinct types.
- Bound invocation, decoded value, reference URI, JSON depth, and JSON node
  counts before untrusted input crosses the public boundary.
- Reject query smuggling, malformed percent encoding, invalid UTF-8,
  duplicate JSON names, trailing JSON, and unsafe reference URI forms.
- Redact diagnostics and zeroize owned offer/reference strings because either
  can carry a bearer capability.
- Add official and independently reconstructed Oxid/Portal-shaped tests with
  immutable behavior-only provenance.

## Non-goals

This change does not interpret Credential Offer fields or grants; fetch a
referenced offer; choose authorization servers; implement token, nonce,
credential, metadata, or state-machine types; define format or chain
extensions; modify consumers; publish a crate; release; or promote to `main`.

## Impact

- **Issue:** #111, child of #7 and #20 / `IDR-023`.
- **Owner:** new `identus-oid4vci` package in protocol semantics.
- **Compatibility:** additive, experimental, unpublished API and external
  standards-defined URI boundary.
- **Dependencies:** `identus-core`, `serde`, `serde_json`, `uriparse`, and
  `zeroize` only.
- **Rollback:** remove the focused crate and associated records before any
  release; downstream repositories remain unchanged.
