# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/269
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-002`: raw secret material remains excluded from debug, error,
  serialization and FFI surfaces; public field copying is removed.
- `SDK-LIM-001`: the workspace and candidate remain unpublished, which permits
  replacing the unsafe pre-release API baseline without a compatibility shim.
- `SDK-LIM-006`: no downstream adoption or migration is inferred.
- `SDK-DELIVERY-001`: the material security/API change uses issue, OpenSpec,
  ADR, threat analysis, signed commits, distinct review and protected CI.

## Introduced or changed constraints

HD private-key and chain-code storage must be opaque. Any SDK-created raw
export copy must cross an explicitly named method into a redaction-safe owner
with a defined drop-time erasure contract. The HD and exposure types must not
implement Serde or enter generated bindings.

## Introduced or changed limitations

Best-effort zeroization does not cover caller/compiler copies, registers,
allocator history, swap, crash dumps, hardware, or deliberate caller logging
or serialization after exposure. The new value is not custody, secure storage,
memory locking or a generic secret-management framework.

## Consumer and product impact

External Rust code that reads HD secret fields or constructs the structs with
literals will stop compiling and must use the named exposure methods or SDK
constructors. No released crate or authorized downstream currently has that
commitment. Algorithms, byte outputs, errors, feature names, targets and wire
forms remain unchanged.

## Activation and rollback

The opaque boundary activates when issue #269 merges to `develop` and updates
the unpublished candidate baseline. Rollback restores the public fields and
old baseline by source revert. Publication, releases and downstream migration
remain separate and cannot be activated here.

## Evidence

Compile-fail field/Clone/format probes, public-API inspection for absence of a
Serde surface, runtime redaction and explicit-erasure tests, unchanged
published/Apollo vectors, API-baseline diff, candidate gate, portable compile
lanes, factory checks, distinct security/API review and protected CI provide
evidence.
