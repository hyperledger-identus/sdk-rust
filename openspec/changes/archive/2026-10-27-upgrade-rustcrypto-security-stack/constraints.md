# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/525
Constraint blockers: none

## Existing entries affected

The crypto facade, workspace dependency convention, MSRV, target, unsafe-code,
security-audit and release constraints remain authoritative. Issue #525 directs
removal rather than suppression of both advisory findings.

## Introduced or changed constraints

- `identus-crypto` retains its SDK-owned methods and exact byte behavior. The
  four public tuple fields that expose RustCrypto key types move from 0.13 to
  0.14 and are governed by `crypto.secp256k1.rustcrypto-security-stack.v1`.
- RustCrypto family versions move together; no duplicate old curve stack may
  remain in root or fuzz lockfiles.
- Tokio retains the existing feature set and cannot enter a domain crate.
- Root and fuzz `cargo audit --deny warnings` must pass with zero ignores.
- Existing crypto vectors and native/WASM/iOS/Android gates are blocking.
- No new unsafe block, native backend, secret-bearing diagnostic or default
  feature may be introduced.

## Introduced or changed limitations

This decision establishes compatibility only for the selected published crate
versions and existing enabled features. It does not promise every upstream
RustCrypto API or Tokio facility as part of the SDK contract.

## Consumer and product impact

Consumers using SDK constructors, encoders, signers and verifiers receive the
same behavior and wire values with a corrected dependency closure. Consumers
that directly construct or read the published tuple fields must update their
`k256`/`p256` types to 0.14 or migrate to the SDK byte boundaries documented in
`docs/release/identus-crypto-rustcrypto-0.14-migration.md`. The change lands on
the release-candidate line before a stable `0.1.0` contract.

## Activation and rollback

Activation requires issue #525's exact planning receipt, ADR 0178, clean
dependency evidence, all existing crypto vectors and targets, independent CLI
review and protected CI. A rollback that restores an affected version is not a
valid release state; remediation must move forward or be covered by a separate
time-bounded risk decision.

## Evidence

The issue, ADR, RustSec records, published crate metadata, exact lockfiles,
tests/vectors, portable builds, Nix gates, review and protected CI form the
evidence set.
