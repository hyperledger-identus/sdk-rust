# Verification evidence

- **Date:** 2026-09-03
- **Implementation head:** `cd8538877c44515fa992520c3f48b79eb25bc012`
- **Develop merge base:** `1e2e64265e048c5f7bfe06ffcba39de881611517`
- **Issue:** #41

## Focused behavior and quality

The following gates passed at the implementation head:

- `cargo test -p identus-did`
- focused strict Clippy for `identus-did`
- focused rustdoc with warnings denied
- `cargo fmt --all -- --check`
- `./scripts/factory check`
- `git diff --check`

The ignored release diagnostic processed 50,000 representative results and
63,050,000 input bytes. Typed-only parsing took 227.985458 ms; hardened parsing
took 392.819166 ms; the normalized scanner ratio was 1.723x. These values are
observational and carry no machine-specific pass threshold.

`cargo llvm-cov 0.9.0 -p identus-did --all-features --summary-only` passed with:

| Scope | Region coverage | Function coverage | Line coverage |
| --- | ---: | ---: | ---: |
| `resolution.rs` | 92.66% | 90.13% | 93.84% |
| `wire_json.rs` | 94.12% | 87.50% | 94.89% |
| `identus-did` total | 89.85% | 90.63% | 90.93% |

The coverage helper was installed under `/tmp`; it did not change the
repository or the user's Cargo installation.

## Workspace and portability

All of these gates passed at the implementation head:

- `cargo test --workspace --all-features`
- `cargo test --workspace --no-default-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`
- `cargo fmt --all -- --check`
- `./scripts/factory check`
- `git diff --check`

`/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs` passed
all 26 `aarch64-darwin` checks from the repository's locked flake. This includes
Rust 1.85 MSRV and feature-minimal builds, 251/251 release workspace tests,
entropy and KMP feature matrices, strict Clippy, rustdoc, formatting, WASM,
Android AArch64, iOS AArch64, architecture/factory contracts, cargo-deny,
cargo-audit, and Nix/TOML/text linting. The flake correctly reported
`x86_64-linux` as an incompatible host system omitted by this Darwin run;
hosted CI remains the Linux authority.

## Resource and conformance evidence

- Raw results are capped at 512 KiB before scanning.
- Scanner work is capped at 64 nested containers, 16,384 JSON values, 128
  members per object, and 128 KiB of simultaneously live decoded names.
- DID Resolution datetime input is capped at 128 ASCII bytes.
- Deterministic tests exercise 512 scalar seeds, 256 RFC 9457 extension errors,
  every resolution/dereferencing state combination, every scanner ceiling, and
  portable result assertions from the pinned official W3C suite.
- The official suite publishes no reusable dereferencing fixture corpus, so no
  dereferencing-suite coverage is claimed.

## Review and repository scope

The distinct exact-head review in `review.md` found no blocker. NeoPRISM,
midnight-identity, Lace ID Portal, Oxid, and Apollo retained their exact
preflight HEAD/branch/status receipts. No dependency, downstream repository,
release, publication, administration setting, or reserved `main` branch was
changed.
