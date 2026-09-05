# Add bounded JWS compact

## Why

Oxid and Lace ID Portal independently implement the same JWS Compact wire
mechanics before applying different OID4VC proof policy. SDK-Rust needs one
small, fail-closed representation that bounds allocations, preserves the exact
signing input and makes its unverified state impossible to miss. Without that
foundation, proof-JWT, SD-JWT and OpenID protocol slices would each repeat a
security-sensitive parser.

## What changes

- Add an experimental `identus-jose` crate in the credential-semantics ring.
- Add strict protected-header values for `alg`, optional `typ` and optional
  `kid` only.
- Add configurable positive limits and conservative SDK defaults.
- Add a canonical JWS Compact encoder and a parser that retains the original
  compact representation and exact signing input.
- Reject ambiguous structure, non-canonical base64url, duplicate or unknown
  header members, `alg: none`, invalid header values, empty signatures and all
  size-limit violations before values cross the public boundary.
- Add RFC and independently reconstructed consumer-shaped conformance cases,
  bounded deterministic round-trip coverage and a release performance
  diagnostic.

## Non-goals

This change does not sign or verify signatures; interpret JWT claims; resolve
or authorize DID keys; select an algorithm allowlist; implement proof-JWT,
time, audience, issuer or nonce policy; support embedded JWKs, X.509 chains,
critical extensions, unencoded/detached payloads, JWS JSON Serialization or
JWE; expose raw private keys; modify consumers; publish a crate; or claim
downstream adoption.

## Impact

- **Issue:** #95, child of #8 and #20 / `IDR-004`.
- **Owner:** new `identus-jose` package in credential semantics.
- **Compatibility:** additive, experimental and unreleased API; JWS Compact is
  an external wire format but the crate defines no stored representation.
- **Dependencies:** runtime `identus-core`, `base64`, `serde` and `serde_json`
  only; no crypto backend, async runtime, network, platform or donor edge.
- **Rollback:** revert this focused change before publication; consumers are
  unchanged.
