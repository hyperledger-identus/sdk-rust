# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/573
Constraint blockers: none

## Existing entries affected

ADR 0193 already requires independent differential oracles while retaining
Identus-owned APIs and a focused production graph. SDK-ARCH-001 through
SDK-ARCH-003, SDK-SEC-001 through SDK-SEC-003, SDK-DEP-001,
SDK-DELIVERY-001, and the exact draft-22 constraint remain unchanged. The
quality declaration `credentials.sd-jwt-vc.draft22.quality.v1` changes only
its differential evidence state after executable proof.

## Introduced or changed constraints

- Exactly two independent maintained engines are pinned by release, revision,
  archive integrity, license, toolchain, dependency graph, and advisory receipt.
- Oracle engines remain absent from Cargo manifests, Cargo.lock, production
  Nix closures, release artifacts, and public Rust types.
- Every packet result is normalized to stable semantic categories. Foreign
  DTOs, error text, timestamps, signatures, and serialization order are not
  contractual.
- Draft-22, not implementation consensus, resolves disagreements.
- Checked-in packets are bounded, offline, non-secret, immutable by version,
  and executable without JavaScript, PHP, package managers, or HTTP.
- `satisfied/source-bound` is permitted only after both oracle receipts and the
  SDK differential route pass at one exact SDK head.

## Introduced or changed limitations

- The selected releases predate draft-22 and are behavioral oracles, not
  complete normative authorities. Missing surfaces are recorded as
  `not-exposed`; disagreements remain visible.
- HTTP retrieval, rendering, issuer authorization, X.509 trust, status-list
  acceptance, and downstream runtime interoperability remain outside the Rust
  profile and cannot be inferred from packet success.
- Advisory receipts are point-in-time evidence for isolated oracle graphs, not
  a promise about future ecosystem state.
- This change does not publish `identus-sd-jwt-vc` or activate the legacy
  adapter.

## Consumer and product impact

No consumer contract changes. Maintainers gain auditable evidence that the
local profile agrees with or deliberately diverges from two independent
engines on the specified semantic surface. Oxid and other consumers still
adopt the crate only after a separate publication decision.

## Activation and rollback

Activation requires issue #573 preflight, exact source receipts, passing
upstream probes, packet schema and provenance validation, focused Rust tests,
quality-plan regeneration, independent review, green protected CI, and merge
to `develop`. Rollback restores the differential obligation to pending and
removes only the new packet/report; it does not alter profile behavior.

## Evidence

Evidence authorities are the draft-22 source hash, candidate release and
archive identities, frozen dependency/advisory receipts, source selectors,
neutral packet, disagreement ledger, Rust differential test, exact SDK head,
review, and protected CI.
