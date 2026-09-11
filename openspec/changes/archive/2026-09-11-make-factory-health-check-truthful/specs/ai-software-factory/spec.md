## MODIFIED Requirements

### Requirement: One portable factory entrypoint controls the workflow

The repository SHALL provide `scripts/factory` with `doctor`, `status`,
`validate`, `check`, `ready` and `receipt` commands. The same implementation
SHALL be callable through the pinned Nix environment and documented `just`
aliases without requiring a global OpenSpec installation.

The entrypoint SHALL additionally provide `archive <change>`, which runs
readiness and the scoped modified-requirement preservation preflight before
invoking the pinned non-interactive OpenSpec archive, then validates the
post-archive factory state.

The `audit` command SHALL enter the repository-pinned Nix environment when
invoked outside a Nix shell and SHALL execute directly exactly once when
already inside a Nix shell.

#### Scenario: Contributor is inside the Nix devshell

- **WHEN** the contributor invokes `scripts/factory check` with `openspec` on
  `PATH`
- **THEN** the script uses the pinned binary and validates the repository

#### Scenario: Contributor has Nix but not global OpenSpec

- **WHEN** the contributor invokes `scripts/factory check` outside the devshell
- **THEN** the script re-enters the pinned Nix environment and performs the same
  validation

#### Scenario: Factory help is requested

- **WHEN** the contributor invokes `scripts/factory help`
- **THEN** the command lists every supported operation and its required inputs

#### Scenario: Guarded archive is requested

- **WHEN** a contributor invokes `scripts/factory archive <change>`
- **THEN** readiness and preservation checks pass before OpenSpec mutates the
  canonical specs or moves the change, and the post-archive state is validated

#### Scenario: Lossy archive is requested

- **WHEN** a modified requirement would silently discard canonical content
- **THEN** `scripts/factory archive <change>` exits non-zero before OpenSpec
  changes either the canonical spec or active change directory

#### Scenario: Runtime audit is invoked outside Nix

- **WHEN** a contributor invokes `scripts/factory audit` without an active Nix
  shell and the repository Nix closure is available
- **THEN** the factory enters that pinned environment once and returns the
  effective runtime audit result with all arguments preserved

#### Scenario: Runtime audit is invoked inside Nix

- **WHEN** a contributor invokes `scripts/factory audit` inside a Nix shell
- **THEN** the factory executes the audit implementation directly without
  recursively entering another shell
