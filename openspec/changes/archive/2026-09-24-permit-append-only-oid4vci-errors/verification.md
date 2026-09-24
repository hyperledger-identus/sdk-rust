# Verification

Verification date: 2026-09-25

## Exact focused evidence

- `cargo test -p identus-oid4vci --lib`: 4 passed.
- `cargo clippy -p identus-oid4vci --all-targets --all-features -- -D warnings`:
  passed.
- `python3 scripts/check-error-golden.py .`: passed with the original OID4VCI
  SHA-256.
- `scripts/factory check`: 74 OpenSpec/factory items passed.

## Workspace evidence

Before the final mutation-only test refinement, the same implementation ran:

- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-targets --all-features`: passed; only declared
  diagnostic tests were ignored.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed for 21 packages.

The final refinement adds only three crate-local test cases and re-ran focused
format, test, Clippy and factory gates at reviewed head
`c15ab7aece643f5250ccea2f4aacbe8bd5fe8a14`.

## Compatibility result

The production/public diff is empty. The immutable fixture and checker are
unchanged, the planning receipt is valid, and no dependency, feature, lockfile,
wire, runtime, target, consumer or release claim changed.
