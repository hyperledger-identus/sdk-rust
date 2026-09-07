# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #149, OpenID4VCI Final section 8.3, proposal, design,
  capability delta, and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — `number` could be narrowed silently to an integer or float.**
   The contract accepts all mathematically positive JSON-number forms and
   retains the bounded exact lexeme without conversion, rounding, or overflow.
2. **Cleared — syntax could acquire scheduling policy.** The core exposes an
   exact interval value only and makes no duration, cap, clock, retry, backoff,
   or wake-up decision.
3. **Cleared — the transaction handle could be treated as public metadata.**
   It is zeroizing, Debug-redacted, and available only through an explicitly
   sensitive accessor; provenance, freshness, and authorization are disclaimed.
4. **Cleared — immediate and deferred members could coexist ambiguously.** Both
   deferred members are mandatory while `credentials` and `notification_id`
   are explicit branch violations; the existing immediate parser is unchanged.
5. **Cleared — extension traversal could bypass resource limits.** Complete
   bytes, depth, aggregate nodes, top-level members, retained identifier, and
   interval lexeme each have positive independent bounds; duplicate decoded
   names fail before projection.
6. **Cleared — a body type could overclaim HTTP or transaction truth.** The API
   carries no status, media, request, endpoint, transport, or correlation state.
7. **Cleared — downstream behavior could become normative.** The pinned Final
   controls semantics; Oxid and Portal remain read-only and no code or fixture
   is copied.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No unresolved correctness, privacy, compatibility,
provenance, or product-scope blocker remains. Implementation may proceed.

# Exact-diff implementation review

## Review boundary

- Base: `39386cf968ee241fd7968e85a506e3aed25bd5b6`
  (`origin/develop`).
- Reviewed implementation head:
  `89b73336d243f01fedb294c4c23aa559bba0a356`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including public types, parser flow, allocation and structural limits,
  errors, tests, ADR, blueprint, inventory, backlog, and OpenSpec artifacts.
- Confirmed all Cargo manifests and `Cargo.lock` are byte-identical to base.

## Verification evidence

- Focused all-feature and no-default-feature deferred-response suites: 9
  passed in each mode.
- Workspace all-feature and no-default-feature tests, warnings-denied Clippy,
  strict rustdoc, formatting, diff, and 44-item factory/OpenSpec validation
  passed.
- Full pinned `nix flake check -L` passed all 27 evaluated gates, including
  Rust 1.85 MSRV, browser-WASM, Android ARM64, iOS ARM64, nextest (587 passed),
  formatting, docs, text/TOML/Nix lint, cargo-deny, cargo-audit, and factory
  checks. The audit derivation retained its known offline yanked-index
  diagnostics and Darwin fixup emitted its known shell-pipeline message; the
  complete flake check exited zero with `all checks passed`.
- Oxid remained clean at
  `fe6db7b87efdaf8d72b42808974b06ce8260ce1c`. Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with only its pre-existing
  untracked `.pi-subagents/`, `.pi/`, and `tmp/` paths.

## Findings

1. **Verified — positive interval syntax is exact and complete.** Valid
   integer, fractional, and exponent forms retain their source lexeme; zero,
   leading-minus, non-number, malformed, and oversized forms fail with static
   interval errors without arithmetic conversion.
2. **Verified — hostile input is bounded before semantic use.** Complete body,
   depth, node, top-level member, decoded transaction ID, and exact interval
   lexeme limits are independent and positive; decoded duplicate names fail.
3. **Verified — response branches cannot be confused.** Both deferred members
   are mandatory and either immediate-only field causes a dedicated branch
   conflict after its value is structurally traversed.
4. **Verified — retained capability material is controlled.** The decoded
   transaction identifier and exact interval enter zeroizing ownership,
   access is deliberate, and core/type Debug plus direct and bridged errors
   reveal no remote canary.
5. **Verified — syntax does not become policy.** The core retains no HTTP,
   request, endpoint, clock, duration, retry, polling, or trust state and makes
   no validity, freshness, correlation, or authorization claim.
6. **Verified — scope and portability are additive.** No dependency, feature,
   manifest, lockfile, unsafe-code, chain, consumer, publication, release,
   repository-setting, or `main` change occurs.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation, or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.
