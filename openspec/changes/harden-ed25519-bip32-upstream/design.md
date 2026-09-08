## Context

The adopted crate is the only narrow maintained implementation found with exact
Apollo/Cardano V2 semantics. Its residual problems are localized to diagnostics,
zeroing and dependency feature declaration, so an upstream patch has lower risk
than replacing or transcribing the derivation algorithm.

## Decisions

### Preserve formatting traits but redact their output

Both `Debug` and `Display` will render the same fixed marker. Keeping the trait
implementations avoids an unnecessary compile-time compatibility break while
removing secret-dependent output. Tests assert exact output and absence of the
known key hex.

### Use zeroize without allocation or derive macros

`Drop` calls `Zeroize::zeroize` on the owned byte array. The dependency is exact
`zeroize = "=1.8.2"` with defaults disabled to preserve Rust 1.81 and `no_std`
without adding a proc-macro package. The local `securemem` module is removed.

### Name the minimal cryptoxide feature family explicitly

The manifest disables defaults and enables `ed25519`, `sha2`, and `hmac`.
`ed25519` supplies `curve25519`, while constant-time comparison is unconditional.
A feature-tree assertion and complete crate tests guard against accidental
under-specification.

### Separate upstream remediation from SDK adoption

This change produces an upstream pull request and repository evidence. It does
not use a Git dependency, patch source, unpublished fork or SDK manifest update.
An immutable upstream release is the trigger for a separate SDK update.

## Risks and mitigations

- A caller may have parsed secret `Display` output: the security correction is
  deliberate and will be called out in the upstream PR.
- Exact zeroize 1.8.2 can duplicate a newer zeroize line: measure this during
  the later SDK release update; do not relax upstream MSRV accidentally.
- A missing cryptoxide feature could hide behind host configuration: run full
  tests plus `thumbv7em-none-eabihf`, WASM, Android and iOS compile checks.
- Upstream may decline the patch: retain the current private SDK facade and use
  ADR 0078's bounded fork trigger only in a separate decision.

## Review contract

Review the exact diff for any change to derivation/signature mechanics, raw key
conversion, public traits beyond formatting behavior, new unsafe code, native
code, target regression or dependency feature expansion. Any such change is a
blocker for this focused contribution.
