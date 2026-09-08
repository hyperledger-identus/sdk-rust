# Verification

Verification date: 2026-09-09

| Evidence | Result |
| --- | --- |
| `scripts/check-apollo-parity.py` | passed; 27 capabilities and 22 vectors |
| `scripts/tests/apollo-parity.py` | passed; 12 mutation/renderer tests |
| `scripts/tests/factory-contract.sh` | passed, including fixture enforcement |
| `scripts/check-factory.sh` | passed |
| `openspec validate apollo-parity-manifest --strict` | passed |
| `cargo fmt --all -- --check` | passed |
| `cargo test -p identus-crypto --test hash --all-features` | passed; two known-answer tests |
| Nix `factory-contract`, `lint-text`, `lint-toml` | passed hermetically |
| `git diff --cached --check` | passed |

The deterministic renderer was published to Discussion #178 as
<https://github.com/hyperledger-identus/sdk-rust/discussions/178#discussioncomment-18356177>.
It reports 14 parity rows, six SDK-exceeds rows, seven accepted differences and
zero gaps at the immutable Apollo and sdk-rust baselines.

The initial full Nix invocation also exposed the repository's existing offline
`cargo audit` index warnings. The audit derivation remained advisory as defined
by the repository gate policy; this change adds no dependency.
