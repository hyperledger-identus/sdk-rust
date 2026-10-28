# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/564
Constraint blockers: none

## Existing entries affected

The native-binding isolation, security-audit, Nix, CI slow-lane and no-public-
FFI constraints remain authoritative. This change narrows only the standalone
generator audit treatment for one maintenance advisory.

## Introduced or changed constraints

- Only RUSTSEC-2026-0374 may be accepted, and only for `basic-toml 0.1.10` on
  the exact UniFFI generator path recorded in ADR 0191.
- The acceptance expires after 2027-01-31 UTC without an automatic renewal.
- The guard must fail before audit when the date, version, path, lock identity
  or expected root changes.
- Generator ban/license/source `cargo deny` checks remain strict. `cargo audit`
  is the sole advisory authority for the tool lock; root/fuzz audits and every
  other advisory remain strict and ignore-free.
- Swift, Kotlin/JVM, Apple and Android generation/consumer E2E checks remain
  blocking in the weekly macOS lane.
- A vulnerability affecting this path invalidates the acceptance immediately.

## Introduced or changed limitations

Until upstream removes the dependency, the unpublished generator retains one
known unmaintained build-time crate. This does not establish precedent for
runtime, cryptographic, protocol, storage or network dependency exceptions.

## Consumer and product impact

There is no consumer-visible API, ABI, package or behavior change. The change
restores honest production-readiness evidence without claiming the underlying
maintenance concern is resolved.

## Activation and rollback

Activation requires issue #564 planning evidence, ADR 0191, deterministic guard
tests, live generator and host E2E evidence, independent CLI review and green
protected CI. Rollback removes the acceptance but returns the slow lane to a
known red state; the preferred exit is an upstream dependency update.

## Evidence

Issue #564, ADR 0191, the OpenSpec archive, exact dependency tree, guard tests,
live audit/deny output, host E2E output, natural scheduled run and protected CI
form the evidence set.
