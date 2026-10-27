# Tasks

## 1. Research and preflight

- [x] 1.1 Pin SIOPv2 draft 13 and record its non-Final maturity.
- [x] 1.2 Inspect Oxid revision and classify reusable versus product behavior.
- [x] 1.3 Assess `siopv2`, `affinidi-siopv2`, `siop` and existing SDK crates.
- [x] 1.4 Decide shared retrieval, DID subject, JOSE, limits and lifecycle boundaries.
- [x] 1.5 Pass research, constraints, strict validation and issue #514 preflight.

## 2. Shared OpenID request foundation

- [x] 2.1 Activate bounded request retrieval in `identus-openid4vc`.
- [x] 2.2 Delegate OID4VP retrieval while preserving public names and behavior.
- [x] 2.3 Add shared boundary, redaction and regression tests.

## 3. SIOPv2 protocol core

- [x] 3.1 Add the unpublished `identus-siopv2` crate and finite limits/errors.
- [x] 3.2 Add caller-parsed by-reference retrieval binding and signed request verification.
- [x] 3.3 Validate the exact draft-13 DID/direct_post profile and replay evidence.
- [x] 3.4 Construct bounded self-issued ID Token signing input and exact form response.
- [x] 3.5 Add deterministic approval/refusal/cancellation/timeout/delivery transitions.

## 4. Knowledge and quality evidence

- [x] 4.1 Add architecture guide, diagrams, API examples and Oxid migration note.
- [x] 4.2 Add an executable clean-room normative vector plus E2E, negative and property tests.
- [x] 4.3 Add fuzz, benchmark and differential evidence with explicit limits.
- [x] 4.4 Update backlog ownership, inventories, blueprint and source matrix.
- [x] 4.5 Run focused/workspace, MSRV, docs, Nix, dependency and portable gates.

## 5. Review and delivery

- [x] 5.1 Run Claude CLI exact-diff review or Codex CLI fallback.
- [x] 5.2 Resolve blocker/high/medium findings and rerun proportionate gates.
- [x] 5.3 Archive OpenSpec, publish metrics, merge to `develop`, and clean the worktree.
