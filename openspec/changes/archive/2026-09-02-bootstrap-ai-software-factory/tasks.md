## 1. Factory Contract

- [x] 1.1 Create the proposal, design and delta specifications for the
  `spec-driven-delivery` and `ai-software-factory` capabilities.
- [x] 1.2 Run strict OpenSpec validation and semantic review; clear every
  contract blocker before implementation.

## 2. Portable Automation

- [x] 2.1 Add `scripts/check-factory.sh` and a shell test covering valid,
  missing-artifact and forbidden-local-state fixtures.
- [x] 2.2 Add `scripts/factory` commands for doctor, status, validate, check,
  ready, receipt and help using the pinned OpenSpec runtime.
- [x] 2.3 Expose the factory through a Nix app, a `factory-contract` flake check
  and documented `just` aliases.

## 3. Human and Agent Interfaces

- [x] 3.1 Add the factory handbook and update README, AGENTS, CONTRIBUTING and
  agentic-SDLC guidance with the executable lifecycle and authority gates.
- [x] 3.2 Add GitHub issue forms and extend the pull-request template with the
  OpenSpec change, readiness and evidence contract.
- [x] 3.3 Replace the hard-coded Pi AFK flow with change-parameterized adapters
  and document the client-adapter policy.

## 4. CI and Repository Controls

- [x] 4.1 Add the pinned `factory-contract` GitHub workflow for pull requests
  and pushes to `develop`.
- [x] 4.2 Add factory ownership and the stable status name to reviewable
  repository settings.

## 5. Verification and Finalization

- [x] 5.1 Pass factory structural tests, ShellCheck, strict OpenSpec validation,
  Markdown lint and relative-link checks.
- [x] 5.2 Pass Rust formatting, strict clippy, tests, docs and `nix flake check`,
  recording any environment-specific result exactly.
- [x] 5.3 Record the complete pre-merge verification evidence and confirm that
  `main` and downstream consumer repositories remain untouched.

After every implementation task is complete, the delivery workflow runs
`factory ready` and `factory receipt`, syncs the capability specifications,
archives this change, creates signed and DCO-compliant commits, and opens a
draft pull request targeting `develop`.
