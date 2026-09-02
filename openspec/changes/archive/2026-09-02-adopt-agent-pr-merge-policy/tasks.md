## 1. Policy Contract

- [x] 1.1 Add the proposal, design and delta specifications for issue-linked,
  CI-gated agent delivery to `develop`.
- [x] 1.2 Record the delegated authority and protected boundaries in ADR 0003.

## 2. Repository Guidance

- [x] 2.1 Align AGENTS, CONTRIBUTING, the factory handbook and agentic SDLC
  with the issue-first, local-review-first delivery rule.
- [x] 2.2 Align the desired repository settings, issue intake and pull-request
  evidence template with the new rule.

## 3. Executable Gate

- [x] 3.1 Add a portable pull-request policy checker for the target branch,
  ready state, issue reference and local review evidence.
- [x] 3.2 Add positive and negative shell fixtures and a pinned, read-only
  GitHub Actions workflow exposing `pull-request-policy`.
- [x] 3.3 Include the new policy assets and tests in the factory contract.

## 4. Verification and Finalization

- [x] 4.1 Pass focused shell tests, actionlint, strict OpenSpec validation and
  the repository factory gate.
- [x] 4.2 Pass the proportional Nix and documentation gates and record exact
  evidence plus local review findings.
- [x] 4.3 Sync the capability specifications, archive the completed change and
  deliver it in a signed/DCO PR linked to issue #16.
