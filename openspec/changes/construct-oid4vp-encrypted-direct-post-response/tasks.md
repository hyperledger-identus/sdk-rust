# Tasks

## 1. Research and preflight

- [x] 1.1 Correct older-draft JARM assumptions against OID4VP Final.
- [x] 1.2 Qualify `jose-rs`, modular `no-way-jose`, `jwt-simple`, and `josekit`.
- [x] 1.3 Decide the SDK-owned port, limits, redaction and rollback boundaries.
- [x] 1.4 Pass research, constraints, strict validation and issue #535 preflight.

## 2. JOSE facade

- [x] 2.1 Add bounded P-256 recipient, algorithm, request/result and error types.
- [x] 2.2 Add the synchronous encrypter port without a production backend.
- [x] 2.3 Add grammar, redaction, boundary and portable tests.

## 3. OID4VP protected response

- [x] 3.1 Validate bounded Final client metadata and deterministic key selection.
- [x] 3.2 Accept `direct_post.jwt` as a distinct validated mode.
- [x] 3.3 Share DCQL payload construction and invoke the JWE port once.
- [x] 3.4 Emit only the bounded deterministic `response` form parameter.
- [x] 3.5 Preserve plain success/error behavior and all stop boundaries.

## 4. Knowledge and quality evidence

- [x] 4.1 Add ADR, architecture/adapter guide and diagrams.
- [x] 4.2 Add vectors plus E2E, property, fuzz, benchmark and differential routes.
- [x] 4.3 Update backlog owner, change ledger, quality declarations and inventory.
- [x] 4.4 Run focused/workspace, MSRV, docs, Nix, dependency and portable gates.

## 5. Review and delivery

- [x] 5.1 Run Claude CLI exact-diff review or Codex CLI fallback immediately.
- [x] 5.2 Resolve blocker/high/medium findings and rerun proportionate gates.
- [ ] 5.3 Archive OpenSpec, publish metrics and deliver the issue-linked PR.

