## Context

Issue #175 requests one explicit decision per repository in discussion #174.
ADR 0061 is intentionally broad and should remain the umbrella rule rather
than grow into an unreviewable matrix of repository-specific exceptions.

## Goals / Non-Goals

**Goals:**

- make every repository decision independently discoverable and supersedable;
- pin claims to dated immutable upstream revisions;
- distinguish production dependency permission from reference/oracle use;
- preserve SDK-owned public types and ports; and
- give agents objective activation and reconsideration triggers.

**Non-Goals:**

- add a Cargo dependency or copy source/fixtures;
- activate a conditional roadmap feature;
- rank projects by popularity;
- claim an audit or cross-platform compatibility; or
- change any consumer repository.

## Decisions

### Use one ADR per repository and one shared evidence report

Each ADR remains short and operational while the report avoids repeating the
same evidence vocabulary. ADRs 0066–0077 are reserved for the twelve sources;
0065 remains available for issue #173's CI decision.

### Use a finite disposition vocabulary

`conditional-adopt`, `spike`, `oracle`, and `not-adopt` retain the meanings
already established by the research-readiness contract. There is no
unconditional whole-framework adoption in this set.
Future issues may propose a narrow dependency only through the relevant ADR's
trigger and ADR 0061's facade rule.

### Separate permission from proof

Conditional adoption is permission to run a focused evidence-bearing adapter
change. It is not proof that Rust, platforms, security, conformance, licensing,
or dependency budgets pass. Those claims belong to the integration receipt.

### Preserve immutable provenance

Each upstream is assessed at a full commit SHA. Floating activity/release
facts are dated. Any copied fixture or source in a future PR must separately
record origin path, license, retrieval, transformation, and expected result.

## Risks / Trade-offs

- Twelve ADRs add documentation surface; they also prevent a change to one
  repository from rewriting the whole portfolio.
- Upstream facts age; objective triggers and immutable revisions make refresh
  precise rather than silently stale.
- Conditional decisions can be misread as dependency approval; the capability
  spec explicitly prohibits adoption without a focused issue and proof.
- Oracle implementations can shape internal behavior incorrectly; normative
  standards and conformance suites continue to outrank implementation precedent.

## Migration Plan

1. Merge the documentation-only portfolio.
2. Use #164 for the first dev-only differential harness.
3. Keep #162 as the Askar adapter spike.
4. Create issue-first adoption work only when a component roadmap row and ADR
   trigger both justify it.

Rollback reverts this documentation PR. No runtime migration exists.

## Open Questions

None block the portfolio. Whether and when IDR-050 activates remains a separate
product/backlog decision.
