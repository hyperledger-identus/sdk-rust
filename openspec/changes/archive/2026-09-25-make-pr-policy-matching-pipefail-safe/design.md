# Design

## Decision

Write the bounded shell variable once to an unpredictable mode-0600 temporary
file, register cleanup with an exit trap, and give that seekable file to each
existing grep expression. This keeps grep as the single matcher, avoids an
independently failing producer process under `pipefail`, works across the
pinned macOS and Linux Bash implementations, and retains issue-line capture
for numeric extraction.

## Tests

Extend `scripts/tests/pr-policy.sh` with bodies larger than a typical pipe
buffer. One valid body places all required fields before the filler; another
places them after it. Both must pass and return the same issue. Existing
negative cases preserve strictness.

Exercise the long-body case through `scripts/factory delivery pr-preflight` in
the factory contract so the safe-file and 64 KiB boundary remain integrated.

## Error and privacy behavior

Diagnostics remain static and do not echo the PR body or temporary path. The
private temporary representation is removed on exit. The delivery facade
continues to own file validation, size bounds and GitHub-facing metadata
composition.
