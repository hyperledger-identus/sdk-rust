# ADR 0041: separate OID4VCI Credential Offer transport from semantics

- **Status:** Accepted for experimental `develop` implementation
- **Date:** 2026-09-06
- **Decision authority:** standing roadmap and autonomous component authority
  under ADR 0004
- **Related work:** issues #7, #20, and #111; `IDR-023`

## Context

OpenID4VCI 1.0 Final starts issuer-initiated flows with a Wallet invocation
that carries an embedded Credential Offer JSON object or an HTTPS URI pointing
to one. Oxid and Lace ID Portal both implement this boundary, but product
extensions and supported flow policy have drifted. SDK-Rust needs a reusable
standards boundary without importing either implementation, fetching network
content, or prematurely designing the complete protocol engine.

The selected seed contains an `identus-openid4vc` marker. The blueprint rejects
turning that inherited name into an umbrella protocol commitment and instead
plans focused `identus-oid4vci`, `identus-oid4vp`, and `identus-siopv2`
packages. Issue #7 already accepts the OID4VCI package direction, while issue
Issue #3 retains namespace and publication authority.

## Decision

1. Add an experimental, unpublished `identus-oid4vci` package in the
   protocol-semantics ring. Keep `identus-openid4vc` quarantined and independent.
2. Make Credential Offer invocation parsing the first package capability. It
   returns distinct embedded and referenced states and performs no network I/O.
3. Treat transport validation and Credential Offer semantic validation as
   separate stages. Embedded state proves bounded, complete, duplicate-free
   JSON-object syntax only and retains the exact decoded object for a later
   semantic parser.
4. Accept only the registered `openid-credential-offer://?` form with exactly
   one literal `credential_offer` or `credential_offer_uri` parameter. Match
   URI schemes case-insensitively and parameter names case-sensitively.
5. Bound complete and decoded input, JSON depth, and aggregate nodes. Cap the
   caller-configurable container depth at 64, below the JSON parser's internal
   recursion ceiling. Reject malformed form encoding, UTF-8, duplicate JSON
   names, trailing JSON, parameter smuggling, and unsafe reference-URI syntax.
6. Require references to be absolute HTTPS URIs with a non-empty host and no
   user information or fragment. Do not claim this prevents SSRF or proves
   issuer trust; fetching and destination policy remain outside the package.
7. Treat embedded and referenced values as bearer-adjacent sensitive strings:
   zeroize owned buffers and omit content from diagnostics and serialization.
8. Derive implementation from OpenID4VCI Final and RFC 3986. Use Oxid and Lace
   only as read-only behavior evidence; copy no code or fixtures.

## Consequences

- OID4VCI gains a small safe ingress boundary before grant/metadata/state
  complexity enters the package.
- Consumers can distinguish “safe to hand to the next parser/fetch adapter”
  from “semantically valid Credential Offer.”
- Lace's legacy `issuer_origin` query parameter is deliberately rejected by
  this final-profile API; a product may translate legacy input before calling
  the SDK under its own compatibility policy.
- The new crate can compile on host, WASM, Android, and iOS without HTTP,
  async, crypto, chain, or product dependencies.
- This decision does not reserve/publish a crate, accept full OID4VCI API
  stability, or authorize downstream edits.

## Alternatives considered

- **Activate the umbrella marker.** Rejected because it would turn seed history
  into a cross-protocol namespace/API commitment contrary to the blueprint.
- **Parse the complete Credential Offer immediately.** Rejected for this slice
  because grants, extensions, metadata linkage, and flow state form a larger
  independently reviewable contract.
- **Fetch `credential_offer_uri` in the same API.** Rejected because it would
  require async/runtime choices plus redirect, DNS/IP, cache, media-type,
  privacy, and trust policy.
- **Adopt either consumer parser.** Rejected because each includes product
  policy or legacy deviations and Lace lacks repository license evidence at
  the inspected revision.

## Migration and rollback

The package is additive, experimental, and unpublished. No existing SDK or
consumer dependency is repointed. Before release, rollback is a focused removal
of the package and its architecture/evidence records. Later semantic, fetching,
protocol-state, adoption, namespace, and release decisions require separate
issues and compatibility evidence.
