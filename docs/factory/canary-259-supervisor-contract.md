# Issue #259 supervisor-to-Pi canary

## Immutable boundary

- Delivery base: `d619fb2e092a2f53014aeb3cee738e6d200ac482`.
- Planning and preflight head: `d43ee1f2f291b3cc24e08c2e76fd4640fb2888f0`.
- Branch: `codex/feat/issue-259`.
- OpenSpec change: `define-supervisor-pi-run-contract`.
- Worker role/task: `quality` / `3.3`.
- Remote mutation, commit, merge, release, publication, cleanup and downstream
  mutation remained supervisor-owned and were not exercised by the worker.

## Implementation-worker observation

The initial bounded Pi implementation session ran through `./bootstrap.sh
--pi` for 2,433.66 seconds. Peak RSS was 385,040,384 bytes. Harvesting its
private persisted artifacts produced exact aggregate evidence:

| Sessions | Turns | Tools | Input | Output | Cache read | Cache write |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 76 | 164 | 213,997 | 81,889 | 11,100,416 | 0 |

The session remained private. No prompt, transcript, runtime identifier,
provider/model, cost or raw output is recorded here.

## Canary iterations

The first real wrapper canary proved envelope, heartbeat, bootstrap launch,
unchanged Git effects and handoff acceptance, then exposed one privacy-contract
defect: Pi created its session JSONL as mode `0644`. Its enclosing run
directory was owner-only, so the file was not traversable by other users, but
the collector correctly rejected it rather than weakening the file rule.

The wrapper was changed to harden the completed session tree to `0700/0600`
and to reject symlinks or non-regular entries. A fresh final canary then passed:

| Evidence | Result |
| --- | --- |
| External heartbeat | terminal `exited`, 81 seconds, exit code 0, no active process |
| Handoff | schema-valid and accepted; exact identity and 22 current changed paths matched |
| Remote authority defense | GitHub tokens absent, private empty `gh` config, loopback-only origin push URL |
| Repository effect | no canary-created repository change |
| Session permissions | regular `0600` file below owner-only run directories |
| Usage harvest | 1 session, 4 turns, 4 tools |
| Tokens | 14,497 input, 3,440 output, 6,528 cache read, 0 cache write |
| Repeat harvest | equal and idempotent |

## Local evidence and review

- `node --test scripts/tests/factory-operations.mjs`: 21/21 passed, including
  stale identity, undeclared task, path escape, unsafe next action, lingering
  process, timeout, session permissions, duplicate/oversized/symlinked JSON,
  token de-duplication and metrics v1/v2 cases.
- `./scripts/factory check`: passed strict factory/OpenSpec structure.
- `./bootstrap.sh --check`: passed structural, effective runtime and 21/21
  operational tests in 3.66 seconds; measured command max RSS was 62,668,800
  bytes.
- ShellCheck and Node syntax checks passed for changed executable surfaces.
- The supervisor's distinct review found and corrected the contaminated JSON
  stream, undeclared-task acceptance, incomplete phase telemetry, additive
  parallel-CI accounting, session mode and remote-authority gaps before
  closeout.

## Limitations

- Tool and environment controls prevent normal accidental remote mutation but
  do not constitute hostile-process or kernel isolation.
- Process-group observation cannot prove that deliberately daemonized work
  escaped into a different session; a hostile-code runner needs OS isolation.
- Pi session format 3 is the only accepted usage source. Another major is
  explicit unavailable evidence until researched.
- Hosted CI timings, PR identity and final disposition are added by the
  supervisor after publication; they are not worker claims.
