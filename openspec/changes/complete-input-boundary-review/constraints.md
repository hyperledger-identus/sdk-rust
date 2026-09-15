# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/168
Constraint blockers: none

## Constraints

Every distinct implemented input family remains inventory-visible even when
another row already covers its package. Native ownership transfer SHALL NOT
turn a depth rejection into recursive destruction. Hosted review threads SHALL
remain unresolved until exact regression and full gate evidence passes.

## Limitations and consumer impact

The correction does not make typed validation prevent prior caller allocation.
Iterative cleanup may use a heap work stack for already allocated hostile
breadth. Concrete cache/clock adapters still own storage, eviction,
synchronization, timeout, cancellation, and backend QoS. No public API,
dependency, wire, persistence, support-tier, or downstream migration changes.

## Activation and rollback

Activation requires updated inventory evidence, hostile-depth regression,
factory/candidate/Nix closure, resolved hosted review, and protected CI.
Rollback restores the broader incomplete-audit limitation; it must not retain
the completed-audit claim while dropping these families or safe cleanup.
