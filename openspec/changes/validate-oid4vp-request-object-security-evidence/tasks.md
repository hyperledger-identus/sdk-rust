# Tasks

## 1. Contract and architecture

- [x] 1.1 Research Final/RFC requirements, reusable crates, existing ports, and dependency impact.
- [x] 1.2 Record constraints, state ownership, compatibility, rollback, ADR, and delta specification.
- [x] 1.3 Pass planning readiness, commit the planning-only contract, and write issue-bound preflight.

## 2. Implementation

- [x] 2.1 Extend bounded claim capture with audience and temporal/JWT-ID claims.
- [x] 2.2 Add policy, clock/replay ports, static errors, and consuming security-evidence state.
- [x] 2.3 Move typed routing successors to security evidence and update public documentation.

## 3. Evidence and closeout

- [x] 3.1 Add clean-room vectors, property, redaction, replay, E2E, fuzz, benchmark, and portable evidence.
- [x] 3.2 Update capability matrix, resource boundaries, quality declaration, change ledger, and knowledge base.
- [ ] 3.3 Pass local/Nix/factory gates and exact-head Claude-or-Codex CLI review.
- [ ] 3.4 Open an issue-linked signed/DCO PR, merge only on green protected CI, publish metrics, reconcile #544/#541, and archive OpenSpec.
