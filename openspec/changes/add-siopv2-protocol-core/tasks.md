# Tasks

## 1. Research and preflight

- [x] 1.1 Pin SIOPv2 draft 13 and record its non-Final maturity.
- [x] 1.2 Inspect Oxid revision and classify reusable versus product behavior.
- [x] 1.3 Assess `siopv2`, `affinidi-siopv2`, `siop` and existing SDK crates.
- [x] 1.4 Decide shared retrieval, DID subject, JOSE, limits and lifecycle boundaries.
- [ ] 1.5 Pass research, constraints, strict validation and issue #514 preflight.

## 2. Shared OpenID request foundation

- [ ] 2.1 Activate bounded request retrieval in `identus-openid4vc`.
- [ ] 2.2 Delegate OID4VP retrieval while preserving public names and behavior.
- [ ] 2.3 Add shared boundary, redaction and regression tests.

## 3. SIOPv2 protocol core

- [ ] 3.1 Add the unpublished `identus-siopv2` crate and finite limits/errors.
- [ ] 3.2 Add by-reference invocation, retrieval binding and signed request verification.
- [ ] 3.3 Validate the exact draft-13 DID/direct_post profile and replay evidence.
- [ ] 3.4 Construct bounded self-issued ID Token signing input and exact form response.
- [ ] 3.5 Add deterministic approval/refusal/cancellation/timeout/delivery transitions.

## 4. Knowledge and quality evidence

- [ ] 4.1 Add architecture guide, diagrams, API examples and Oxid migration note.
- [ ] 4.2 Add clean-room/oracle vectors plus E2E, negative and property tests.
- [ ] 4.3 Add fuzz, benchmark and differential evidence with explicit limits.
- [ ] 4.4 Update backlog ownership, inventories, blueprint and source matrix.
- [ ] 4.5 Run focused/workspace, MSRV, docs, Nix, dependency and portable gates.

## 5. Review and delivery

- [ ] 5.1 Run Claude CLI exact-diff review or Codex CLI fallback.
- [ ] 5.2 Resolve blocker/high/medium findings and rerun proportionate gates.
- [ ] 5.3 Archive OpenSpec, publish metrics, merge to `develop`, and clean the worktree.
