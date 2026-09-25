# Make PR policy matching pipefail-safe

## Why

Issue #374 records a deterministic false rejection in
`scripts/check-pr-policy.sh`: with `set -o pipefail`, an early successful
`grep -q` match can close the pipe while `printf` is still writing a long,
bounded PR body. The producer then exits on `SIGPIPE`, turning valid metadata
into a policy failure. The same defect affected the #373/#379 delivery flow.

## What changes

- Match the already-bounded `PR_BODY` without a producer pipeline whose status
  can be changed by an early-exiting consumer.
- Preserve the accepted issue, local-review, constraint-impact and limitation
  syntax and existing diagnostics.
- Add long-body regressions with required fields near both the beginning and
  end, including the file-backed delivery preflight.

## Capability

### Modified capability

- `factory-operations`: local and hosted PR metadata validation is
  deterministic for every body inside the existing 64 KiB boundary.

## Non-goals

- No metadata requirement, input bound, contribution rule, workflow, crate,
  dependency, toolchain, target, product API or release behavior changes.
- No acceptance of malformed template placeholders or arbitrary prose.

## Delivery

Issue #374 owns this factory-only correction. It targets protected `develop`
and requires the normal exact-head fast gate.
