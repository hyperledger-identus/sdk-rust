# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/285
Constraint blockers: none

## Existing entries affected

- `SDK-AGENT-001` and `SDK-AGENT-002`: autonomous work selection gains a
  read-only freshness gate while bounded execution and human-protected actions
  remain unchanged.
- `SDK-LIM-006`: release, publication, repository settings and `main` remain
  inactive.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005`: Rust 1.98.1 and the
  target matrix are unchanged.

## Introduced or changed constraints

No SDK consumer constraint changes. Repository operations now require an
explicit live issue-state check before treating an `in_progress` roadmap row as
selectable autonomous work.

## Introduced or changed limitations

The live audit requires GitHub CLI access and sufficient public-repository
visibility. It is intentionally not an offline or required-PR-CI guarantee.
Issue state proves coordination freshness only; it does not prove specification
quality, implementation completeness, security, conformance or priority.

## Consumer and product impact

Rust APIs, wire behavior, dependencies, compiler floor, target claims and
consumer repositories are unchanged. Maintainers and supervisors receive an
actionable failure before delegating work against a closed active owner.

## Activation and rollback

The behavior activates when issue #285 merges into `develop`. Rollback removes
the opt-in command and restores the two CSV references; it does not migrate
runtime data or GitHub settings. `main` remains reserved.

## Evidence

Hermetic fixtures prove state semantics and error handling. A final read-only
live invocation proves current issue visibility and open active ownership.
