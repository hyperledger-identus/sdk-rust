## 1. Research and specification

- [x] 1.1 Create issue #201 under #10 and define scope, bounds, W3C baseline, donor revision, compatibility, tests, rollback and stop conditions.
- [x] 1.2 Inspect the current SDK resolver/result contracts and NeoPRISM donor source/tests; classify every deliberate divergence.
- [x] 1.3 Evaluate negotiation and HTTP candidates, including versions, features, maintenance, license, unsafe surface, coupling and rejection reasons.
- [x] 1.4 Add ADR 0090 and this OpenSpec change before Cargo or Rust implementation.

## 2. Dependency and architecture

- [ ] 2.1 Add exact private workspace dependencies with minimal features; record integrated checksums, cone, advisories, licenses and runtime absence.
- [ ] 2.2 Add `identus-did-resolver-http` to the outer-boundary rulebook and public inventory without adding it to portable target matrices.
- [ ] 2.3 Expose only constants and a state-closed fixed-route constructor over `Arc<dyn DidResolver>`.

## 3. Implementation

- [ ] 3.1 Add bounded repeated-header aggregation, quote-aware media-range counting and strict quality-value preflight.
- [ ] 3.2 Negotiate the three supported representations privately and construct the correct `ResolutionOptions`.
- [ ] 3.3 Convert extractor/DID/query failures and resolver error/deactivation states into redacted W3C envelopes and statuses.
- [ ] 3.4 Project successful document responses only when metadata content type matches the negotiated representation; avoid panic and add `Vary: Accept` everywhere.

## 4. Verification and delivery

- [ ] 4.1 Add deterministic in-memory router tests for all three representations, default/wildcard/quality/specificity/repeated headers, every standard status, malformed/duplicate/oversized negotiation, invalid path/DID/query, metadata mismatch and redaction.
- [ ] 4.2 Pass formatting, focused tests, factory, Rust 1.98, dependency/security and complete Nix gates; record exact counts and intentionally unrun portable checks.
- [ ] 4.3 Perform a distinct exact-diff architecture/security/protocol review with no unresolved blocker and prepare review and verification receipts.
- [ ] 4.4 Synchronize canonical capability and architecture records, archive the change safely and prepare the signed/DCO issue-linked PR; hosted review/CI, merge and issue receipts remain GitHub evidence.
