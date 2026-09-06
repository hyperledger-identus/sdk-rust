# Tasks

## 1. Contract and preflight

- [x] 1.1 Validate and semantically review the issue #102 proposal, modified
  requirements, algorithm, threats, limits, compatibility and rollback.
- [x] 1.2 Add the offline modified-requirement preservation checker with
  bounded parsing, rename resolution and exact intent acknowledgements.

## 2. Factory integration and guidance

- [x] 2.1 Run the checker from the structural factory contract and require its
  executable/test assets.
- [x] 2.2 Add `scripts/factory archive <change>` with readiness, scoped
  preflight, pinned OpenSpec archive and post-archive validation.
- [x] 2.3 Document the complete-block authoring rule, intent sidecar and guarded
  archive command in contributor and factory guidance.

## 3. Evidence

- [x] 3.1 Add deterministic temporary-tree fixtures for partial loss, complete
  additive replacement, exact and stale intent, duplicate/unused intent,
  rename plus modification, and new capability success.
- [x] 3.2 Run focused mutation tests, shell/Python/TOML/text hygiene, strict
  OpenSpec validation, factory ready/receipt and the full Nix gate.
- [x] 3.3 Complete and record a distinct local review with compatibility,
  security, provenance, documentation and rollback findings resolved.
- [x] 3.4 Synchronize the canonical capabilities, archive through the guarded
  facade, and prepare the signed/DCO issue-linked PR to `develop`.
