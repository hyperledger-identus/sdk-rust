# Verification

## Acceptance evidence

| Requirement | Evidence |
| --- | --- |
| One required PR line | Workflow triggers remain unchanged; policy mutation tests require exactly `fast`. |
| Fast SLO and optimization trigger | Policy and target-plan output carry 360/480/600-second values; invalid ordering fails. |
| Bounded review/remediation | Root/Pi agents, operations, SDLC and machine policy all require one discovery plus one remediation round with blocking severity escape hatches. |
| Local-first batching | Machine policy, target plan and operator/agent documents carry `local-first-batched`. |
| Slice guidance | Target plans compute changed text/binary counts and set `decompositionNoteRequired`; malformed numstat fails. |
| Exact production promotion | Policy and target plan require an unchanged exact SHA and block production promotion, publication and release preparation. |
| No duplicate scheduler | The slow workflow and issue #276 scheduler contract are unchanged. |

## Commands passed

- `scripts/factory research-ready optimize-ci-delivery-lines`
- `scripts/factory constraints-ready optimize-ci-delivery-lines`
- `scripts/factory preflight optimize-ci-delivery-lines --issue 303 --write`
- `scripts/factory check`
- `node --test scripts/tests/factory-operations.mjs` — 23 tests passed
- `jq empty .factory-policy.json .pi/delivery-profiles.json`
- pinned `actionlint` with the single ADR-0111 `cache-mode` compatibility
  ignore — passed
- Nix fast-equivalent aarch64-darwin checks: factory contract, Nix/text/TOML
  lint, Rust fmt, workspace build, Clippy and 739 tests — passed
- `git diff --check`

## Commands not run

- The weekly/manual slow workflow was not dispatched. This governance/tooling
  slice changes no Rust runtime, dependency, platform package, binding or
  release artifact, and the slow line is intentionally not an ordinary PR
  gate. Hosted exact-head `fast`, policy, DCO and hygiene results remain
  required after push.
- Linux hosted execution is pending the issue-linked PR. The local compatible
  Nix graph passed; GitHub remains authoritative for the required Linux status.

## Compatibility and rollback

No public API, wire, persistence, dependency, compiler, platform or consumer
behavior changed. Reverting the policy, target-plan v2 fields/tests and
documentation restores the previous factory semantics without altering the
fast/slow workflows or historical evidence.
