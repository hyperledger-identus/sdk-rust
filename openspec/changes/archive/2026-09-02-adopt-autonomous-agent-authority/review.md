# Local review

## Scope reviewed

- standing authority for routine backlog selection, specification and delivery;
- protected product, governance, release, disclosure, secret and repository
  boundaries;
- the `contract-ready` state and semantic review requirement;
- consistency across governance, agent, contributor, factory and GitHub
  templates;
- repository isolation, issue linkage, local review and green-CI merge gates.

## Findings

### Resolved: delegation ADR was not required agent context

The root agent instructions required ADR 0001 but did not direct agents to the
operational delegation decisions in ADRs 0003 and 0004. Both are now required
reading before repository work, so clients receive the same stop/go policy.

### Resolved: roadmap used scope-acceptance language for placeholders

The roadmap correctly rejected inherited placeholder crates as commitments but
described them as lacking "accepted scope." That phrase could be mistaken for a
universal human approval state. It now says placeholders do not enter the
roadmap automatically; agents can crystallize them through the ordinary issue
and contract path when roadmap priority reaches them.

### Resolved by finalization: current capability specs retained the old gate

The active change correctly modifies the factory and spec-driven requirements,
while the current capability specs retain the prior text until OpenSpec archive
sync. Finalization must archive the change before publication so no active
source of truth retains `contract-approved` or human scope-acceptance language.

## Result

Passed with no unresolved blocker. The change preserves issues, semantic and
distinct local review, specialist review where risk requires it, signatures,
DCO, CI, protected pull requests and repository isolation. It grants no secret,
disclosure, publishing, release, live-settings, downstream or `main` authority.
