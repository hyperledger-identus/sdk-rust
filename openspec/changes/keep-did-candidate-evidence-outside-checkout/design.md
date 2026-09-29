# Design

## External evidence root

The DID matrix job writes primary and MSRV results to
`$RUNNER_TEMP/did-matrix/{primary,msrv}`. GitHub's artifact action uploads
`${{ runner.temp }}/did-matrix` under the unchanged artifact name. The builder
continues to require an absolute new output path and emits its current bounded
lane receipt without schema changes.

## Observable clean-source boundary

After primary qualification and before MSRV qualification, a dedicated step
checks Git porcelain status and fails on any tracked, staged, or untracked
checkout mutation. This makes the orchestration invariant visible in the run
instead of relying only on the MSRV builder to diagnose it.

## Offline enforcement

The release-candidate checker requires both runner-temporary output paths, the
runner-temporary upload path, and the explicit cleanliness step. It rejects the
old checkout-local output prefix. Mutation tests independently remove or
replace these workflow elements and require bounded diagnostics.

## Verification and rollback

Focused checker/mutation tests and workflow lint prove policy shape. Factory,
OpenSpec, Nix, review, and required CI prove repository integration. A natural
or authorized slow run later proves hosted behavior. Rollback removes the four
path/assertion checks together and therefore knowingly restores the failure.
