# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/364
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. This is chain-neutral bounded OID4VCI protocol behavior; the
Rust 1.98.1, zeroizing ownership, no-I/O, issue-first and honest-target rules
remain unchanged.

## Introduced or changed constraints

- Only the #362 correlated success may enter the new transition, and it is
  consumed whether construction succeeds or fails.
- Credential Endpoint, token, selected configuration and authorized dataset
  cannot be supplied or replaced by the caller.
- Dataset selection uses one checked source-order index; missing selection
  fails before constructing Authorization or JSON output.
- Existing positive `JwtCredentialRequestLimits`, Bearer/proof validation,
  deterministic serialization and redacted diagnostics remain exact.
- One private serializer helper is shared rather than duplicating behavior.
- Existing detached constructors remain available for unpublished compatibility.
- No dependency, feature, lockfile, unsafe/native code, network, storage,
  release or downstream behavior is introduced.

## Introduced or changed limitations

The result proves only that exact locally correlated authority was converted
to a bounded unencrypted wire request. It does not prove endpoint provenance,
token validity/scope/freshness, proof validity, product dataset selection,
request execution, response acceptance, credential trust or storage safety.

## Consumer and product impact

Additive unpublished Rust API only. Consumers gain a stronger one-shot route
while existing configuration-ID and detached authorized-dataset constructors
remain unchanged. No stored data, migration, target, product, chain,
credential-format or release promise changes.

## Activation and rollback

Activation requires issue #364's PR to pass local and hosted gates and merge
to `develop`. Rollback removes the additive method and private correlated-state
decomposition, then restores the serializer helper placement without changing
existing public constructors or output.

## Evidence

Exact wire output, missing index, endpoint/token/identifier non-substitution,
proof/token/body bounds, one-shot ownership, redaction, old-route regression,
portable compilation, factory, Nix, signed+DCO and exact-head hosted CI evidence
are mandatory.
