# Record activated repository controls

## Why

Issue #26 now has explicit project-sponsor authority and its primary live
controls have been applied. Repository-local evidence still claims that all
GitHub activation is external work, so agents and contributors cannot determine
the real merge boundary from the repository.

## What changes

- Record the active `develop` ruleset and enabled security/community controls.
- Preserve the enterprise-controlled secret-scanning deviation explicitly.
- Update the executable governance inventory and README without changing the
  reserved `main`, release, publication or maintainer authority.

## Capabilities

### Modified capabilities

- `sdk-governance-evidence`: distinguish active live controls from remaining
  enterprise-admin work.

## Non-goals

- No `main` population, release, publication, environment or ownership change.
- No bypass of the enterprise policy that rejected secret-scanning activation.
- No Rust, Cargo, Nix, API or downstream change.

## Delivery

Issue #26 owns this settings receipt from
`develop@e197810b7d94cfa2e9182b7413110c5edac944db`.
