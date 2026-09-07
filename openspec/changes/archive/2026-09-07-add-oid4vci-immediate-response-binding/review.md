# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #143, OpenID4VCI Final section 8.3, RFC 9110 media
  grammar, proposal, design, capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — proof count is not necessarily distinct key count.** The state
   enforces only the necessary response-count upper bound available from the
   current opaque proofs and explicitly disclaims unique-key or credential-key
   correlation.
2. **Cleared — non-normative examples must not create Cache-Control policy.**
   Final requires JSON media for unencrypted responses but does not require a
   cache field in section 8.3. The API omits it and leaves stricter product
   policy downstream.
3. **Cleared — immediate and deferred status must remain distinguishable.**
   Exact 200 is accepted, 202 returns the existing unsupported-deferred error
   before body parsing, and other status values fail with a static HTTP error.
4. **Cleared — private grammar reuse could regress Nonce validation.** The
   reviewed helper moves without semantic edits and both the existing Nonce
   suite and new Credential suite are required gates.
5. **Cleared — a request method could overclaim transport provenance.** The
   method accepts caller-supplied effective values, retains none of them, and
   documents that endpoint origin, execution, TLS, DPoP, replay and single-use
   policy remain external.
6. **Cleared — invalid envelopes should not inspect credential content.**
   Status and bounded media validation precede body parsing; errors remain
   fieldless and canary tests cover validation order and diagnostics.
7. **Cleared — legacy Portal wire shape conflicts with Final.** It remains
   incompatible evidence and is neither copied nor accepted. Final normative
   text and the Oxid array shape control this slice.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration or downstream-write boundary is
crossed. Implementation may proceed.

# Exact-diff implementation review

## Review boundary

- Base: `bd0a5a0063ba8f227933c040c5b6960205ba7feb`
  (`origin/develop`).
- Reviewed implementation head:
  `9004e384241688a2cb9aee1021bc540a52e9a008`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including public API, validation order, helper extraction, bounds, errors,
  tests, ADR, blueprint, inventory, backlog replacement and OpenSpec artifacts.
- Confirmed `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` are
  byte-identical to the base.

## Verification evidence

- Focused all-feature and no-default-feature response tests: 8 passed in each
  mode; existing Nonce HTTP regression suite: 10 passed.
- Workspace Clippy with warnings denied, all-feature tests,
  no-default-feature tests and strict rustdoc passed.
- Factory structural and strict OpenSpec validation passed: 41 items.
- Full pinned `nix flake check -L` passed all 27 gates, including Rust 1.85
  MSRV, wasm32, Android ARM64, iOS ARM64, nextest, formatting, docs, text/TOML/
  Nix lint, cargo-deny, cargo-audit and factory checks. The Nix audit derivation
  retained its pre-existing offline yanked-index diagnostics but exited green.
- Oxid remained clean at
  `5ba38b9bbc9326c294b353daaf2a074eca18c22f`. Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with only its pre-existing
  untracked `.pi-subagents/`, `.pi/` and `tmp/` paths.

## Findings

1. **Verified — status classification is exact and fail-fast.** Only 200
   reaches media/body processing; 202 preserves the unsupported-deferred
   distinction and every other `u16` status fails before remote input work.
2. **Verified — HTTP parsing is reused without Nonce drift.** The existing
   private media and Cache-Control implementation moved intact. The new path
   invokes only media validation, while the complete prior Nonce suite passes.
3. **Verified — cardinality claims match available evidence.** Equal or fewer
   credentials pass and excess credentials fail only after bounded core
   parsing. Public names/docs consistently disclaim distinct keys and
   credential-to-key binding.
4. **Verified — untrusted allocations and diagnostics are bounded.** A
   positive independent Content-Type byte maximum precedes grammar and body
   work; existing body limits remain intact; returned state and all new errors
   expose no credential, notification, bearer, proof, header or body content.
5. **Verified — state authority is narrow.** Private construction requires a
   validated JWT request, stores only its non-secret proof count and parsed
   response, borrows the request, and makes no transport, provenance, retry,
   trust, format, storage or notification claim.
6. **Verified — compatibility and ownership are additive.** No dependency,
   feature, manifest, lockfile, target, unsafe-code, chain-specific, downstream,
   publication, release or `main` state changes.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.
