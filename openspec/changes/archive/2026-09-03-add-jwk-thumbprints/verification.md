# Verification evidence

- **Issue:** #32 (child of #9 / `IDR-004`)
- **Develop base:** `3237baf4c6abf48019c9c9b5b37407e31bd1e22c`
- **Reviewed implementation head:** `958ab87a6d398df81a5db9312bf0f4373d1612d6`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command | Result |
| --- | --- |
| `cargo test -p identus-crypto --no-default-features --features jwk-thumbprint --lib --test jwk` | passed; canonical-input unit test and 16 JWK contract tests |
| `cargo build -p identus-crypto --no-default-features --features jwk` | passed; JWK remains independent of SHA-2 |
| `cargo build -p identus-crypto --no-default-features --features jwk-thumbprint` | passed |
| `cargo build -p identus-crypto --target wasm32-unknown-unknown --no-default-features --features jwk-thumbprint` | passed |
| `cargo test -p identus-crypto` | passed; 83 passed, one manual performance test ignored |
| `cargo clippy -p identus-crypto --all-targets --all-features -- -D warnings` | passed |
| `cargo fmt --all -- --check` and `git diff --check` | passed |
| `./scripts/factory check` | passed; active change and every governance contract valid |
| `nix flake check --print-build-logs` | passed all 28 applicable local Darwin checks after formatting correction |

The Nix matrix included Rust 1.85 MSRV, default and minimal builds, workspace
nextest (149/149), KMP compatibility nextest (85/85), WASM, Android, iOS,
clippy, rustdoc, Rust formatting, text/TOML/Nix lint, `cargo-deny`, RustSec
audit and the factory contract. Linux is an incompatible omitted system on the
local Darwin run and is exercised independently by CI.

## Standards and performance evidence

- RFC 8037 Appendix A.3 matched digest
  `90facafea9b1556698540f70c0117a22ea37bd5cf3ed3c47093c1707282b4b89`
  and base64url `kPrK_qmxVWaYVA9wwBF6Iuo3vVzz7TxHCTwXBygrS4k`.
- P-256 and secp256k1 generator-point canonical strings produced SHA-256
  digests `c71d01700fb0328870f1ab580c939eea9786328764946db04470655c732f9743`
  and `d8917cbe0f5eb49ce31706709a4be104b2d9d1b7cc55538f8af611e6516d71e7`
  in both the independent fixture computation and OpenSSL.
- The production path streams fixed fragments directly into SHA-256 and has
  no `String`, `Vec`, serializer or generic canonicalizer. Its work is bounded
  by the validated profile and coordinate widths. The test-only sink allocates
  solely to assert the exact canonical byte sequence.

## Provenance recheck

No downstream or donor repository was modified. Their pre-existing state was
unchanged after implementation: Apollo retains its modified nested secp256k1
checkout; midnight-identity retains its untracked nested midnight-did; Lace
retains `.pi-subagents/`, `.pi/` and `tmp/`; Oxid retains `.claude/` and
`.pi/taskflows/`; NeoPRISM remains clean. Source revisions and license
decisions are recorded in the design, ADR and issue.

Hosted CI, security review, merge and final integration SHAs are recorded on
the pull request and issue because they occur after this immutable local
receipt.
