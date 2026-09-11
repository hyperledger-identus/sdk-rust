## Why

The post-milestone retrospective in discussion #256 and the issue #259 canary
show that the pinned Pi runtime works, but its operational handoff is still a
prose convention. A persistent Codex or Claude Desktop supervisor cannot
reliably distinguish progress from a buffered worker, harvest exact usage, or
reject a stale or out-of-scope closeout without an executable contract.

Issue #259 owns the next bounded factory slice: make the supervisor-to-Pi run,
liveness, handoff and metrics boundary machine-readable while preserving Pi as
a replaceable worker and OpenSpec as the implementation authority.

## What changes

- Add a closed supervisor invocation envelope bound to repository, issue,
  profile, exact `origin/develop` base, branch/head, OpenSpec receipt, role,
  task identity, allowed paths, tool surface and deadlines.
- Add a repository wrapper that launches Pi only through `./bootstrap.sh
  --pi`, persists session data privately, emits an external heartbeat, applies
  a process deadline and distinguishes worker exit from handoff validity.
- Add a closed terminal handoff contract and reject stale identity,
  out-of-scope paths, malformed or oversized records and unsafe next actions.
- Harvest exact non-overlapping counters from observed Pi v3 session JSONL
  while discarding prompts, messages, identifiers, cost and raw output.
- Introduce work-item metrics v2 with v1 validation/render compatibility,
  phase and CI durations, attempt history, runtime counters and separate
  worktree/target/cache peaks.
- Add known-good, known-bad and one bounded live canary evidence path.

## Capabilities

### Modified capabilities

- `factory-operations`: defines the supervisor/worker run, liveness, handoff
  and closeout boundary.

## Non-goals

- No Pi, Node, npm, Rust, Nix, package or dependency upgrade.
- No provider, model, authentication, prompt policy or personal session
  configuration change.
- No OS security sandbox claim: implementation workers retain the explicitly
  allowlisted Pi tools required by their role, and the supervisor validates
  repository effects before accepting a handoff.
- No autonomous merge, release, publication, `main`, repository-setting or
  downstream mutation.
- No metrics database, transcript ingestion or provider billing collection.

## Delivery

Issue #259 owns this production-ready factory change from
`develop@d619fb2e092a2f53014aeb3cee738e6d200ac482`. The PR targets `develop`.
Issue #260 and the remaining issue #261 classification remain separate work.
