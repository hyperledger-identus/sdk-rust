# Exact-diff architecture and decision review

- **Reviewer:** Codex fresh specification, source and evidence pass
- **Date:** 2026-09-08
- **Base:** `61210a8613ccd6afc8ab66460e1f00c12b54f712`
- **Reviewed head:** `48691efc783034ed5646ee258fcbd00bf7072113`
- **Scope:** issue #155, ADR 0082, dependency research/ledger, the
  `Multihash` documentation and the consumer-payoff factory requirement
- **Result:** no unresolved blocker

## Findings

1. **Verified - standards categories are distinct.** The did:key source defines
   a multibase encoding of a multicodec public-key type and raw key bytes. The
   multihash source defines a hash-function code, digest length and digest.
   Living SDK text no longer represents the former as the latter.
2. **Verified - the dependency decision follows actual payoff.** The candidate
   remains technically suitable, with exact provenance retained, but no
   current SDK or inspected consumer uses multihash semantics. The change
   cannot add it to Cargo until a named method proves the requirement.
3. **Verified - the factory rule is reusable and bounded.** The canonical delta
   requires a current capability/consumer, normative behavior and concrete
   replacement payoff while preserving a focused reconsideration route. It
   does not prohibit justified foundational dependencies.
4. **Verified - existing compatibility is untouched.** The only Rust change is
   documentation and a test comment. Constructors, accessors, traits,
   formatting, serde, bytes and tests are unchanged. Cargo manifests and the
   lockfile have no diff.
5. **Verified - policy ownership stays method-specific.** A future integration
   must define allowed codes/digest sizes, capacity, canonical varints,
   representation, errors and migration instead of treating structural
   parsing as algorithm or trust policy.
6. **Verified - provenance is corrected.** The annotated `v0.19.5` tag object
   is `08383e21`, is unsigned and targets release commit `e2044a2e`; the
   published checksum, MIT license, Rust 1.81 declaration and two-package
   minimal cone are retained as dated evidence.
7. **Verified - delivery integrity is clean.** All three reviewed commits carry
   valid GPG signatures and DCO trailers. An invalid first local signature was
   detected before push and the unpublished branch was re-signed completely.

## Residuals

8. The public type keeps its historical `Multihash` name and accepts arbitrary
   bytes. This is explicit existing 0.0.x behavior, not a standards promise.
   Removal, deprecation or validation requires a separate consumer/API issue.
9. The Rust 1.98 compiler change clears #156's historical compiler
   prerequisite for `multibase`, but it does not authorize that separate
   dependency. Issue #156 must be refreshed before its own implementation.
10. Nix on the local Darwin host emitted a non-fatal fixup-process diagnostic
    after some artifact builds. Every selected derivation returned success and
    hosted Linux `fast` remains required before merge.

## Decision

The change correctly converts a technically good but unused crate from
speculative adoption to a consumer-gated option. It is approved for guarded
OpenSpec archive and an issue-linked PR to `develop`.
