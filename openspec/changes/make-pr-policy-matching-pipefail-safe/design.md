# Design

## Decision

Feed the bounded shell variable directly to each existing grep expression by
here-string. This keeps grep as the single matcher, avoids an independently
failing producer process under `pipefail`, and retains issue-line capture for
numeric extraction.

## Tests

Extend `scripts/tests/pr-policy.sh` with bodies larger than a typical pipe
buffer. One valid body places all required fields before the filler; another
places them after it. Both must pass and return the same issue. Existing
negative cases preserve strictness.

Exercise the long-body case through `scripts/factory delivery pr-preflight` in
the factory contract so the safe-file and 64 KiB boundary remain integrated.

## Error and privacy behavior

Diagnostics remain static and do not echo the PR body. Inputs are not written
to disk by the checker. The delivery facade continues to own file validation,
size bounds and GitHub-facing metadata composition.
