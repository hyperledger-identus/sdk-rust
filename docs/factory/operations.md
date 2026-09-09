# Factory operations

Repository policy is authoritative. The `factory` vault and Oxid snapshot are
design guidance; they never force a package, toolchain, process or product
upgrade. Changes to tracked versions still require an issue, research,
constraints and the normal dependency gates.

## Production-ready dev-loop

1. Refresh `origin/develop`, select or create one issue, and create an issue
   branch/worktree. `main` is not a delivery target.
2. Create the OpenSpec change. Complete proposal, research, constraint impact,
   capability deltas, design and ordered tasks.
3. Run research and constraint readiness plus strict OpenSpec validation.
4. Commit only the planning contract. With a clean worktree, run:

   ```bash
   scripts/factory preflight <change> --issue <number> --write
   ```

   The receipt binds the issue, branch, `origin/develop` base and planning head.
   It rejects implementation paths in the planning commit. Implementation may
   begin only after this receipt exists.
5. Implement one bounded task at a time. Use `scripts/factory plan --base
   <sha> --head <sha>` to record the exact target plan. The only required hosted
   PR lane remains `fast`; slow evidence is weekly or manually requested.
6. Run focused tests, `./bootstrap.sh --check`, and any risk-routed gates. Record
   commands not run as limitations, not implied successes.
7. Complete a fresh local review. Mark tasks complete, run `factory ready` and
   archive through the guarded facade.
8. Push signed, DCO-bearing commits, open a ready PR that closes the issue, and
   merge into `develop` only at the reviewed exact head with all required checks
   green.
9. Close the managed worktree only with the merged PR number and exact head.

Prototype work uses the same issue and OpenSpec boundary but cannot publish or
claim merge readiness. Promotion to production-ready refreshes the base and
invalidates provisional evidence.

## Bootstrap and local controls

`./bootstrap.sh` enters the pinned shell without mutating user state.

```bash
./bootstrap.sh --audit-pi
./bootstrap.sh --configure-pi
./bootstrap.sh --configure-git
./bootstrap.sh --check
./bootstrap.sh --pi
```

The two configure commands are explicit. Pi configuration preserves unknown
keys and only updates the bounded subagent file. Git configuration refuses to
invent identity or signing material. Neither path reads or writes Pi auth.

## Worktrees and targets

```bash
scripts/factory worktrees audit
scripts/factory worktrees ensure --issue 123 --branch codex/feat/issue-123 \
  --base origin/develop --execute
scripts/factory worktrees closeout-pr --pr 456 \
  --path /absolute/canonical/path --expect-head <sha> --execute
scripts/factory plan --base <sha> --head <sha> --profile production-ready
```

Cleanup rejects dirty, locked, symlinked, current, primary, noncanonical,
wrong-head or unmerged targets. Unknown path classifications fail closed by
recommending the complete slow set; they do not silently expand required PR CI.

## Post-merge canary

After this factory is merged to `develop`, create a separate small issue and
OpenSpec change, launch it through `./bootstrap.sh --pi`, and capture bounded
metrics. Any harness tuning is a follow-up issue driven by that evidence. The
canary does not activate `main` or waive ordinary review and CI.
