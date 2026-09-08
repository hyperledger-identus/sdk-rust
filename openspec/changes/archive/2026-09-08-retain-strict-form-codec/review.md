# Exact-diff architecture and security review

- **Reviewer:** Codex fresh specification, source and evidence pass
- **Date:** 2026-09-08
- **Base:** `ce45c375b0682f2226b42d83e98991101409aed7`
- **Reviewed head:** `923e9b698b9cdb582f1745974bc508380c3f5f3a`
- **Scope:** issue #158, ADR 0083, dependency research/ledger and the
  fail-closed parser-equivalence factory requirement
- **Result:** no unresolved blocker

## Findings

1. **Verified - parser semantics are not equivalent.** Exact packaged source
   shows that `form_urlencoded 1.2.2` preserves malformed percent escapes via
   `percent-encoding` and calls `decode_utf8_lossy`. The current SDK rejects
   malformed escapes, raw non-ASCII, decoded invalid UTF-8, decoded NUL and
   decoded-size exhaustion.
2. **Verified - valid serializer behavior alone has insufficient payoff.** The
   candidate matches the assessed literal set, plus-for-space and uppercase
   escape behavior, but checked sizing, deterministic field order, zeroizing
   allocation and the strict decoder would remain local. A dependency would
   delete only a small encoding loop.
3. **Verified - security posture does not regress.** No dependency or unsafe
   surface is added. Secret-bearing bodies retain `Zeroizing<String>`, errors
   remain static and redacted, and existing resource ceilings remain active.
4. **Verified - API and wire compatibility are untouched.** Cargo manifests,
   `Cargo.lock` and `crates/` are byte-identical to the base. The change alters
   specifications, ADRs and research decisions only.
5. **Verified - provenance is artifact-specific.** The assessment records the
   crates.io checksum, dirty VCS metadata, full recorded revision, matching
   packaged/source-file digest, license, feature surface, two-package cone and
   Rust 1.51 declaration. The published crate remains authoritative.
6. **Verified - the factory rule generalizes safely.** Parser substitution now
   requires rejected-input and resource-bound equivalence, avoiding the common
   mistake of treating happy-path format support as a security-compatible
   replacement.
7. **Verified - historical evidence remains auditable.** The archived original
   portfolio retains its then-current decision, while ADR 0083 and living
   research explicitly supersede it rather than rewriting history.

## Residuals

8. The SDK continues maintaining a small bespoke form codec. Its current
   positive, malformed-input, capacity, ordering and redaction tests are the
   control; a future strict candidate or measured multi-consumer payoff is the
   explicit reconsideration trigger.
9. The candidate was source-reviewed and may be used as a valid-serialization
   oracle, but no target build is claimed because it is deliberately absent
   from the production dependency graph.
10. Hosted Linux `fast` and hosted Codex review remain required before merge.

## Decision

The evidence supports retaining the stricter local boundary and rejecting this
candidate for production use today. The change is approved for guarded
OpenSpec archive and an issue-linked pull request to `develop`.
