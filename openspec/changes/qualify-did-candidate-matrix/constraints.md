# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/387
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-001`: all candidate evidence remains stable Rust.
- `SDK-COMPAT-002`: Rust 1.89.0 remains the exact `0.1.x` floor.
- `SDK-COMPAT-003`: the bounded compiler/package/profile/host/target matrix is
  extended to the independently governed DID train without changing the first
  crypto train.
- `SDK-COMPAT-004` and `SDK-COMPAT-005`: Rust 1.98.1 remains primary/etalon;
  sanitizer nightly remains unrelated.
- `SDK-DELIVERY-001`: one required Linux fast lane is preserved; matrix work is
  weekly/manual slow evidence.
- `SDK-LIM-002`: no supported FFI or binding surface is introduced.
- `SDK-LIM-003`: portable results remain compile-only and do not establish
  runtime/device/browser support or certification.
- `SDK-REL-001` and `SDK-REL-002`: DID remains candidate-only and cannot
  publish through this change.

## Introduced or changed constraints

The DID candidate gains one closed matrix: both packages are host-qualified on
Linux/macOS using primary and MSRV compilers, while only `identus-did` is
portable-target eligible. This activates evidence already directed by M5 and
does not alter the compiler values or global support tiers.

## Introduced or changed limitations

`identus-did-resolver-http` is explicitly host-only for this candidate. A
successful incidental portable compilation cannot upgrade that state. All
portable `identus-did` results remain compilation evidence only. Linux plus
complete cross-host results are unavailable until a natural or separately
authorized manual slow run executes the merged wiring.

## Consumer and product impact

Rust consumers can review a package-specific portability statement instead of
inferring support from workspace gates. Existing exact-Git consumers, Cargo
features, runtime behavior, APIs, downstream products, and rollback paths do
not change.

## Activation and rollback

The descriptor, ADR, runner, Nix apps, workflow, checker, tests, documentation,
and canonical specs activate atomically on protected `develop`. Rollback
reverts those repository-local additions. It cannot relabel missing evidence
as passing and does not affect the published crypto train or any registry.

## Evidence

Preimplementation readiness, structural mutation tests, synthetic receipt
rejection, local macOS primary/MSRV lanes, Nix evaluation, workflow policy,
factory/OpenSpec, review, and exact-head fast CI are required. Final M5 must
also attach a clean aggregate receipt from a natural or authorized manual slow
run at the unchanged candidate SHA.
