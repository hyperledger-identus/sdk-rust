## ADDED Requirements

### Requirement: Pi project packages use an external content-addressed cache

The bootstrap SHALL keep Pi project-package state out of every registered Git
working tree. It SHALL select a repository-private cache from the exact pinned
runtime, project package declarations and resolved package lock, verify
completeness before use and preserve Pi's expected project path without reading
or moving authentication, session, prompt, transcript, provider or model state.

#### Scenario: Two worktrees use the same exact harness

- **WHEN** two trusted managed worktrees launch the same pinned Pi, Node, npm
  and exact project package set
- **THEN** each `.pi/npm` resolves to the same complete external cache identity
  without retaining a duplicate package installation in either worktree

#### Scenario: Harness inputs change

- **WHEN** any pinned runtime or exact project package declaration differs
- **THEN** bootstrap selects a distinct cache identity and does not mutate the
  cache created for the previous inputs

#### Scenario: Worktree package path is unsafe or operator-owned

- **WHEN** `.pi/npm` is a regular directory, an unexpected symlink, or the
  derived store fails canonical path and non-symlink checks
- **THEN** bootstrap exits before Pi starts and does not replace, move, delete
  or follow that data

#### Scenario: Raw Pi bypasses bootstrap

- **WHEN** Pi creates `.pi/npm` directly in a repository working tree
- **THEN** the root ignore fallback prevents generated packages from dirtying
  Git while the next bootstrap reports explicit non-destructive recovery

#### Scenario: Concurrent first initialization

- **WHEN** two worktrees prepare one previously absent cache identity
- **THEN** isolated staging and atomic promotion expose only a verified complete
  canonical store, and incomplete staging is never selected by a Pi launch
