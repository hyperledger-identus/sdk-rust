## MODIFIED Requirements

### Requirement: Delivery metrics are private, bounded and exact-head

One authoritative metrics record SHALL be keyed by repository, issue,
optional PR and exact head in a private Git-common-dir store. Public output
SHALL contain allowlisted aggregates and exactly one bounded hidden payload,
and publication SHALL update only the authenticated publisher's matching
comment. Unknown counters SHALL be `null`, never estimated as zero.

The factory SHALL continue to validate and render closed version 1 records.
New version 2 records SHALL additionally distinguish total and phase duration,
CI queue and execution duration, failed/canceled/retry and post-CI-push counts,
sessions, turns, tools, non-overlapping input/output/cache-read/cache-write
tokens, and separate worktree/target/cache peaks. Every unavailable v2 value
SHALL have an enum-bounded reason.

#### Scenario: Valid exact-head closeout is rendered

- **WHEN** a schema-valid private record matches the current repository,
  issue, PR and head
- **THEN** the renderer emits a bounded summary without prompts, transcripts,
  credentials, raw command output or provider billing data

#### Scenario: Metrics payload is untrusted

- **WHEN** a payload is malformed, oversized, duplicated, stale, forged or
  contains a forbidden field
- **THEN** collection/publication fails closed while product evidence remains
  independently reportable

#### Scenario: Historical version 1 evidence is read

- **WHEN** an existing closed version 1 record is validated or rendered after
  version 2 activates
- **THEN** the factory preserves its original semantics and versioned marker
  without inventing version 2 fields

#### Scenario: Version 2 counter is unavailable

- **WHEN** a phase, CI, runtime or resource value cannot be measured exactly
- **THEN** its value is `null` and the record contains one closed reason for
  that field rather than zero or an estimate

## ADDED Requirements

### Requirement: Supervisor-to-Pi runs are exact, bounded and observable

The factory SHALL prepare a closed invocation envelope that binds repository,
issue, profile, exact `origin/develop` base, branch/head, active OpenSpec
receipt, worker role, task, allowed paths, tool surface, deadline and
post-artifact grace. It SHALL launch the worker only through `./bootstrap.sh
--pi`, emit an external content-free heartbeat, bound the process group, and
keep merge, release, publication, CI and cleanup authority with the supervisor.

#### Scenario: Exact ready work item launches

- **WHEN** the envelope matches the current issue worktree and durable
  preflight receipt
- **THEN** the supervisor launches one Pi worker with the declared tools and
  private run/session locations and independently updates liveness

#### Scenario: Invocation identity is stale

- **WHEN** repository, issue, base, branch, head, change or receipt differs
  from the current worktree
- **THEN** launch fails before Pi starts and does not claim work completion

#### Scenario: Worker exceeds its deadline

- **WHEN** the Pi process remains active past the declared hard deadline
- **THEN** the supervisor terminates its process group after bounded grace and
  records a timed-out terminal heartbeat separately from handoff validity

### Requirement: Worker handoff is machine-readable and effect-checked

The worker SHALL write one closed, byte-bounded terminal handoff with exact
identity and status, changed paths, acceptance evidence, check outcomes and
durations, finding dispositions, process ownership and a closed safe next
action. The supervisor SHALL validate that record and current Git effects
before accepting it.

#### Scenario: Valid worker checkpoint is accepted

- **WHEN** the handoff identity matches the envelope/current head, changed
  paths are allowlisted, checks and findings are structurally valid and no
  worker-owned process remains
- **THEN** the supervisor reports an accepted checkpoint without granting the
  worker CI, merge, release, publication or cleanup authority

#### Scenario: Handoff is malformed or out of scope

- **WHEN** the handoff is absent, oversized, symlinked, stale, malformed,
  declares an unapproved path/task or proposes an unsafe next action
- **THEN** the supervisor records handoff failure distinctly from product-code
  outcome and refuses closeout acceptance

### Requirement: Persisted Pi usage is harvested without session content

The factory SHALL harvest exact session, turn, tool-call and non-overlapping
token counters from supported private persisted Pi session data. It SHALL NOT
retain or emit prompts, message content, identifiers, commands, raw output,
provider/model data, cost, credentials or billing data. Unsupported or unsafe
input SHALL produce explicit unavailable reasons, never guessed counters.

#### Scenario: Supported Pi v3 session is harvested

- **WHEN** one bounded regular Pi version 3 session and its content-free event
  stream contain terminal assistant usage and tool lifecycle events
- **THEN** the collector counts each terminal response/tool once and emits
  only aggregate sessions, turns, tools and input/output/cache token buckets

#### Scenario: Session source is unsafe or unsupported

- **WHEN** the session is malformed, oversized, symlinked, outside the private
  run, uses an unknown major or has incomplete usage
- **THEN** the collector rejects or marks the affected aggregate unavailable
  with a closed reason and does not copy raw content to metrics or logs
