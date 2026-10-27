# Tasks

## 1. Evidence and preflight

- [x] 1.1 Pin Final response and state semantics and audit the existing crate.
- [x] 1.2 Decide dependency, effect, privacy, limit and rollback boundaries.
- [ ] 1.3 Pass readiness, strict validation and exact issue #487 preflight.

## 2. Request and DCQL evidence

- [ ] 2.1 Retain bounded optional state without exposing request bytes.
- [ ] 2.2 Retain private query-id, credential-set and cardinality response rules.
- [ ] 2.3 Preserve existing request/JAR/DCQL behavior and public compatibility.

## 3. Response construction

- [ ] 3.1 Add bounded redacted presentation and query-result SDK values.
- [ ] 3.2 Validate exact query coverage, credential-set options and cardinality.
- [ ] 3.3 Serialize deterministic Final `vp_token` and form body.
- [ ] 3.4 Return an inert target/media-type/body envelope with sensitive accessors.

## 4. Evidence

- [ ] 4.1 Add official, negative, boundary, redaction and contract vectors.
- [ ] 4.2 Add property, fuzz, benchmark and dependency evidence.
- [ ] 4.3 Run focused/workspace, MSRV, rustdoc, native and portable target gates.
- [ ] 4.4 Activate `oid4vp.direct-post-response.quality.v1` and ledger mappings.

## 5. Review and delivery

- [ ] 5.1 Run exact-head Claude CLI review or immediate Codex CLI fallback.
- [ ] 5.2 Resolve actionable blocker/high/medium findings and rerun gates.
- [ ] 5.3 Archive OpenSpec, publish metrics and deliver the issue-linked PR.

## 6. Stop boundary

- [ ] 6.1 Keep HTTP, protected responses, presentation generation, credential
      verification, trust, consent, replay, persistence and consumer mutation out.
