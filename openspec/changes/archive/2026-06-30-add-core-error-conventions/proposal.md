## Why

The `add-nix-tooling` change (archived) established the workspace with a single placeholder crate `identus-ssi`. Before any domain crate (DID, crypto, credentials, messaging) can land, the workspace needs a shared, redaction-safe error and result contract that flows inward through every crate and can later cross language boundaries to bindings without leaking secrets. This change introduces `identus-core` as that foundation crate — renaming the placeholder and filling it with the typed error model. It is the keystone every later domain crate adopts; shipping it alone, with no domain adopter bundled, keeps the contract's single concern clean.

## What Changes

- Rename `crates/identus-ssi` → `crates/core` (crate `name = "identus-core"`), including the dormant `[workspace.metadata.crane] name` in the root `Cargo.toml`. The rename is transparent to the `members = ["crates/*"]` glob and to the `nix-tooling` crane checks.
- Implement `identus-core` with: `IdentusError`, `ErrorCode`, `ErrorKind` (11 families), `CapabilityId`, `IdentusResult<T>`, `Component` metadata, and a redaction-safe `Display`.
- Document the two-surface bridging convention in `design.md`: domain crates keep idiomatic errors (`FromStr`) and expose `to_identus_error()` / `parse_with_core_error()` for the core surface. No adopter ships in this change; the convention is exercised only by `identus-core`'s own tests.
- Defer the binding-facing surface (`ResultEnvelope`, `ErrorEnvelope`, `RedactionPolicy`, `to_error_envelope()`) to the bindings change, which is the first change with a binding consumer.
- Do not modify workspace version (`0.0.0`), resolver, or lint policy — those are out of scope for an error-conventions change.

## Capabilities

### New Capabilities

- `core-error-conventions`: redaction-safe typed error/result contract for the workspace foundation crate — stable error codes, error families, capability attribution, redaction-safe display, and a domain-facing bridging convention.

### Modified Capabilities

<!-- None. `nix-tooling` is untouched; the rename is transparent to the
     `members = ["crates/*"]` glob, so no nix check or CI change is needed. -->

## Impact

- **Renamed/filled files**: `crates/identus-ssi/` → `crates/core/` (`Cargo.toml` name becomes `identus-core`; `src/lib.rs` filled with the error model).
- **Root `Cargo.toml`**: one edit — `[workspace.metadata.crane] name` from `"identus-ssi"` to `"identus-core"`. No version, resolver, or lint change.
- **Dependency**: assumes `add-nix-tooling` has landed (it has — archived); it created the workspace and the `identus-ssi` stub this change renames and fills.
- **No nix change**: the `members = ["crates/*"]` glob and crane checks operate unchanged on the renamed crate.
- **No runtime/API impact** beyond the new crate's public types and tests.