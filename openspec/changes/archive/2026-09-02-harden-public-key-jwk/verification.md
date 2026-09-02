# Verification evidence

- **Issue:** #28 (child of #9 / `IDR-004`)
- **Develop base:** `0449d206ceb3437cd3b26a3437c8bc4a52945666`
- **Reviewed implementation head:** `d7e3af398c94512f465a1f044c2c4e8498a95bcf`
- **Local platform:** `aarch64-darwin`
- **Result:** all applicable local gates passed

## Executed gates

| Command | Result |
| --- | --- |
| `cargo test -p identus-crypto` | passed, including existing compatibility suites and new JWK tests |
| `cargo test -p identus-crypto --test jwk` | passed, 13/13 contract and negative tests |
| `cargo build -p identus-crypto --no-default-features --features jwk` | passed |
| `cargo build -p identus-crypto --no-default-features --features ed25519` | passed |
| `cargo build -p identus-crypto --target wasm32-unknown-unknown` | passed with default features |
| `cargo test --workspace --all-features` | passed, including 20/20 conformance tests |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | passed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps` | passed |
| `cargo fmt --all -- --check` and `git diff --check` | passed |
| `./scripts/factory check` | passed; 16 OpenSpec/spec items, zero failures |
| `nix flake check` | passed all 30 local checks |

The local Nix run reports Linux as an incompatible omitted system on Darwin;
the pull-request workflow runs the flake independently on Ubuntu and macOS.

## Conformance and threat evidence

- RFC 8037 Appendix A.2 Ed25519 public JWK round-trips exactly.
- Ed25519, X25519, P-256 and secp256k1 encoders preserve raw coordinates.
- Constructor and serde gates reject unsupported or incompatible profiles,
  missing/extra coordinates, invalid alphabet, padding, non-zero trailing bits,
  wrong widths, duplicate structural members, private `d` and reserved
  extension keys.
- Unknown public extensions round-trip as uninterpreted JSON.
- Local and bridged errors do not echo rejected private/coordinate values.
- No downstream repository was modified. Pre-existing donor working-tree state
  was rechecked unchanged after implementation.

## Effort

- Issue opened: `2026-09-02T22:25:36Z`
- Spec/ADR commit: `2026-09-02T22:29:37Z` (4 minutes)
- Implementation commit: `2026-09-02T22:41:54Z` (12 minutes after spec)
- Local gate/review completion: approximately 17 minutes from issue creation

CI, hosted review, merge latency and final merge SHA are recorded on the pull
request and issue because they occur after this immutable branch receipt.
