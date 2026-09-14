# Clippy policy and exception registry

The repository denies Rust and Clippy warnings. Required fast CI evaluates the
default workspace surface; the local or externally orchestrated slow command
evaluates every Cargo target with every feature enabled. Hosted activation is
pending issue #276. Reproduce that complete surface locally on
the pinned Rust 1.98.1 toolchain with either command:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
nix build --print-build-logs .#checks.$(nix eval --impure --raw --expr builtins.currentSystem).rust-clippy-all-targets-all-features
```

The Nix selector is generated from `nix/checks/gates.toml`; the offline support
policy binds its structured Cargo selection. The complete check is pre-release
slow evidence under ADR 0081 and is deliberately absent from the required fast
selector set.

## Exception rules

A first-party exception must:

- be attached to the smallest item that owns the trade-off;
- use `#[expect(clippy::<lint>, reason = "...")]` for production code so a
  stale exception becomes a warning;
- have an entry below naming its component owner, rationale, and objective
  removal condition;
- preserve the public, wire, security, and resource contracts that caused it;
- avoid crate-level and workspace-wide Clippy allowances.

An exception is documented debt, not permission to suppress adjacent findings.
Remove it in the focused change that satisfies its removal condition. Test-only
helpers should normally be reshaped for readability rather than registered.

## Active production exceptions

| Source item | Lint | Owner | Rationale | Removal condition |
| --- | --- | --- | --- | --- |
| `ImmediateCredentialResponseLimits::new` in `crates/oid4vci/src/limits.rs` | `too_many_arguments` | `identus-oid4vci` | The unpublished public positional constructor exposes nine independent resource budgets. Changing it inside a static-analysis slice would mix API migration with lint cleanup and could obscure #168 resource semantics. | A focused pre-release API migration supplies a cohesive parameter type or builder, preserves every independent bound and error, includes migration evidence, and updates callers. |
| `CredentialIssuerMetadataLimits::new` in `crates/oid4vci/src/limits.rs` | `too_many_arguments` | `identus-oid4vci` | The unpublished public positional constructor exposes ten independent metadata budgets. Changing it inside a static-analysis slice would advance #7 API design and could obscure #168 resource semantics. | A focused pre-release API migration supplies a cohesive parameter type or builder, preserves every independent bound and error, includes migration evidence, and updates callers. |

No other production Clippy exception is accepted by this baseline.
