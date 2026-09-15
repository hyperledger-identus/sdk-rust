# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/276
Constraint blockers: none

## Existing entries affected

- `SDK-SUPPLY-001`: the external emulator image remains exact-path selected;
  unnecessary Google Play/license coupling is removed.
- `SDK-LIM-002` and `SDK-LIM-003`: Android/FFI support remains experimental and
  unsupported despite one AOSP emulator proof.
- `SDK-LIM-004`: the runtime proof remains weekly/manual slow evidence.
- `SDK-REPO-001`: delivery remains through protected `develop`.

## Introduced or changed constraints

The macOS slow lane SHALL install and execute exact package path
`system-images;android-35;default;arm64-v8a`. Workflow, verifier package and
filesystem tag SHALL agree. Automation SHALL NOT execute blanket SDK license
acceptance. A missing accepted license or package SHALL fail closed.

## Introduced or changed limitations

The SDK package path does not pin the catalog's underlying image revision or
archive checksum; current stable revision 2 is research evidence, not a durable
runtime promise. The image requires the ordinary Android SDK license to have
been accepted by runner provisioning. AOSP excludes Google apps/services, which
the smoke test does not use. Only one arm64 API-35 emulator remains proven.

## Consumer and product impact

No Rust, AAR, public API, ABI, wire, feature, MSRV, application target or
consumer dependency changes. No Android support/publication claim is activated.
The effect is limited to the external runtime used for weekly evidence.

## Activation and rollback

Activation requires protected merge and a complete manual slow canary on exact
merged `develop`. Rollback restores the prior package path and known-red license
prompt; it does not require artifact or consumer migration.

## Evidence

Google's stable AOSP and Play catalog snapshots, failed job log, mutation suite,
factory/OpenSpec checks, workflow syntax validation, protected fast CI and the
post-merge hosted macOS execution form the evidence. Natural schedule evidence
remains separately outstanding.
