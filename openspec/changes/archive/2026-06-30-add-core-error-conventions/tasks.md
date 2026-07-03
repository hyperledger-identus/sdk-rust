## 1. Rename stub to identus-core

- [x] 1.1 `git mv crates/identus-ssi/ crates/core/` (preserve history)
- [x] 1.2 Update `crates/core/Cargo.toml`: `name = "identus-core"` (inherit version/edition/rust-version/license from workspace)
- [x] 1.3 Update root `Cargo.toml`: `[workspace.metadata.crane] name = "identus-core"` (was `"identus-ssi"`)
- [x] 1.4 Verify `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings` pass on the empty placeholder before filling
- [x] 1.5 Verify `nix flake check` stays green (rename is transparent to the `members = ["crates/*"]` glob)

## 2. Implement the core error model (domain-facing scope)

- [x] 2.1 Add `Component` struct + `COMPONENT` const for `identus-core`
- [x] 2.2 Add `CapabilityId` and `ErrorCode` newtypes over `&'static str` (`new`, `as_str`, `const fn`, `#[must_use]`)
- [x] 2.3 Add `ErrorKind` enum with all 11 families, each variant documented
- [x] 2.4 Add `IdentusError` (code, kind, capability, public_message) with `public()` and `internal()` constructors and `const fn` accessors
- [x] 2.5 Implement redaction-safe `Display` (renders `"{code}: {message}"` only) and `Error` for `IdentusError`
- [x] 2.6 Add `IdentusResult<T> = Result<T, IdentusError>` alias
- [x] 2.7 Confirm no deferred types (`ResultEnvelope`, `ErrorEnvelope`, `RedactionPolicy`) appear in the public API

## 3. Tests

- [x] 3.1 Test: `Display` renders code + public message only
- [x] 3.2 Test: `internal()` sets `ErrorKind::Internal`, `capability = None`
- [x] 3.3 Test: `CapabilityId` attribution flows through `public()`
- [x] 3.4 Test: `ErrorCode::new(...).as_str()` round-trips a stable string
- [x] 3.5 Test: all 11 `ErrorKind` families are present
- [x] 3.6 Verify `cargo test` passes; `cargo clippy -- -D warnings` clean; `cargo fmt --check` clean

## 4. OpenSpec artifacts

- [x] 4.1 Finalize `proposal.md`, `design.md`, `specs/core-error-conventions/spec.md`, `tasks.md`
- [x] 4.2 Run `openspec validate add-core-error-conventions --strict` and resolve any findings