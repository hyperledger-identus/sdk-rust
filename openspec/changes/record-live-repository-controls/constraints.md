# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/26
Constraint blockers: none

## Existing entries affected

- `SDK-REPO-001`: the documented protected `develop` integration path is now
  active in GitHub.
- `SDK-REPO-002`: `main` remains reserved, protected and outside release flow.
- `SDK-DELIVERY-001`: required PR/CI evidence is enforced by the live ruleset.

## Introduced or changed constraints

No new constraint. The repository records activation of an accepted one.

## Introduced or changed limitations

Secret scanning, push protection, non-provider patterns and validity checks
remain disabled under enterprise policy and require enterprise-owner action.

## Consumer and product impact

Contributors and agents receive an enforced, accurate `develop` merge boundary.
Rust consumers and downstream repositories are unchanged.

## Activation and rollback

The live ruleset is already active under sponsor direction. This PR makes local
evidence match it. Rollback requires a protected settings decision and matching
inventory update; documentation alone cannot deactivate a live rule.

## Evidence

The dated settings receipt records effective rules, security status, branch
SHAs and the enterprise-policy deviation.
