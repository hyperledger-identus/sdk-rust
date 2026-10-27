# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/532
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep dependencies inward and effects at the
edge. SDK-SEC-003 and SDK-LIM-007 require finite untrusted work and explicit
input/output bounds. Existing protocol, error, portability, unsafe-code and
release constraints remain effective. ADR 0179 supplies the consuming
request-lineage and plain direct-post envelope boundary.

## Introduced or changed constraints

- Error construction consumes exactly one `ValidatedAuthorizationRequest`; it
  cannot accept a substituted target or state.
- The public code is a closed SDK-owned enum covering only the pinned RFC/Final
  vocabulary. It is never inferred from an internal SDK or transport error.
- Optional description is non-empty RFC NQSCHAR ASCII under an explicit byte
  limit. Optional error URI is a bounded RFC URI-reference and is never fetched.
- Form fields are mutually exclusive with success response fields and ordered
  deterministically as error, description, URI, then exact request state.
- Inputs, encoded output and arithmetic are bounded before allocation growth.
- Public errors and diagnostics reveal static categories, enum variants,
  presence flags, counts and lengths only; no target, state, description or URI.
- No crate, unsafe/native code, HTTP runtime, executor, filesystem, clock,
  randomness, signing, encryption, logging, persistence or global state is added.

## Introduced or changed limitations

- Only OpenID4VP 1.0 Final plain `direct_post` Authorization Error Responses are
  constructed. Unknown extension error codes are rejected by type construction.
- Syntax validation does not prove that developer information is truthful,
  localized, display-safe, privacy-safe or authorized by the End-User.
- The envelope confers no transport authority and proves no successful delivery.
- DC API, `direct_post.jwt`, JWE, SIOPv2, HAIP and consumer adapters are separate.

## Consumer and product impact

Headless wallets gain deterministic portable protocol material while retaining
explicit control over privacy and consent. Existing callers are unchanged because
the crate is unpublished and the API is additive. No consumer repository changes.

## Activation and rollback

Activation requires issue #532 preflight, strict OpenSpec validation, official
and negative vectors, property/fuzz/benchmark declarations, native/portable and
dependency evidence, exact-head independent CLI review, green protected CI and
an issue-linked PR. Rollback deletes the additive error types, constructor,
tests, declarations, ADR and delta before publication; no data migration exists.

## Evidence

Authorities are issue #532, OpenID4VP Final sections 8.2/8.5/15.9, RFC 6749,
ADR 0179, IDR-024, this OpenSpec receipt, exact-head CLI review and protected CI.
