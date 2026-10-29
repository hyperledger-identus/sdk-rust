# factory-operations delta

## ADDED Requirements

### Requirement: Immutable planning evidence survives protected squash integration

The factory SHALL validate immutable error-golden planning evidence across the
repository's protected squash-merge path without requiring the source feature
branch or discarded planning commit to remain in current ancestry. Every
binding SHALL pin the complete preimplementation receipt bytes and exact golden
bytes. Active or ancestry-preserving evidence SHALL retain the exact
base-to-contract-to-head and planning-only-diff proof. An archived
squash-integrated contract MAY use canonical snapshot evidence only when the
fixed base remains in current ancestry, the complete receipt is exact, the
receipt plus archived and stable goldens resolve byte-for-byte from `HEAD`, the
archived and stable goldens are exact, and any locally reachable named contract
object passes the full base, planning-path, and golden-blob validation.

The fallback SHALL NOT claim that the behavior-bearing squash commit was a
planning-only commit and SHALL NOT depend on a network fetch, remote branch,
tag, private merge receipt, or repository-setting change.

#### Scenario: ancestry-preserving contract is checked

- **WHEN** the named planning commit descends from its fixed base and is an
  ancestor of current `HEAD`
- **THEN** the factory requires its exact receipt, planning-only diff, contract
  golden blob, archived or active planning golden, stable golden, and fixed
  hashes to agree

#### Scenario: protected squash archive is checked

- **WHEN** canonical history contains the exact archived receipt and golden but
  the named planning commit is not an ancestor of current `HEAD`
- **THEN** the factory accepts only an archived change whose fixed base is an
  ancestor of `HEAD`, whose receipt and golden bytes resolve identically from
  `HEAD` and match their immutable hashes, and whose reachable contract object,
  if present, validates fully

#### Scenario: source planning object was pruned

- **WHEN** the protected squash archive is exact and the source planning commit
  is no longer locally reachable after branch cleanup
- **THEN** the factory validates the canonical snapshot without a network or
  external-ref dependency

#### Scenario: active evidence loses ancestry

- **WHEN** an active change's planning commit is not an ancestor of current
  `HEAD`
- **THEN** the factory rejects it and does not apply the archived-snapshot path

#### Scenario: exact evidence exists only in the working tree

- **WHEN** exact receipt and golden files are copied into an unrelated or
  uncommitted working tree but the same paths and bytes do not exist in `HEAD`
- **THEN** the factory rejects them as non-canonical evidence

#### Scenario: immutable receipt or reachable history is forged

- **WHEN** any receipt byte, fixed identity, golden byte, base ancestry,
  planning path, or reachable contract blob differs
- **THEN** the factory fails closed and identifies the invalid evidence class
