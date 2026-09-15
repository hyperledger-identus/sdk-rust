## MODIFIED Requirements

### Requirement: Baseline evidence is bound to policy and Git content

The policy SHALL pin the baseline revision, authored-source fingerprint and
canonical report digest. Fast validation SHALL resolve the revision and
recompute source fingerprint, generated exclusions, and line/file populations.
Slow validation SHALL use the pinned analyzer version to regenerate and compare
the entire report, including function counts and signals, when invoked by the
native weekly/manual workflow from protected default `develop` or reproduced
locally. The run SHALL bind requested and actual revision and SHALL NOT promote
numeric attention signals into automatic architecture limits. Report schema
keys SHALL be closed recursively.

#### Scenario: Default branch contains the slow workflow

- **WHEN** protected `develop` is GitHub's default branch and the native slow
  schedule is accepted
- **THEN** the analyzer runs against the exact default-branch SHA and the
  evidence identifies that revision and GitHub run

#### Scenario: Canonical report field is forged

- **WHEN** revision, fingerprint, population, signal or generated-exclusion
  content is changed while retaining canonical JSON
- **THEN** policy, Git-tree, or full regeneration validation fails closed
