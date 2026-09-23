# factory-operations

## ADDED Requirements

### Requirement: Hosted policy distinguishes platform synchronization from authored commits

Hosted contribution provenance SHALL treat a merge commit as platform
synchronization metadata only when its GitHub-verified signature, GitHub
committer identity, canonical base-into-head subject, two-parent graph, exact
reproduced merge tree, prior pull-request head and protected-base ancestry all
match the pull-request event.
For that record only, hosted validation SHALL omit Conventional Commit subject
and DCO trailer checks while continuing to enforce accepted signature
provenance. Every ordinary, malformed, locally evaluated or incompletely
evidenced commit SHALL retain the authored-commit rules.

#### Scenario: GitHub updates a pull-request branch with the protected base

- **WHEN** GitHub creates a verified canonical synchronization merge whose
  first parent is the preceding pull-request commit and whose second parent is
  the current protected base or its ancestor
- **THEN** hosted provenance accepts the platform subject and absent DCO
  trailer while still validating its signature

#### Scenario: A merge resembles synchronization but lacks structural evidence

- **WHEN** a two-parent commit has an unexpected parent, subject, committer,
  signature state, conflicting or mismatched merge tree, or incomplete
  pull-request context
- **THEN** it receives no exemption and must satisfy every authored-commit rule

#### Scenario: A local merge lacks hosted pull-request context

- **WHEN** local range validation encounters a merge commit
- **THEN** it applies the ordinary subject, DCO and signature rules without
  inferring a GitHub synchronization exemption
