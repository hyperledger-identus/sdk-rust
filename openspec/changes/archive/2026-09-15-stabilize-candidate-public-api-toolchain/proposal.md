## Why

The first post-activation slow-lane canary at protected `develop` revision
`59a8db6b7c34439f4f84d762bc1fd56c6ecba9d0` proved that the unpublished
candidate is not reproducible on a clean Linux runner. `cargo-public-api`
observes the stable Cargo binary, silently selects a rustup-owned `nightly`
toolchain for rustdoc JSON, and fails because the Nix application deliberately
does not install or mutate rustup state.

The intended contract in ADR 0113 is already narrower: Rust 1.98.1 plus a
subprocess-scoped `RUSTC_BOOTSTRAP=1` generates the inspection-only rustdoc
JSON. The implementation must make that boundary explicit instead of relying
on `cargo-public-api`'s implicit toolchain discovery.

## What changes

- Generate the all-feature `identus-crypto` rustdoc JSON explicitly with the
  Nix-pinned Rust 1.98.1 Cargo/rustdoc and subprocess-scoped
  `RUSTC_BOOTSTRAP=1`.
- Give the completed JSON file to locked `cargo-public-api 0.52.0` for parsing
  so it cannot invoke or install a rustup toolchain to build evidence.
- Fail closed when JSON generation does not produce the one expected package
  file, and keep the build output inside disposable candidate scratch.
- Extend the structural checker and mutation suite to preserve this separation.
- Add successor ADR 0121 without mutating accepted ADR 0113, and update the
  candidate specification so the executable path and compiler claim agree.

## Capabilities

### Modified capabilities

- `unpublished-crypto-candidate`: makes public-API evidence independent of an
  installed rustup nightly while retaining the exact stable compiler and
  locked parser.

## Non-goals

- No nightly compatibility lane, rustup installation, floating download, new
  dependency, package version, public API, release or publication operation.
- No change to required fast CI or to the weekly/manual slow cadence.
- No claim that `RUSTC_BOOTSTRAP` is acceptable for SDK compilation; it remains
  restricted to release-evidence rustdoc JSON generation.

## Delivery

Issue #276 owns the activation canary. This focused repair targets protected
`develop`, closes and immediately reopens #276 under the current contribution
policy, and is complete only after a new exact-merged-head canary proves the
candidate plus the remaining slow jobs.
