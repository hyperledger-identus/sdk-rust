# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #147, OpenID4VCI Final sections 8.3.1.1 and 8.3.1.2,
  proposal, design, capability delta, and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — payload and authorization errors could be conflated.** The new
   boundary accepts only the section 8.3.1.2 status/media/body contract,
   rejects its explicitly displaced generic `invalid_request`, and leaves RFC
   6750 responses and authentication challenges to a separate capability.
2. **Cleared — a non-normative example could become a false header mandate.**
   Cache-Control is neither accepted nor required because `no-store` appears
   only in the example, unlike the normative Credential Nonce response rule.
3. **Cleared — a method on request state could overclaim correlation.** The
   associated parser returns only the bounded body core and does not accept or
   retain request state, endpoint identity, status, or header data.
4. **Cleared — HTTP composition could weaken parser bounds.** Status is checked
   before remote fields, media length/grammar before body, and the existing
   complete body/depth/node/value limits remain authoritative.
5. **Cleared — strict known-code validation would break extensions.** Only the
   exact generic value forbidden by the Final payload branch is rejected; all
   other valid unknown codes remain exact `Extension` values without policy.
6. **Cleared — remote content could escape through diagnostics.** New errors
   are fieldless/static, envelope values are discarded, and the existing
   zeroizing/redacted body state is returned unchanged.
7. **Cleared — consumer precedent could become normative.** The pinned Final
   text controls behavior; Oxid and Portal remain read-only evidence and no
   source or fixture is copied.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration, or downstream-write boundary
is crossed. Implementation may proceed.

# Exact-diff implementation review

## Review boundary

- Base: `1b6cb5bdc755d6bf11fe2ded08c91cdcc50306e6`
  (`origin/develop`).
- Reviewed implementation head:
  `b239f975c368d3481574c613e87a433394a000b0`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including the public API, ordering, parser composition, limits, errors,
  tests, ADR, blueprint, inventory, backlog replacement, and OpenSpec
  artifacts.
- Confirmed `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` are
  byte-identical to the base.

## Verification evidence

- Focused all-feature and no-default-feature HTTP-response suites: 8 passed in
  each mode.
- Workspace all-feature and no-default-feature tests, warnings-denied Clippy,
  strict rustdoc, formatting, diff, and factory/OpenSpec validation passed.
- Full pinned `nix flake check -L` passed all 27 evaluated gates, including
  Rust 1.85 MSRV, browser-WASM, Android ARM64, iOS ARM64, nextest (578 passed),
  formatting, docs, text/TOML/Nix lint, cargo-deny, cargo-audit, and factory
  checks. The Nix audit derivation retained its known offline yanked-index
  diagnostics and the Darwin fixup emitted its known shell-pipeline message;
  the complete flake check exited zero with `all checks passed`.
- Oxid remained clean at
  `5ba38b9bbc9326c294b353daaf2a074eca18c22f`. Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with only its pre-existing
  untracked `.pi-subagents/`, `.pi/`, and `tmp/` paths.

## Findings

1. **Verified — protocol branches stay distinct.** Exact status 400 and JSON
   media are required for payload errors, while RFC 6750 authorization errors
   and challenges remain outside the API.
2. **Verified — validation order limits hostile work.** Status fails before
   either remote field, Content-Type size and grammar fail before body parsing,
   and the existing complete body/depth/node/value limits remain authoritative.
3. **Verified — Final codes remain extension-compatible.** The exact displaced
   `invalid_request` value is rejected only at this envelope; all seven known
   payload errors retain their classes and every other valid extension remains
   exact without policy authority.
4. **Verified — the API does not fabricate provenance.** Success returns only
   the existing bounded body core and retains no status, media, endpoint,
   request, or transport state.
5. **Verified — remote values remain redacted.** New errors are fieldless and
   static, tests cover canaries through direct and bridged diagnostics, and the
   existing zeroizing body representation is unchanged.
6. **Verified — scope and portability are additive.** No dependency, feature,
   manifest, lockfile, unsafe-code, target, chain, consumer, publication,
   release, repository-setting, or `main` change occurs.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation, or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.
