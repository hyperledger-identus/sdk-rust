## Why

Backlog item `IDR-005` requires one chain-neutral DID document boundary before
PRISM, Midnight, Lace and Oxid can share document handling safely. The SDK now
owns validated DID and DID URL values, but consumers still carry incompatible
document shapes, implicit limits and method-specific authorization rules.

GitHub issue #37 defines this bounded child of parent #5. It delivers the
structural W3C DID Core document, relationship and service model. Resolution,
cryptosuite interpretation, DID method policy and downstream migrations remain
separate changes.

## What Changes

- Add immutable validated DID document, verification method, relationship and
  service domain types to `identus-did`.
- Add a small bounded absolute RFC 3986 `Uri` value for aliases, service ids and
  string service endpoints.
- Preserve W3C scalar-or-array forms and unknown extension entries across
  semantic JSON round trips.
- Validate public JWK/multibase structural boundaries without assigning curve,
  controller, relationship or cryptosuite policy.
- Bound raw documents, collections, strings and arbitrary extension trees.
- Reject duplicate identifiers, reserved-key collisions, private JWK members
  and simultaneous known verification material encodings.
- Pin standards-derived, donor-shaped and adversarial tests plus a release-mode
  document parsing diagnostic.
- Record the ownership, extensibility and policy boundary in ADR 0009.

## Capabilities

### Modified Capabilities

- `did-core`: extend the lexical DID capability with a bounded, extensible W3C
  DID document structural model.

## Impact

- **Issue:** #37, child of #5 (`IDR-005`).
- **API:** additive types in the unpublished `identus-did` package.
- **Dependencies:** promote the existing workspace `serde_json` dependency from
  dev-only to production; add no new third-party package.
- **Consumers:** downstream repositories gain one adapter target, but none is
  modified by this change.
- **Naming:** publication remains governed by #3; no package rename occurs.
- **Rollback:** revert issue #37's pull request. No published API, persisted SDK
  storage or downstream migration is involved.
