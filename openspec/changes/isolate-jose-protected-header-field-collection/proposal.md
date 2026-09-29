# Isolate JOSE protected-header field collection

## Why

The production module-decomposition milestone is complete. The next unowned
security-sensitive function signal is
`RawProtectedHeaderVisitor::visit_map`: 67 SLOC / cognitive 13 / cyclomatic 31.
It combines Serde trait adaptation, closed-name dispatch, duplicate and typed
value handling, required-algorithm enforcement, exclusive key-reference
correlation, and raw result construction. Issue #470 requires characterization
before movement because its first static error is observable protocol behavior.

## What changes

- Bind duplicate, unknown, invalid-value, missing-algorithm, and ambiguous-key
  priority before production edits.
- Keep `identus-jose` and every public protected-header type unchanged.
- Introduce a private closed member classifier plus one private raw-field owner;
  leave the Serde visitor as the adapter that iterates entries.
- Preserve exact accepted JSON, limits, decoded values, exclusive key-reference
  semantics, stable errors, redaction, and allocation classes.
- Remove the touched signal only if the private owners remain cohesive and
  create no equivalent replacement signal.

## Capability

### Modified capability

- `jws-compact`: protected-header parsing gains explicit private field
  collection ownership with unchanged outward behavior.

## Non-goals

No new JOSE algorithm, JWE/general JWT surface, certificate/attestation/trust
validation, OIDC/OID4VCI policy, public parser/builder, third-party JOSE engine,
dependency, feature, error, wire, limit, or target change.

## Delivery

Issue #470 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, Discussion #399 update, and metrics. Protected squash delivery
requires an implementation PR followed by a canonical-evidence closeout.
