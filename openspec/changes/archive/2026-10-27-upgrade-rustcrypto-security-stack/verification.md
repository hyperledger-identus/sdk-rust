# Verification

## Reviewed head

- Branch: `codex/fix/issue-525`
- Base: `origin/develop` at
  `67c30d599a538b873646a6e857809f2106391bb1`
- Reviewed implementation head:
  `5ee0b83a5be72c3c69d0347ac833f5bbe21db96c`
- Reviewed archived candidate head:
  `f39656e6c0d94fb81e4156a9a39820c215e3443a`
- Independent reviewer: Codex CLI `0.163.0`, `review --base origin/develop`
- Final result: no actionable defects found

Claude CLI availability is not a delivery dependency. Codex CLI was used under
the repository's mandatory fallback rule. Earlier Codex passes identified the
published upstream-type source break and two conformance-packet integrity gaps;
all were corrected before the clean final pass.

## Local evidence

The following commands passed on the reviewed branch:

- `cargo test -p identus-crypto --all-features`
- `cargo test -p identus-crypto --no-default-features`
- `cargo test -p identus-crypto --no-default-features --features secp256k1`
- `cargo test -p identus-crypto --no-default-features --features secp256r1`
- `cargo test -p identus-crypto --no-default-features --features secp256k1,hex --test secp256k1_compat`
- root and standalone-fuzz `cargo audit --deny warnings`
- root and standalone-fuzz `cargo deny --locked check`
- standalone-fuzz formatting and strict Clippy
- workspace formatting, strict all-target/all-feature Clippy, all-feature and
  no-default-feature tests, and documentation
- `scripts/tests/factory-contract.sh`
- `./scripts/factory check`
- `/nix/var/nix/profiles/default/bin/nix flake check`

The final Nix run passed all 38 applicable `aarch64-darwin` derivations,
including MSRV, WASM, iOS, Android, audit, dependency policy, docs, Clippy,
nextest and factory mutation evidence. Nix reported only the expected omission
of incompatible `x86_64-linux` derivations from the local Darwin run.

## Security and compatibility result

Root and fuzz locks contain `crypto-bigint 0.7.10`, `k256 0.14.0`,
`p256 0.14.0` and Tokio `1.54.1`; the rejected versions are absent and no
advisory ignore was added. SDK-owned crypto methods and byte behavior retain
their vector evidence. The four published tuple fields that expose RustCrypto
types are explicitly governed as a release-candidate source migration by
`crypto.secp256k1.rustcrypto-security-stack.v1`.
