# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/352
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. Generic OID4VCI state transitions belong in sdk-rust;
bounded inputs, explicit secret ownership, Rust 1.98.1, issue-first delivery
and honest target claims do not change.

## Introduced or changed constraints

- The transition consumes the issue #350 server-bound state and performs no
  wire parsing, serialization or I/O.
- Configuration selection uses an existing offered-list index and retains no
  duplicate identifier.
- Client identifier, redirect URI and state use explicit positive byte limits.
- Redirect URI is absolute and fragment/userinfo-free; application redirect
  registration policy is not inferred.
- Verifier grammar is the non-configurable RFC 7636 43-to-128-byte range and
  unreserved ASCII set.
- S256 is derived internally through feature-minimal existing SDK primitives;
  a caller cannot supply an unrelated challenge.
- Retained sensitive values are zeroized; Debug and diagnostics are redacted.
- New diagnostics append after every existing error-contract row.

## Introduced or changed limitations

No effective limitation is removed. The state does not generate entropy,
serialize an Authorization Request, choose scope or `authorization_details`,
perform PAR, prove server/client/redirect registration, validate a callback,
compare returned state/issuer, prevent mix-up by itself, exchange a code,
perform I/O or establish successful authorization/issuance.

## Consumer and product impact

Additive unpublished API only. Consumers receive a deterministic, validated
input owner and PKCE relationship without importing OAuth/runtime types. No
wire, storage, migration, product, chain or credential-format behavior changes.

## Activation and rollback

Activation requires issue #352's PR to pass local and hosted gates and merge
to `develop`. Rollback removes only the additive input state, feature-minimal
internal dependency, errors, tests, ADR and capability.

## Evidence

RFC vector, all exact verifier boundaries, invalid characters, client/redirect/
state bounds and syntax, configuration-index failure, ownership, issuer-state
preservation, redaction, dependency-cone, stable error contract, compatibility,
factory, Nix, signed+DCO and exact-head hosted CI evidence is mandatory.
