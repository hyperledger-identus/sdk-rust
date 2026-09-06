# Pre-implementation semantic review

## Review basis

- Issue #111 and parent #7
- OpenID4VCI 1.0 Final sections 4.1, 13.5, 15.4.2, and G.7.1
- RFC 3986 URI scheme/percent-encoding semantics
- SDK blueprint, crate-ring, support-policy, and upstream-program contracts
- immutable Oxid and Lace behavior evidence recorded in `design.md`

## Architecture and API

- PASS: the focused package follows the accepted portfolio and does not
  activate the quarantined umbrella marker.
- PASS: transport validity is a weaker explicit state than semantic offer
  validity; names and accessors do not claim issuer/grant acceptance.
- PASS: the runtime dependency cone points inward only and omits HTTP, async,
  crypto, chain, storage, and product authority.
- PASS: adding an unpublished experimental package is reversible and does not
  cross namespace/publication authority.

## Standards and compatibility

- PASS: exactly one by-value or by-reference query parameter matches section
  4.1; extensions remain inside the offer object rather than the invocation.
- PASS: scheme comparison is ASCII-case-insensitive under RFC 3986 while
  registered query parameter names remain case-sensitive.
- PASS: HTTPS reference syntax is enforced without pretending syntax proves
  destination trust or media type.
- PASS: rejecting Lace's `issuer_origin` and retaining its behavior only as a
  negative case avoids standardizing a legacy product extension.

## Security, privacy, and resources

- PASS: complete/decoded/depth/node limits cover allocation and parser abuse;
  all limits are positive and testable at exact boundaries.
- PASS: raw parameter names prevent encoded-alias smuggling; strict form
  decoding, UTF-8, duplicate-name, and trailing-input checks fail closed.
- PASS: reference fetching remains absent, so SSRF/DNS/redirect policy is not
  accidentally granted to a domain type.
- PASS: offer/reference content is bearer-adjacent, zeroized, and redacted
  from diagnostics and serialization.

## Provenance and verification

- PASS: no code or fixture is copied; exact source paths/digests and Lace's
  unresolved license posture are explicit.
- PASS: official plus independently reconstructed consumer-shaped tests,
  portable gates, a diagnostic, full Nix, and post-implementation review are
  required.

## Review correction

The initial issue wording made the registered scheme case-sensitive and
omitted `identus-core` from the declared dependency cone. Semantic review
corrected both before implementation: RFC 3986 scheme matching is
case-insensitive, and the shared component/error dependency is explicit.

## Pre-implementation finding

No unresolved blocker remains. Implementation may begin after strict OpenSpec
and factory structural validation pass.

# Post-implementation semantic, security, and API review

- **Reviewed head:** `e52d22c53ee4dab57d0c9541fe418c187173054b`
- **Review completed:** 2026-09-06T18:45:57+08:00
- **Result:** passed after three corrections; no unresolved finding

The exact diff from the recorded `develop` base was re-read after focused,
workspace, conformance, portable target, MSRV, supply-chain, and all 27 local
Nix checks. The new crate remains unpublished and independent from the
quarantined umbrella marker. Its only internal runtime dependency is
`identus-core`; no network, async, crypto, DID, storage, chain, product, or
donor authority entered the package.

The public types distinguish embedded and referenced transport without
claiming semantic offer validity or destination trust. Invocation, decoded
bytes, JSON depth, and JSON node counts have explicit deterministic ceilings.
The streaming scan rejects duplicate decoded member names and trailing or
non-object JSON without materializing a second tree. Owned bearer-adjacent
content and allocated scanner strings are zeroized, while public formatting
and core errors remain static and redacted.

Review found and resolved three issues:

1. `UnsupportedTransport` now bridges to the shared `ErrorKind::Unsupported`
   family, and every public error variant has an exhaustive code/kind/capability
   assertion.
2. A lexical authority precheck now rejects `https:///hostless` before
   `uriparse` can normalize path text into a host; empty and port-only
   authorities are also explicit negative evidence.
3. The positive standards fixtures now reproduce the exact OID4VCI 1.0 Final
   by-value and by-reference examples with presentation line breaks removed,
   rather than merely using conforming representative values.

The original-wire authority guard, strict single-pair form decoder, exact JSON
retention, HTTPS-only reference syntax, and no-network boundary now match the
OpenSpec contract. Oxid and Lace ID Portal remain unchanged at their recorded
preflight revisions and worktree states.
