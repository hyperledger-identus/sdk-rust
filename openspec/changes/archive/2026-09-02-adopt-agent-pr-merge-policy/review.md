# Local review

## Scope reviewed

- delegated feature-branch push and eligible `develop` merge authority;
- mandatory issue linkage and local review evidence;
- protected human authority and repository-isolation boundaries;
- PR-policy shell logic, workflow permissions and injection surface;
- consistency across governance, contributor, agent and factory documents.

## Findings

### Resolved: primary governance still prohibited agent merge

The first implementation updated the agentic SDLC but left `GOVERNANCE.md` and
the architecture blueprint with the old human-only merge and draft-PR language.
Those controlling documents now distinguish protected human decisions from the
delegated operational merge into `develop`.

### Resolved: a pull request number could satisfy issue existence

GitHub's issue CLI accepts both issues and pull requests. The workflow now
compares the resolved object URL with the exact repository `/issues/<number>`
URL, so a pull request number is rejected even when it exists.

### Resolved: untracked policy scripts were absent from the Nix source

The first local flake attempt ran before new files were staged, so Nix's
Git-filtered source omitted `scripts/tests/pr-policy.sh`. Staging the complete
candidate made the hermetic factory derivation include and pass the new tests.

## Result

Passed with no unresolved blocker. The review found no release, `main`, secret,
security-disclosure, live repository-setting or downstream mutation in scope.
Pull-request body text is passed as environment data, not interpolated into a
shell program. The GitHub workflow uses only read permissions.
