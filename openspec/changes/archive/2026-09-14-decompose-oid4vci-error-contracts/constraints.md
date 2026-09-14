# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001/002/003`: OID4VCI ownership remains chain/product-neutral,
  crate-local, protocol-cohesive, and inward-only.
- `SDK-COMPAT-002/004/005`: Rust 1.98.1 and the temporary target evidence
  policy remain unchanged.
- `SDK-SEC-001/002`: no unsafe code is added and diagnostics remain
  static/redacted.
- `SDK-DELIVERY-001`: #271/#277, ADR 0119, planning receipt, exact review, and
  protected-`develop` delivery govern the slice.
- `SDK-LIM-001/002/003/006/007`: unpublished status, FFI, runtime targets,
  consumers, and inherited resource-bound limitations do not change.

## Introduced or changed constraints

No cross-cutting constraint changes. ADR 0119's three-field private record and
six protocol catalogue groups are slice-level acceptance criteria, not a
workspace standard or a shared dependency.

## Introduced or changed limitations

The change does not provide a new OID4VCI flow, broader profile coverage,
retryability, structured error metadata, localization, public introspection,
bindings, runtime/device proof, or downstream adoption. The golden covers only
the 171 fieldless `CredentialOfferError` variants at the exact assessed base.

## Consumer and product impact

No consumer action or OID4VCI/JOSE/wire behavior change is expected. Existing
typed protocol error responses, parsing limits, endpoints, transports,
metadata, token, credential, nonce, and issuance behavior remain exact. No
consumer repository is inspected or modified. Issues #7 and #168 remain
outside the delta.

## Activation and rollback

Activation is merge of one issue-linked PR to protected `develop` after green
CI and resolved review. A focused source/test/tooling revert restores the
single explicit match. No data, wire, release, or consumer migration is
required.

## Evidence

The exact base, planned immutable 171-row golden, ADR 0119, source inventory,
target-policy boundary, and implementation gates are recorded in this change.
No material constraint activation is requested or permitted.
