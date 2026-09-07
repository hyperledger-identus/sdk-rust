# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #145, OpenID4VCI Final section 8.3.1.2, proposal,
  design, capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — the Final's named codes are not a closed wire registry.** The
   specification says implementations SHOULD use the seven values, so the
   contract preserves every valid exact code and classifies unknown values as
   `Extension` rather than rejecting future interoperability.
2. **Cleared — Credential and Token Endpoint errors must not share a public
   type.** Their known registries and members differ. The implementation reuses
   only private bounded JSON machinery and exposes a distinct credential
   response surface without `error_uri`.
3. **Cleared — superseded draft nonce fields could regain authority.**
   `c_nonce`, `error_uri`, and other foreign fields are bounded unknown
   extensions, duplicate checked and discarded without semantic access.
4. **Cleared — remote descriptions are unsafe wallet copy.** The optional
   value is strict NQSCHAR-compatible input, stored in zeroizing memory, exposed
   only through an explicitly untrusted accessor, and absent from diagnostics.
5. **Cleared — body syntax could be mistaken for transport truth.** The type
   and requirements disclaim HTTP status/media/authentication, origin,
   correlation, issuer truth, retryability, blame, remediation, and UI policy.
6. **Cleared — hostile extensions can bypass known-field limits.** Complete
   bytes, depth, aggregate nodes, and duplicate decoded names remain bounded
   for every known and unknown value before unknown content is discarded.
7. **Cleared — consumer code is not normative.** Oxid and Portal supply only
   read-only architectural evidence; no source or fixture is copied. The Final
   text controls code grammar and semantics.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral and within the
standing IDR-023 mandate. No protected product, governance, release,
licensing, security-disclosure, repository-administration or downstream-write
boundary is crossed. Implementation may proceed.

# Exact-diff implementation review

## Review boundary

- Base: `38512431f38872b19d1ddfea1b558b5ad928f9fa`
  (`origin/develop`).
- Reviewed implementation head:
  `595926840bc32d77a0a556df8473679d2e4b651e`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including the public API, parser projection, bounds, errors, tests, ADR,
  blueprint, inventory, backlog replacement and OpenSpec artifacts.
- Confirmed `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` are
  byte-identical to the base.

## Verification evidence

- Focused all-feature and no-default-feature response suites: 8 passed in each
  mode.
- Workspace all-feature and no-default-feature tests, warnings-denied Clippy,
  strict rustdoc and factory/OpenSpec validation passed.
- Full pinned `nix flake check -L` passed all 27 gates, including Rust 1.85
  MSRV, browser-WASM, Android ARM64, iOS ARM64, nextest (570 passed), format,
  docs, text/TOML/Nix lint, cargo-deny, cargo-audit and factory checks. The Nix
  audit derivation retained its pre-existing offline yanked-index diagnostics
  but exited green.
- Oxid remained clean at
  `5ba38b9bbc9326c294b353daaf2a074eca18c22f`. Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with only its pre-existing
  untracked `.pi-subagents/`, `.pi/` and `tmp/` paths.

## Findings

1. **Verified — exact standard classes remain extension-compatible.** All
   seven Final strings classify case-sensitively, while other non-empty valid
   ASCII codes retain their exact value and receive only `Extension` authority.
2. **Verified — resource work is bounded before retention.** Complete bytes,
   depth, aggregate nodes, code bytes and description bytes have positive
   independent limits; duplicate decoded names fail at every object depth.
3. **Verified — foreign fields do not regain legacy semantics.** `c_nonce`,
   `error_uri` and other bounded members are traversed and discarded, with no
   public accessor or policy effect.
4. **Verified — remote values are least-authority and redacted.** Only code and
   optional description survive in zeroizing storage; descriptions are named
   untrusted; response/code Debug and direct/bridged errors expose no canary.
5. **Verified — body parsing does not overclaim protocol state.** The API and
   contract consistently disclaim HTTP/authentication validity, provenance,
   correlation, issuer truth, retry, blame, remediation, localization, UI,
   trust, verification and storage.
6. **Verified — compatibility and ownership are additive.** The implementation
   reuses the private scanner without changing existing response behavior and
   adds no dependency, feature, manifest, lockfile, target, unsafe-code, chain,
   downstream, publication, release or `main` change.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.
