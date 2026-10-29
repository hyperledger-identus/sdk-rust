# Tasks

## 1. Planning and exact-base contract

- [x] 1.1 Compare the existing JOSE golden lifecycle with the separate JWE enum and choose an independent version axis.
- [x] 1.2 Accept ADR 0195 and complete research, constraints, design, and delta specification.
- [x] 1.3 Capture and independently inspect the exact-base eight-row planning golden.
- [x] 1.4 Pass strict validation and exact issue #539 preimplementation receipt.

## 2. Behavior-neutral implementation

- [x] 2.1 Copy the planning golden byte-for-byte to the stable JOSE fixture path.
- [x] 2.2 Route every JWE error through one private compile-time record with a wildcard-free match.
- [x] 2.3 Add exact golden, order, uniqueness, conversion, source, and redaction regressions.
- [x] 2.4 Extend immutable-golden factory and mutation evidence with the bounded JWE binding.
- [x] 2.5 Update the adopter knowledge page with versioning, maintenance, and evidence routes.

## 3. Verification and delivery

- [ ] 3.1 Run focused JOSE, immutable-golden, factory, workspace, Nix, public API, and dependency evidence.
- [ ] 3.2 Complete an independent Claude or Codex CLI review and resolve actionable findings.
- [ ] 3.3 Archive the complete OpenSpec change, publish metrics, open an issue-linked PR, and monitor protected CI.
