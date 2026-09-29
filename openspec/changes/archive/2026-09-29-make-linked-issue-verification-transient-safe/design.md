# Design

## Decision

Add `scripts/ci/verify-repository-issue.sh` as the single owner of hosted issue
identity verification. It validates repository and numeric issue inputs, calls
`gh api` for the REST issue resource, and parses the bounded JSON response with
`jq`. A valid result must have the exact canonical issue URL and no
`pull_request` member.

The script makes three total attempts. It retries only errors that identify a
transient HTTP status (408, 425, 429, 499, or 5xx) or a bounded set of transport
failures such as timeout, reset connection, temporary DNS failure, EOF, or TLS
handshake interruption. All other command failures stop immediately. Retry
delay is two seconds in CI and injectable as zero for deterministic tests.

## Tests

A shell contract places a fake `gh` first on `PATH` and proves immediate
success, one transient recovery, permanent failure without retry, exhausted
transient failure, pull-request rejection, and URL mismatch. Tests assert
attempt counts and stable diagnostics without live GitHub access.

## Security and operations

The workflow continues to execute only protected-base code under
`pull_request_target`. Untrusted PR content is not executed. Temporary output
and diagnostics use unpredictable files, are removed by trap, and never print
the token. Retry exhaustion remains red and visible.
