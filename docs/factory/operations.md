# Factory operations

Repository policy is authoritative. The `factory` vault and Oxid snapshot are
design guidance; they never force a package, toolchain, process or product
upgrade. Changes to tracked versions still require an issue, research,
constraints and the normal dependency gates.

## Production-ready dev-loop

1. Refresh `origin/develop`, run `scripts/factory backlog-live`, select or
   create one issue, and create an issue branch/worktree. The live audit is a
   read-only supervisor gate, not part of offline required CI. `main` is not a
   delivery target.
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
   <sha> --head <sha>` to record the exact target plan, text churn, integration
   line, promotion line and decomposition guidance. The only required hosted
   PR lane remains `fast`; slow evidence is run locally or requested through
   native GitHub scheduling from protected default `develop`, manual dispatch,
   or exact local reproduction. Run `scripts/factory slow-live` after the
   weekly window to fail closed on missing, stale or unhealthy scheduled
   evidence; the command is read-only and never dispatches recovery work.
6. Run focused tests, `./bootstrap.sh --check`, and any risk-routed gates. Record
   commands not run as limitations, not implied successes.
7. Complete a fresh local review, then batch a candidate push. Request one
   automatic discovery review only after the first green fast head. Perform at
   most one remediation round: P0/P1, security regression, introduced defect
   or failed acceptance remains blocking; later independent P2/P3 hardening is
   linked to a follow-up issue. Mark tasks complete, run `factory ready` and
   archive through the guarded facade.
8. Push signed, DCO-bearing commits, open a ready PR that closes the issue, and
   merge into `develop` only at the reviewed exact head with all required checks
   green.
9. Close the managed worktree only with the merged PR number and exact head.

Prototype work uses the same issue and OpenSpec boundary but cannot publish or
claim merge readiness. Promotion to production-ready refreshes the base and
invalidates provisional evidence.

## CI delivery lines

`fast` answers whether one bounded active-development slice may integrate into
`develop`. It is the single required Linux status and retains policy, OpenSpec,
formatting, normal workspace build, strict Clippy, tests and bounded first-party
analysis under the pinned toolchain. Its observational execution SLO is p50 at
most six minutes and p95 at most eight minutes. A comparable rolling p95 above
ten minutes starts focused optimization; it does not authorize silent evidence
removal.

`slow` answers whether one exact unchanged SDK candidate may be promoted toward
production or release. It retains the complete platform, target, binding,
security, conformance, coverage, performance, fuzz/sanitizer, deterministic
package and receipt evidence. It runs weekly from protected `develop` and
manually after a candidate becomes stable. A failure blocks production
promotion, publication and release preparation, but does not create another
ordinary PR matrix or invalidate unrelated green integrations.

Work locally before using hosted CI. The default delivery budget is one
candidate push, one automatic discovery review and at most one remediation
push. More than 12 changed files or 1,000 changed text lines requires a
decomposition note explaining why the slice remains cohesive or how it will be
split; the threshold alone never approves or rejects correctness.

## Bootstrap and local controls

`./bootstrap.sh` enters the pinned shell without mutating user state.

```bash
./bootstrap.sh --audit-pi
./bootstrap.sh --prepare-pi-cache
./bootstrap.sh --configure-pi
./bootstrap.sh --configure-git
./bootstrap.sh --check
./bootstrap.sh --pi
```

Use `./bootstrap.sh --check` as the single local health verdict. It runs the
structural/OpenSpec contract, effective pinned-runtime audit and operational
tests in that order inside one devshell, and propagates the first failure.
For a runtime-only diagnostic, `scripts/factory audit` enters the repository
devshell once when called from the host and runs directly when already inside
Nix. Its arguments and exit status are preserved across that boundary.

The two configure commands are explicit. Pi configuration preserves unknown
keys and only updates the bounded subagent file. Git configuration refuses to
invent identity or signing material. Neither path reads or writes Pi auth.

## Supervisor-to-Pi boundary

Use the [closed supervisor contract](supervisor.md) when a persistent supervisor
launches a bounded worker. `scripts/factory supervisor prepare` validates the
current branch, exact `origin/develop` base, head and active preflight receipt,
then records only a task identity plus path/tool/deadline bounds in a private
Git-common-dir run. `supervisor run` accepts worker input on standard input and
launches only through `./bootstrap.sh --pi`; worker output, the persisted Pi
session and stderr remain private.

Liveness comes from the supervisor heartbeat, not model text. Worker exit,
timeout and handoff acceptance are separate states. Acceptance requires exact
identity, an unchanged head, declared Git effects equal to the current changed
paths, allowlisted path prefixes, closed acceptance/check/finding outcomes, no
remaining worker-owned process and a safe supervisor-owned next action.
`supervisor harvest` reduces a supported Pi v3 session/event pair to counters
without emitting content or runtime identifiers. See the supervisor document
for commands and schemas.

Before `--pi` starts the agent, bootstrap prepares the exact project packages
in a content-addressed cache under a hidden sibling of the primary checkout.
For a primary checkout named `sdk-rust`, the root is
`../.sdk-rust-factory/pi-packages/v1/<sha256>`. Each worktree keeps Pi's
expected `.pi/npm` path as an ignored symlink to that complete store. The key
binds Pi, Node, npm, the ordered exact package declarations and the tracked
resolved npm lock, so identical worktrees share one installation and changed
inputs select another. First population uses `npm ci` against that lock.

Package lifecycle scripts are disabled during population. The store contains
packages and a closed identity marker only; Pi authentication, sessions,
prompts, transcripts, providers and models are not moved or inspected. Initial
concurrent workers populate private staging directories and atomically converge
on one verified store. No automatic pruning runs. Use `--prepare-pi-cache` to
prepare or inspect the reported exact path before a Pi launch.

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

At terminal closeout, the supervisor first finalizes the closed metrics JSON,
then runs `scripts/factory metrics publish --file <record> --issue N --target
auto --execute`. The command retains or confirms the local exact-head record
before creating or updating the bounded PR comment; it falls back to the issue
when the record has no PR. Use `--target issue` for an explicit issue receipt.
If the bounded retry is exhausted, record visible telemetry debt and preserve
the local record; do not reinterpret independent product gates.
