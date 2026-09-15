# Repository settings transition receipt — 2026-09-15

## Scope

This receipt stages the post-merge administration required by ADR 0120. It does
not claim the settings changed before the reviewed workflow reached `develop`.

## Verified pre-activation state

- repository: `hyperledger-identus/sdk-rust`;
- default branch: `main`;
- `develop` ruleset: enabled and explicitly targets `refs/heads/develop`;
- reserved ruleset: enabled and uses default-branch indirection;
- Actions default token: read-only;
- Actions artifact/log retention: seven days, the organization maximum;
- slow workflow: present on `develop`, absent from the effective default branch.

## Ordered mutation after activation PR merge

1. Export the complete live reserved ruleset document.
2. Change only its branch include condition to `refs/heads/main` and verify the
   remaining enforcement, bypass actors and rules are unchanged.
3. Select `develop` as the default branch.
4. Verify both explicit ruleset targets, required `develop` checks and reserved
   `main` history before dispatching any workflow.
5. Dispatch `slow` at the exact merged `develop` SHA and record its run URL,
   attempt, duration, conclusion, job results and receipt artifact.

## Rollback boundary

If either ruleset cannot be verified after mutation, do not dispatch work.
Restore the exported ruleset, leave or restore `main` as default, and open an
incident issue. Never populate `main`, weaken `develop`, delete run evidence or
convert the manual canary into natural schedule evidence.

## Post-activation evidence

Pending the activation PR merge. This section must be updated with exact live
values and the manual canary before the settings-receipt follow-up merges.
