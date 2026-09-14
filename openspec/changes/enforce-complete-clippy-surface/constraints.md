# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-004`: every ordinary check remains on primary Rust 1.98.1.
- `SDK-COMPAT-005`: nightly remains isolated to sanitizer tooling.
- `SDK-CI-001`: the exact required fast gate set remains unchanged.
- `SDK-LIM-004`: complete slow evidence remains weekly/manual and
  pre-release, not a per-PR compatibility promise.

## Introduced or changed constraints

The existing weekly/manual slow lane gains an explicit generated Clippy check
covering `--workspace --all-targets --all-features -- -D warnings` on both
host systems. Narrow first-party lint exceptions require local scope, a reason,
an owning component, and an objective removal condition.

## Introduced or changed limitations

The complete check does not become a required pull-request status during the
temporary active-development period. The two public OID4VCI constructors retain
their existing arity until a separate API change supplies a compatible
migration; their narrow expectations are recorded debt, not a waiver of other
warnings.

## Consumer and product impact

No Rust or foreign-language consumer API, wire representation, resource limit,
dependency, feature, target, or support promise changes. Consumers may gain
earlier warning detection from weekly/manual evidence only. Consumer
repositories remain read-only.

## Activation and rollback

Merge to protected `develop` activates the slow check and lint policy. A
focused revert removes the generated gate and restores the former warning
findings; it does not require data or consumer migration. Fast CI selectors
remain unchanged in both directions.

## Evidence

The support-policy validator will bind the gate's operation, primary
toolchain/artifacts, workspace selection, all-target and all-feature flags,
warning denial, host policy references, and explicit slow-workflow selectors.
Mutation tests will prove each load-bearing field fails closed.
