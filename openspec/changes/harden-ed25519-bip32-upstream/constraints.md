# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/179
Constraint blockers: none

## Existing entries affected

ADR 0078 already directs an upstream-first attempt and permits a bounded fork
only on an objective failure trigger. `SDK-SEC-001` remains effective for SDK
source; the upstream patch improves a third-party boundary and introduces no
SDK unsafe exception. `SDK-SEC-002` remains effective because the SDK-owned
private facade and explicit export are unchanged.

The SDK's Rust 1.98.1 policy is unchanged. The upstream crate's independent
Rust 1.81 declaration is preserved by pinning a compatible zeroize line rather
than imposing the SDK's active-development compiler policy on the upstream.

## Introduced or changed constraints

The dependency research contract gains an upstream-first rule: when a
conditionally adopted dependency has a focused remediable defect, agents SHALL
offer the minimal patch upstream and record its disposition before activating
an SDK-maintained fork. The rule does not require indefinite waiting and does
not permit an upstream edit to change SDK public behavior silently.

## Introduced or changed limitations

Until upstream merges and releases the patch, the SDK continues to resolve
`ed25519-bip32 0.4.3` and its current residual risks. An open pull request is
coordination evidence, not a fixed dependency artifact or security remediation.
The proposed zeroize dependency may cause two zeroize versions in consumers
that require 1.9; a later SDK update must measure this exact lockfile outcome.

## Consumer and product impact

No SDK, Apollo or NeoPRISM code changes in this iteration. A future released
upstream version can reduce transitive risk without changing the Identus API or
Cardano V2 bytes. Downstream adoption remains separately owned.

## Activation and rollback

The upstream contribution activates only if its maintainers merge it. The SDK
continues to use 0.4.3 until a separate issue validates and pins an immutable
release. If upstream declines or does not release before the first publishable
SDK candidate, issue #179 permits a separate minimal-fork decision with a
sunset condition. Closing the contribution leaves current behavior unchanged.

## Evidence

Acceptance requires upstream vector and formatting tests, Rust 1.81, `no_std`,
host and portable-target compilation, feature-tree and unsafe scans, audit,
exact-diff review, and a linked upstream issue or pull request.
