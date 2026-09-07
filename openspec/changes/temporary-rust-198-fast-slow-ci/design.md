## Context

The flake already generates named Crane checks from one gate manifest and
shares dependency artifacts. The current workflows invoke every check on two
hosts per PR while fuzz workflows add nightly jobs. The safest simplification
is scheduling and provider convergence, not replacing Nix or deleting checks.

## Decisions

### One stable SDK compiler, one tooling exception

Keep the existing Rust 1.98.1 primary toolchain. Alias the compatibility/MSRV
Crane providers and full-feature artifacts to that same stable compiler so old
gate identities can continue to describe feature coverage without compiling
with three Rust versions. Add a separately named `fuzzToolchain` for the pinned
nightly; it is available only in the fuzz devshell.

### One Linux fast status

The existing factory workflow becomes workflow/job status `fast`. It builds
the manifest-derived `factory-contract`, `rust-fmt`, `rust-build`,
`rust-clippy`, and `rust-test` derivations on `ubuntu-latest`. These share Nix
and Crane artifacts and cover repository validity plus the normal contributor
loop. Lightweight DCO, PR-policy and file-hygiene workflows may remain separate
checks, but `fast` is the single Rust/factory integration signal.

### Full flake becomes weekly/manual slow evidence

The existing Nix matrix keeps both Ubuntu and macOS and still runs
`nix flake check`; only its triggers change to weekly schedule and manual
dispatch. It retains portable targets, isolated features, docs, audit, deny and
all repository checks. It is not a required PR signal during the temporary
phase.

### Sanitizers are weekly/manual experimental evidence

Remove pull-request and push triggers from all sanitizer workflows. Each keeps
a staggered weekly schedule and manual dispatch and runs a bounded soak in the
nightly fuzz shell. Their names and output must explicitly avoid claiming Rust
1.98 compatibility evidence.

### Preserve stable gate identities during the temporary phase

Retaining existing manifest gate names minimizes Nix/validator churn and keeps
historical receipts navigable. `rust-msrv` and `rust-etalon` become slow
feature/build evidence on the same 1.98.1 compiler; policy explicitly states
that they are not independent compiler lanes. A release-phase redesign may
rename or replace them with consumer-driven gates.

## Risks and mitigations

- Fast can omit an essential check: machine policy enumerates its exact gates,
  validator tests reject trigger or selection drift, and slow covers the full
  graph weekly.
- Aliases can secretly select a second stable compiler: the support-policy
  checker binds all ordinary providers to the same exact toolchain.
- Nightly can leak into ordinary work: only `fuzzToolchain` enters the fuzz
  devshell; primary checks reject nightly-only source.
- Scheduled failures can be ignored: release guidance makes all slow failures
  blockers before a candidate, and the policy has a dated review trigger.
- Branch protection may not enforce `fast`: the runbook names the exact status
  and requires an observed blocking test; the implementation does not claim
  settings are already active.

## Rollout

Update specifications and constraints first, pass readiness, then update Cargo,
Nix, workflows, validator/tests and contributor docs. Run the selected fast
derivations and full local slow matrix, complete a distinct review, archive the
change, open the issue-linked PR, and merge only after every hosted check
triggered on that PR is green.
