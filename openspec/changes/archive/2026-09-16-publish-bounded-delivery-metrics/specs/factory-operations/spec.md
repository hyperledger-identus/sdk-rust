# factory-operations delta

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

Every completed production-ready work item SHALL retain or confirm its exact
private record before attempting one public allowlisted receipt. Publication
SHALL default to the record's exact PR and SHALL fall back to its issue when no
PR exists. An explicit issue target MAY support issue-centric or historical
reporting. Historical PR-backed publication SHALL verify repository, issue,
PR, and exact hosted head without requiring the caller's checkout to remain on
that head. One bounded retry MAY handle a transient comment failure; persistent
failure SHALL remain visible telemetry debt and SHALL NOT invalidate otherwise
independent product evidence.

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

#### Scenario: Completed PR metrics are published

- **WHEN** a schema-valid record names an authoritative PR whose hosted head
  equals the record head
- **THEN** the factory atomically retains or confirms the private record before
  creating or updating the publisher's unique versioned comment on that PR

#### Scenario: Issue receipt is selected

- **WHEN** a valid record has no PR or the supervisor explicitly selects its
  existing authoritative issue
- **THEN** the same local-before-remote and unique-marker rules publish the
  bounded receipt to that issue

#### Scenario: Historical or local evidence conflicts

- **WHEN** the hosted PR head differs, the issue/PR is unavailable, a private
  issue/head record differs, or multiple owned target comments share a marker
- **THEN** publication fails before an ambiguous or forged public receipt is
  created

#### Scenario: Public mutation remains unavailable

- **WHEN** comment creation or update still fails after one bounded retry
- **THEN** the private record remains authoritative and the supervisor reports
  visible telemetry debt without treating independent product evidence as red
