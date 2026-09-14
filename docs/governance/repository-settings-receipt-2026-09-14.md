# Repository settings receipt — 2026-09-14

**Repository:** `hyperledger-identus/sdk-rust`

**Observed:** `2026-09-14T08:44:44Z` through the authenticated GitHub REST API
with repository administrator access.

**Authority:** project-sponsor direction to complete issue #26 in the Codex
supervisor session; issue #26 is the durable settings work item.

## Branch state

| Branch | Revision | Contract |
| --- | --- | --- |
| `develop` | `e197810b7d94cfa2e9182b7413110c5edac944db` | active integration branch |
| `main` | `b94d12897a0eb04fe1ea2decbde42a04006aca01` | reserved default branch; no release activation |

The existing `main` revision includes the previously disclosed file-hygiene
workflow-pin change from PR #63. This receipt does not classify that commit as
a release or authorize further `main` changes.

## Effective `develop` rules

Repository ruleset `23274352` (`develop integration`) is active for exactly
`refs/heads/develop`, with no bypass actor. It:

- blocks deletion and non-fast-forward updates;
- requires signed commits and pull requests;
- requires all review conversations to be resolved while requiring zero
  blanket human approvals and no extra approval for unattributed changes;
- requires branches to be current with `develop`;
- requires `DCO`, `pull-request-policy`, `fast`, `File Hygiene (editorconfig)`,
  `Shell Scripts`, `Markdown`, and `YAML` from their observed GitHub Apps; and
- applies GitHub code-quality errors.

Auto-merge is enabled, merged branches are deleted automatically, and web
commits require sign-off. SDK-Rust PR #264 is the operational canary for
pending-check and protected-merge behavior.

## Reserved `main`

Ruleset `16710155` remains active for the default branch. It blocks deletion
and non-fast-forward updates and requires signatures and DCO. `main` remains
outside the normal integration and release flow.

## Security and community controls

| Control | State |
| --- | --- |
| Repository visibility | public |
| Private vulnerability reporting | enabled |
| Dependency vulnerability alerts | enabled |
| Dependabot security updates | enabled |
| Secret scanning | disabled by enterprise policy |
| Secret scanning push protection | disabled by enterprise policy |
| Non-provider pattern scanning | disabled by enterprise policy |
| Secret validity checks | disabled by enterprise policy |

GitHub rejected repository-level secret-scanning activation with HTTP 422:
the enterprise policy owns these controls. No bypass or weaker substitute was
applied. An enterprise owner must change that policy before the repository can
activate the four scanning controls.

## Intentional deviations and follow-up

- The secret-scanning family remains the only observed deviation from the
  desired repository state that cannot be resolved by a repository admin.
- No `crates-io` environment was created; namespace and publishing authority
  remain separate under issue #3.
- The default branch remains reserved `main`; normal pull requests still target
  the explicitly protected `develop` branch.

This receipt is a dated observation, not a self-updating proof. Refresh it after
any ruleset, branch strategy, visibility, security-policy, or release-authority
change.
