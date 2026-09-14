# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

The local inventory says `external-action-required`. Live GitHub API evidence
now shows active rulesets for reserved `main` and integration `develop`.

## Normative sources

- `docs/governance/repository-settings.md`
- https://github.com/hyperledger-identus/sdk-rust/issues/26
- GitHub REST repository/ruleset responses retrieved 2026-09-14

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Record active controls plus exact deviation | `adopt` | Truthful and actionable without weakening policy. |
| Continue claiming no controls | `not-adopt` | Would misdirect agents and contradict live state. |
| Bypass enterprise scanning policy | `not-adopt` | Outside repository-admin authority. |

## Compatibility and dependency evidence

Documentation and offline governance inventory only; no dependency, feature,
public API, wire, compiler or target change.

## Security, privacy and maintenance evidence

Private vulnerability reporting and Dependabot security updates are active.
Secret scanning remains disabled because GitHub returned HTTP 422 under an
enterprise policy. The receipt contains no credential or alert content.

## Rejected or deferred candidates

Enterprise secret-scanning activation is deferred to an enterprise owner.

## Open questions and blockers

No blocker to recording the truthful partial activation. The enterprise control
remains a visible follow-up rather than a waived requirement.

## Evidence commands

- `gh api repos/hyperledger-identus/sdk-rust/rules/branches/develop`
- `gh api repos/hyperledger-identus/sdk-rust`
- `gh api repos/hyperledger-identus/sdk-rust/private-vulnerability-reporting`
