# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/168
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: the completed audit must include distinct DID registry/cache
  families and safe rejection cleanup.
- `SDK-LIM-007`: its narrowed wording is valid only after these omissions are
  corrected; outer allocation and generic adapter work remain disclosed.
- `SDK-SEC-001`: native JWK rejection remains redaction-safe and non-panicking.

## Introduced or changed constraints

Every distinct implemented input family remains inventory-visible even when
another row already covers its package. Native ownership transfer SHALL NOT
turn a depth rejection into recursive destruction. Hosted review threads SHALL
remain unresolved until exact regression and full gate evidence passes.

## Introduced or changed limitations

Typed validation still cannot prevent prior caller allocation.
Iterative cleanup may use a heap work stack for already allocated hostile
breadth. Concrete cache/clock adapters still own storage, eviction,
synchronization, timeout, cancellation, and backend QoS.

## Consumer and product impact

No public API, dependency, wire, persistence, support-tier, or downstream
migration changes. Consumers gain an accurate ownership map and native JWK
rejection cannot recurse through hostile depth after ownership transfer.

## Activation and rollback

Activation requires updated inventory evidence, hostile-depth regression,
factory/candidate/Nix closure, resolved hosted review, and protected CI.
Rollback restores the broader incomplete-audit limitation; it must not retain
the completed-audit claim while dropping these families or safe cleanup.

## Evidence

The directed issue/PR review, new OpenSpec requirements, inventory rows, exact
regression tests, API baseline, factory/Nix closure, final review, and protected
CI are the evidence chain.
