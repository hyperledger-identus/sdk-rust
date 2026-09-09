# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Base: f28b88010b46e2dd6c3e9aaec2f087d515399ed6
Implementation head: f61a109
Specification commit: e71040d
Implementation commits: 6499cf8 and f61a109
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `develop@f28b880...f61a109` diff, issue
#237, OpenID4VCI Final/RFC 9396 sources, ADR 0105, public API, custom scanner,
limits/errors, eight focused tests, roadmap ledgers and local verification.

## Findings

1. **State and compatibility — accepted.** The original partial
   `TokenResponseCore::parse` contract is unchanged. Semantic validation is a
   consuming transition, so presence-only and validated states cannot be
   accidentally conflated.
2. **Normative shape — accepted.** A non-empty RFC 9396 array and every
   recognized `openid_credential` entry require the Final configuration and
   non-empty dataset identifier fields. JSON member order is irrelevant.
3. **Extension behavior — accepted.** Unknown fields and unrelated detail
   types are traversed under the original complete-response budgets and
   discarded. Unknown shapes do not gain credential authority.
4. **Resource behavior — accepted.** Independent positive entry/count/string
   limits supplement the existing complete JSON byte, depth and node bounds.
   Exact/one-over and decoded UTF-8 cases pass. A second scan is bounded and
   intentional for an additive proof state.
5. **Ambiguity — accepted.** Decoded duplicate identifiers within or across
   recognized entries fail closed. Review removed a temporary second retained
   copy and now scans the already bounded result, preserving zeroizing least
   authority.
6. **Privacy and errors — accepted.** Tokens, exact JSON, configuration IDs and
   dataset identifiers do not appear in Debug, Display, stable errors or core
   bridges. New errors are fieldless and capability-attributed.
7. **Architecture and supply chain — accepted.** The capability stays in
   `identus-oid4vci`, adds no dependency/feature/unsafe/native/network/storage
   authority and keeps the exact inward dependency cone.
8. **Delivery scope — accepted.** No donor/downstream mutation, publication,
   release, trust or product-policy claim occurs.

## Residual limitations

- The typed state does not match configuration IDs to Issuer Metadata or
  select/build a Credential Request.
- Outer transport allocation occurs before this crate and remains caller-owned.
- Unknown authorization-detail types are counted but not retained.
- Rust target compilation is not runtime/device/product support evidence.

## Review decision

The slice is cohesive, additive, bounded, redaction-safe and independently
reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains before hosted review.
