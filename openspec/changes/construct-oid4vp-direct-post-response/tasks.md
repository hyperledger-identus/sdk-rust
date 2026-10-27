# Tasks

## 1. Evidence and preflight

- [x] 1.1 Pin Final response and state semantics and audit the existing crate.
- [x] 1.2 Decide dependency, effect, privacy, limit and rollback boundaries.
- [x] 1.3 Pass readiness, strict validation and exact issue #487 preflight.

## 2. Request and DCQL evidence

- [x] 2.1 Retain bounded optional state without exposing request bytes.
- [x] 2.2 Retain private query-id, credential-set and cardinality response rules.
- [x] 2.3 Preserve existing request/JAR/DCQL behavior and public compatibility.

## 3. Response construction

- [x] 3.1 Add bounded redacted presentation and query-result SDK values.
- [x] 3.2 Validate exact query coverage, credential-set options and cardinality.
- [x] 3.3 Serialize deterministic Final `vp_token` and form body.
- [x] 3.4 Return an inert target/media-type/body envelope with sensitive accessors.

## 4. Evidence

- [x] 4.1 Add official, negative, boundary, redaction and contract vectors.
- [x] 4.2 Add property, fuzz, benchmark and dependency evidence.
- [x] 4.3 Run focused/workspace, MSRV, rustdoc, native and portable target gates.
- [x] 4.4 Activate `oid4vp.direct-post-response.quality.v1` and ledger mappings.

## 5. Review and delivery

- [x] 5.1 Run exact-head Claude CLI review or immediate Codex CLI fallback.
- [x] 5.2 Resolve actionable blocker/high/medium findings and rerun gates.
- [ ] 5.3 Archive OpenSpec, publish metrics and deliver the issue-linked PR.

## 6. Stop boundary

- [x] 6.1 Keep HTTP, protected responses, presentation generation, credential
      verification, trust, consent, replay, persistence and consumer mutation out.
