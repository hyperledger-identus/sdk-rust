# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The checker enables `set -euo pipefail` and evaluates each required field with
`printf '%s\n' "$pr_body" | grep -q`. GNU/BSD grep may stop reading after an
early match. For a body larger than the pipe buffer, `printf` can then receive
`SIGPIPE`; `pipefail` reports the pipeline as failed even though grep matched.
The outcome depends on match position and body size rather than policy.

## Normative sources

Issue #374 defines the defect and expected behavior. The canonical
`factory-operations` specification, `scripts/check-pr-policy.sh`, the 64 KiB
delivery-facade bound, and the existing policy tests govern the correction.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Disable `pipefail` | `not-adopt` | Weakens unrelated shell failure detection. |
| Append required fields at EOF | `not-adopt` | Leaks an implementation accident into contributor formatting. |
| Add `|| true` around pipelines | `not-adopt` | Can hide real non-matches and errors. |
| Pass the body through a here-string to grep | `not-adopt` | Fast on macOS Bash 3.2, but the pinned Linux Bash 5.3 stalled while materializing the 60 KiB regression input. |
| Write one mode-0600 temporary file and trap cleanup | `adopt` | Gives grep a seekable bounded input on both supported hosts without a producer pipeline or early-close race. |

## Compatibility and dependency evidence

No public SDK API, dependency, feature, lockfile, toolchain, target, wire
format or consumer changes. The checker continues to require Bash and grep,
which are already part of the factory environment.

## Security, privacy and maintenance evidence

The 64 KiB file-backed delivery bound and safe-file checks are unchanged. The
same ERE patterns, case handling, issue extraction and error messages remain
authoritative. No untrusted text is evaluated as shell code or printed by the
checker. The checker creates one unpredictable mode-0600 file below the
platform temporary directory and removes it on every ordinary exit by trap.

## Rejected or deferred candidates

Disabling `pipefail`, formatting workarounds, error-masking pipelines and
here-strings are rejected. Replacing the regex contract or contribution policy
is outside this correction.

## Open questions and blockers

No implementation blocker remains. The temporary representation retains the
checker's existing trailing-newline input shape.

## Evidence

- Issue #374 and PR #373 reproduce the false failure.
- `scripts/check-pr-policy.sh` is the affected policy owner.
- `scripts/tests/pr-policy.sh` and `scripts/tests/factory-contract.sh` are the
  existing deterministic test owners.

## Evidence commands

Before implementation: `scripts/factory research-ready`,
`scripts/factory constraints-ready`, `scripts/factory validate`, focused
source inspection, and a bounded reproduction. Implementation and mutation
tests remain delivery tasks.
