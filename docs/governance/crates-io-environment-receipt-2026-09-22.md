# crates.io protected-environment receipt — 2026-09-22

Repository: `hyperledger-identus/sdk-rust`

Environment: `crates-io`
Environment ID: `22476721593`

The GitHub Environments API reported these controls immediately after
configuration:

| Control | Observed value |
| --- | --- |
| Required reviewer | organization team `hyperledger-identus/identus-maintainers` (team ID `12106599`) |
| Prevent self review | `true` |
| Administrators may bypass protection | `true` (must become `false` before dispatch) |
| Deployment refs | protected branches only |
| Custom branch policies | `false` |
| Wait timer | `0` |

The release workflow re-reads and validates those non-secret controls before it
builds release evidence, including a fail-closed requirement that administrator
bypass is disabled. GitHub enables administrator bypass by default, and the API
used to create the environment does not expose that setting. A repository owner
must deselect **Allow administrators to bypass configured protection rules** in
the environment UI before the first dispatch. A deployment is dispatched from
protected `develop`; the signed tag and exact target SHA are separate verified
inputs.

Secret values are intentionally absent from this receipt. The GitHub identity
used for configuration could not enumerate an environment- or repository-level
`CARGO_PUBLISH` secret, and could not enumerate organization secrets without
organization-admin secret permission. The bootstrap job therefore treats the
secret as unproven and fails closed when it is unavailable. A successful
protected publication run is the only acceptable availability evidence; it
must never print or persist the token.

After namespace creation, the release manager configures crates.io trusted
publishing for all three crates, revokes the bootstrap token, removes the
environment secret, and attaches those external receipts to issues #3/#326.
