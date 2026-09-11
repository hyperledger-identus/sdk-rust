# Design: read-only cache on the fast path

## Decision

Keep the already reviewed and pinned Magic Nix Cache action for restore reads,
but let GitHub enforce a read-only token at the job boundary. Configure the
action itself to use GitHub Actions cache, decline FlakeHub, disable optional
diagnostics and continue on cache errors. Bound the complete job to 20 minutes.

```yaml
jobs:
  fast:
    cache-mode: read
    timeout-minutes: 20
    # ...
    - name: Restore Nix cache (read-only, best effort)
      continue-on-error: true
      uses: DeterminateSystems/magic-nix-cache-action@908b...
      with:
        use-gha-cache: enabled
        use-flakehub: disabled
        diagnostic-endpoint: ""
```

## Why two controls are necessary

`cache-mode: read` is the authority boundary: GitHub issues a token that can
restore but cannot save. Explicit action inputs avoid opportunistic FlakeHub
selection and optional diagnostic traffic. `continue-on-error` makes cache
availability non-normative. The job timeout is the last-resort liveness bound;
it does not turn a timed-out check green.

## Gate invariance

The workflow continues to request exactly the existing factory-contract,
lint-nix, lint-text, lint-toml, rust-fmt, rust-build, rust-clippy and rust-test
Nix attributes. Rust/Nix pins and the flake lock do not change. The support
policy checker parses the workflow and enforces both this set and the cache
contract so a formatting-only edit cannot silently restore cache writes.

The latest released `actionlint` predates this GitHub syntax and reports the
job-level key as unknown. Its invocation ignores only that exact diagnostic.
The support-policy checker compensates by requiring exactly one occurrence,
with value `read`, under the fast workflow; any second workflow occurrence is
rejected. All other `actionlint` diagnostics remain active.

## Failure behavior

- Hit: restore accelerates Nix; save is denied and skipped.
- Miss/eviction/rate limit: logs remain visible; Nix realizes the same
  derivations from signed substituters or source.
- Cache action error: `continue-on-error` lets substantive gates decide the
  job result.
- Hung runner/action: the 20-minute job timeout produces a failing required
  check, preventing merge rather than waiting indefinitely.

## Alternatives

Removing the action is the immediate fallback if the exact-head canary shows a
post phase over 30 seconds. A replacement cache action and authenticated
FlakeHub were rejected for this slice because they enlarge the dependency,
service, credential or cost boundary without first proving necessity.

## Observability and review

GitHub job/step timestamps are the measurement source. The issue receives the
three-run baseline and candidate result; discussion #256 receives the decision
and before/after summary. Three comparable post-change runs determine the
short-term median target. Revisit with ADR 0081 no later than 2026-12-08.
