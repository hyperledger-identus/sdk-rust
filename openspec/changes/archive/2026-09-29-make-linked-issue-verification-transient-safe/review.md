# Local review

Review date: 2026-09-29
Review scope: exact diff from protected base `90f1c5f0dd51ca4d49615acbb2a4e85a1a079c7e`
Review result: passed

## Architecture and cohesion

The workflow now delegates one responsibility to one repository-owned script:
establishing that the referenced number is an exact issue in this repository.
Retry classification, attempt state, parsing, and diagnostics are cohesive and
do not leak into the workflow or product crates. The existing metadata checker
continues to own extraction of the numeric reference.

## Security and failure review

- Privileged `pull_request_target` still checks out and executes only exact
  protected-base code before using the verifier.
- Repository and issue inputs are syntax-validated; none is evaluated as code.
- The workflow token is inherited by `gh` and is never rendered.
- Provider output is parsed as JSON from a private temporary directory and
  removed on exit.
- Retry is restricted to 408, 425, 429, 499, 5xx, and explicit transport
  interruption text; permanent, malformed, missing, pull-request, and mismatch
  cases remain immediate failures.
- Three attempts and a maximum ten-second injectable delay bound resource use;
  production defaults add at most four seconds.

## Maintainability and portability review

The script uses Bash features already required by repository automation and
commands already present on GitHub runners. The fake-provider test has no live
network dependency and asserts exact attempt counts. `shellcheck`, `actionlint`,
the complete factory contract, and a live read-only lookup of issue #474 pass.

## Findings

No blocking or follow-up finding remains. A broader reusable GitHub API retry
layer would be premature without another evidence-backed consumer.
