# Verification evidence

## Source and scope

- Develop base: `beb0f24cec178121dca25ae7a5ff9585efda3ae5`.
- Fully verified implementation head: `d79ef53`.
- Issue: <https://github.com/hyperledger-identus/sdk-rust/issues/193>.
- Normative sources: RFC 3986 and RFC 9110 section 4.1, retrieved from the RFC
  Editor on 2026-09-08.
- No `Cargo.toml`, `Cargo.lock`, dependency, feature, license, MSRV, native,
  build-script or FFI change.

## Focused commands passed

- `cargo fmt --all -- --check`.
- `cargo test -p identus-core`: 23 unit tests and doc-tests passed.
- `cargo clippy -p identus-core --all-targets -- -D warnings`.
- `scripts/factory research-ready bound-core-url-input` before implementation.
- `scripts/factory constraints-ready bound-core-url-input` before
  implementation.
- `scripts/factory check`: 50/50 specifications passed with the active change.
- `cargo tree -p identus-core --edges normal --prefix none`: unchanged direct
  and resolved normal dependency cone.
- `git diff --exit-code origin/develop...HEAD -- Cargo.toml Cargo.lock
  crates/core/Cargo.toml`: unchanged manifests and lockfile.
- `rg -n "unsafe|extern \"|build\.rs|links\\s*=" crates/core Cargo.toml
  crates/core/Cargo.toml`: only the inherited workspace unsafe-code prohibition;
  no authored unsafe or native boundary.
- `git diff --check origin/develop...HEAD`.

## Complete Nix gate

`nix flake check --print-build-logs` passed all 29 compatible aarch64-Darwin
checks on the committed implementation, including:

- factory contract, OpenSpec, text, TOML and Nix formatting;
- workspace/default/all/minimal feature builds under Rust 1.98.1;
- strict workspace and focused feature Clippy;
- WASM, aarch64 iOS and aarch64 Android builds;
- documentation, cargo-deny and cargo-audit derivations;
- complete nextest: 621/621 executed tests passed with 22 configured skips;
- KMP crypto: 113/113, crypto minimal: 8/8, entropy all: 4/4,
  getrandom: 1/1, deterministic: 3/3.

The local host correctly omitted x86_64-Linux as incompatible. Hosted fast CI
is the authoritative Linux gate. Cargo-audit emitted the repository's known
offline yanked-index lookup warnings but the advisory derivation completed.
The recurring Darwin Nix fixup script segmentation-fault message appeared
after successful tests and remained non-fatal, as in prior green develop runs.

## Boundary evidence

- A currently valid ASCII URL of exactly 8,192 bytes succeeds through `parse`,
  `FromStr`, `try_new`, `TryFrom<String>` and serde.
- The same shape at exactly 8,193 bytes returns/surfaces `TooLong` through every
  validated path.
- Two-byte Unicode path scalars prove `str::len()` byte semantics independently
  of character count.
- An oversized string without any URL syntax returns `TooLong`, proving the
  resource check precedes delimiter search, scheme traversal and authority
  search.
- Bridging `TooLong` yields exactly
  `core.invalid_url: URL exceeds the SDK byte limit` and contains neither the
  rejected input nor a dynamic measured length.

## Residual limitation

The bound applies after `&str`/`String` reaches validation. A caller, transport,
decompressor or generic serde deserializer may already have allocated the
source. `SDK-LIM-007` therefore remains effective for outer limits and the
other inherited surfaces not yet audited under #168.

## Archive evidence

The repository archive wrapper invoked OpenSpec after the canonical requirement
had already been synchronized. OpenSpec printed a duplicate-requirement abort
and changed no files, but returned success, so the wrapper incorrectly printed
`archived safely`. The completed active directory was then moved intact to
`openspec/changes/archive/2026-09-08-bound-core-url-input`. Post-move
`scripts/factory check` passed 49/49 canonical specifications with zero active
changes, and explicit path checks proved the active path absent and dated
archive present. The wrapper behavior is tracked separately and is not hidden
as successful OpenSpec automation.
