# Verification receipt

Verification status: passed with recorded non-authoritative host-tool failures
Verification date: 2026-09-09
Base: develop@1c47ff01094c08a4575ad86a162fa40d49f0d7c0
Evidence head: ff298fb5eb4dde38c64eec5b2d3e1719093243ac

## Binding proof

- `docs/research/uniffi-did-spike/scripts/verify.sh`: passed.
- Rust: 4/4 unit tests plus strict fixture Clippy and release `cdylib` build.
- Swift 6.3: compiled and linked host program; valid round-trip/component,
  invalid input, oversized input and redaction assertions passed.
- Kotlin 2.2.20/JVM 17/JNA 5.18.1: Gradle compile/link/runtime passed the same
  behavior families.
- Two complete generated trees: byte-identical.
- Swift and Kotlin normalized API snapshot diffs: passed.
- Isolated Cargo lock: 85 packages,
  `248c8414b01e937dfcce98f23bca85887a1775634e30639adbed972ab1a3d8b4`.

## Repository evidence

- `scripts/factory check`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo doc --workspace --no-deps`: passed.
- `nix flake check`: all 31 compatible aarch64-Darwin checks passed, including
  factory, format/TOML/text, pinned Clippy, tests, MSRV/etalon, WASM, Android,
  iOS, docs, deny and root audit; x86_64-linux was host-incompatible and remains
  hosted `fast` evidence.
- `git diff --check`: passed after report/TOML normalization.
- Root `Cargo.toml`, `Cargo.lock`, `crates/did` and `crates/bindings`: no diff.
- All three evidence commits have good local GPG signatures and DCO trailers.

## Recorded failures and exclusions

- Direct host `cargo clippy --workspace --all-targets --all-features -- -D
  warnings` failed on four Rust 1.98 lints in unchanged baseline files: three
  collapsible `if` findings in conformance and one manual no-op waker in a
  credentials test. The repository-pinned Nix Clippy derivation passed.
- Fixture `cargo-audit` did not evaluate the lock because the installed client
  cannot parse a current CVSS 4.0 advisory database entry. No clean fixture
  advisory claim is made; #222 requires the pinned supply-chain gate.
- Intentionally unrun: mobile device/emulator runtime and packaging, React
  Native, browser/Node, callbacks/async/objects, secrets/custody, publication,
  release and certification.

## Isolation

Apollo, NeoPRISM, Midnight, Oxid, Lace and other consumer repositories were not
mutated. Hosted Linux `fast`, PR policy/signature, merge and Discussion/issue
receipts remain delivery evidence after publication of the branch.
