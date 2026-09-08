# Distinct review

Review date: 2026-09-09
Reviewer role: security, compatibility, and architecture review
Base revision: `04266d649d83d27213a11cea88cab064ec5d4811`
Scope: issue #209 exact diff plus specification, constraints, and ADR

## Outcome

Approved with no unresolved findings.

## Security review

- The borrowed parser invokes `validate_identifier(value, limit)` before the
  only `to_owned()`. The existing validator checks byte length before trimming
  or control-character traversal, bounding rejection work before allocation.
- Exact and one-over tests cover all six generated identifier types through
  both borrowed and owned constructors. A later malformed canary on over-limit
  input exercises deterministic short-circuit precedence.
- Local `Display` and `Debug` output omit the rejection canary. No unsafe,
  secret, PII, FFI, native code, build script, or dependency was introduced.

## Compatibility review

- Accepted values, public signatures, grammar, 256/1,024-byte ceilings, owned
  move semantics, and Registration lifecycle behavior are unchanged.
- The change affects only rejection allocation ordering. It has no wire or
  stored-data migration and can be rolled back by reverting the focused PR.

## Architecture and dependency review

- The change remains in generic `identus-did`; no PRISM, Midnight, Cardano,
  Oxid, transport, ledger, wallet, or product policy moves upstream.
- Direct local validation is the cohesive implementation. A dependency cannot
  replace private macro call order or own SDK-selected resource ceilings.

## Residual limitations

Outer transports and deserializers can allocate before calling the SDK.
`SDK-LIM-007`, #168, and all unreviewed inherited boundaries remain active.
