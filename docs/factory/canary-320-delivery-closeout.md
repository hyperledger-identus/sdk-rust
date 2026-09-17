# Issue #320 delivery-closeout canary

## Immutable boundary

- Delivery base: `81593db3418d5b70990ec3c31ae8ddc3a4bc6edf`.
- Planning head: `4c3983b2ff13e1a18adad825c85c8d2563c945e8`.
- Preflight/launch head: `8474e3694eec4d79f63b44ba3f17a2b624ee841b`.
- Branch: `codex/fix/issue-320`.
- OpenSpec change: `harden-factory-delivery-closeout`.
- Worker role/task: `developer` / `2.1`.
- Worker paths were limited to the delivery module, factory facade, focused
  operational tests, and the named task receipt. Remote mutation, commit,
  merge, release, publication, cleanup, and downstream mutation remained
  supervisor-owned.

## Aggregate worker evidence

The bounded Pi session ran for 624 seconds. The worker exited with code 0 and
its exact-identity handoff was accepted. Content-free harvesting produced:

| Sessions | Turns | Tools | Input | Output | Cache read | Cache write |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 27 | 67 | 130,459 | 21,762 | 2,156,288 | 0 |

No prompt, transcript, command, raw output, provider/model field, runtime
identifier, credential, or billing data is retained here. Peak process RSS was
not measured and remains unavailable rather than estimated.

## Canary finding and repair

The worker correctly implemented the bounded PR-preflight task, passed its
focused checks, exited successfully, and produced an accepted handoff. The
outer `scripts/factory supervisor run` shell nevertheless returned code 2
afterward because the worker had modified `scripts/factory`; the returning
shell read the changed facade generation and reported an unmatched quote even
though the final file was syntactically valid.

The facade now uses `exec node` for supervisor dispatch, so the mutable shell
wrapper cannot resume after the Node supervisor finishes. A focused regression
test locks that transition. The final module also clears `GITHUB_OUTPUT` before
calling the hosted shell policy locally, preventing accidental writes to an
ambient workflow output file.

## Delivery-edge evidence

- Both hosted PR metadata validators now run locally from a bounded regular
  body file.
- Protected squash merge uses stdin-backed `gh --body-file -`, exact-head
  matching, required-check verification, and post-merge GitHub signature and
  message verification before immutable private receipt retention.
- A lost merge response can recover the equal receipt without re-merging.
- Superseded PR #316 / replacement PR #319 exercised recoverable closeout:
  only the clean issue-297 worktree was removed; local and remote branch
  `codex/fix/issue-297` both remained at
  `1d3f4295345ad063255d33cd7cce2543537bc49d`.

## Limitations

- The canary does not prove hostile-process isolation or semantic equivalence
  between superseded and replacement code.
- GitHub availability is required for hosted merge and recovery evidence.
- Release, publication, protected settings, `main`, and consumers remain
  outside the worker and this factory slice.
