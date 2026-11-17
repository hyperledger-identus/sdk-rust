# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/596
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-002`: effective MSRV moves from Rust 1.89.0 / `0.1.x` to
  Rust 1.99.1 / `0.2.x`.
- `SDK-COMPAT-003`: the explicit candidate matrix moves to `0.2.0-rc.1` while
  preserving all host, feature, and portable-target evidence classes.
- `SDK-COMPAT-004`: primary validation moves from Rust 1.98.1 to 1.99.1.
- `SDK-COMPAT-005`: etalon validation moves from Rust 1.98.1 to 1.99.1;
  sanitizer nightly remains tooling-only.
- `SDK-SUPPLY-001`: the stable overlay revision and UniFFI generator lock move
  to immutable patched sources.

## Introduced or changed constraints

Rust 1.99.1 becomes the exact workspace MSRV, primary development compiler,
candidate-preparation compiler, and compatibility etalon for the next
unpublished `0.2.x` line. Every declared feature, host, and portable target
remains gated. The separately pinned nightly remains restricted to sanitizer
tooling.

## Introduced or changed limitations

Rust 1.89 through 1.98 consumers lose the compatibility promise for the
`0.2.x` source line. Historical `0.1.x` receipts remain historical and are not
reissued or relabeled. The compiler change does not activate publication,
stable API, release, Windows, WASI, runtime mobile, or certification support.

## Consumer and product impact

NeoPRISM already builds with a Rust 1.100 nightly and is compiler-compatible;
its immutable SDK revision remains unchanged until a downstream-owned refresh.
Apollo has no Rust SDK dependency. SDK-TS consumes generated boundaries rather
than setting the Rust floor. Other source consumers must install Rust 1.99.1
before adopting the `0.2.x` line.

## Activation and rollback

Activation occurs only after this complete atomic policy/toolchain change
merges through protected `develop`. Rollback reverts the change, returns the
repository to the 0.1/Rust 1.89 policy, and marks the slow lane release-ineligible
until the `rustix` advisory is repaired by another bounded change. No external
release cleanup is needed because publication is not authorized.

## Evidence

The exact issue direction, successor ADR, support-policy/constraint mutation
tests, patched supply-chain audit, workspace/profile/target gates, consumer
compiler inspection, complete local Nix gate, and hosted exact-head CI are the
required evidence. Any unrun target remains visible rather than inferred.
