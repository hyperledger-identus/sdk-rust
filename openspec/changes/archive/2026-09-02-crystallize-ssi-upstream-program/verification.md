# Verification evidence

Verified on 2026-09-03 from `codex/crystallize-ssi-upstream-program`, based on
`origin/develop` at `5b23855273765db20380c8f597c38c5cbef84ab4`.

## Program evidence

| Evidence | Result |
| --- | --- |
| GitHub coordination | Issue #20 created before implementation |
| Source rows | 30 SDK `IDR-*` rows; 17 Midnight `MID-*` rows excluded from SDK ownership |
| Source-field parity | Owner, priority, gate, component, outcome, acceptance, consumer and commitment compare exactly with the supplied CSV |
| Delivery distribution | 5 in progress, 4 specified, 15 queued, 6 conditional; no row is claimed delivered |
| Issue distribution | 9 rows use existing component issues; 21 queued/conditional rows use program issue #20 |

## Passing gates

| Gate | Result |
| --- | --- |
| `scripts/check-ssi-upstream-backlog.py` | Passed all 30 rows |
| `scripts/tests/ssi-upstream-backlog.py` | Passed 6 positive/negative tests |
| `scripts/tests/factory-contract.sh` | Passed factory, PR-policy and backlog contract tests |
| `scripts/factory validate crystallize-ssi-upstream-program` | Strict validation passed |
| `scripts/factory check` | Passed all current specs and the active change |
| `git diff --check` | Passed |
| Nix formatter and ShellCheck focused gates | Passed in the pinned devshell |
| `nix flake check --print-build-logs` | Passed all 13 compatible `aarch64-darwin` checks |

The full flake gate includes Rust formatting, default and KMP-compatible
clippy, two 104-test Nextest runs, docs, browser WASM build, dependency policy,
factory/OpenSpec, Nix/TOML/text hygiene and the advisory derivation.

## Environment diagnostics

- The local full gate omitted incompatible `x86_64-linux` outputs. Hosted CI
  remains the Linux confirmation.
- The existing advisory derivation reported that offline crates.io index data
  could not answer yanked-package queries, then completed successfully. This
  pre-existing limitation is not represented as a clean online yanked audit.
- Nix emitted the known non-fatal macOS `audit-tmpdir.sh` segmentation warnings
  during some artifact fixup phases; all affected derivations completed.

## Read-only source boundary

| Repository | Inspected revision | Status retained after audit |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Pre-existing dirty nested secp256k1 submodule |
| NeoPRISM | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | Clean |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Pre-existing dirty `third_party/midnight-did` submodule |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Pre-existing untracked agent/temporary paths |
| Oxid | `685f9670af4846d52697a4cfeb94779758ae1075` | Pre-existing untracked agent paths |

Only remote references were refreshed. No donor or consumer working-tree file
was switched, staged or edited.

Apollo, NeoPRISM, midnight-identity and Oxid expose Apache-2.0 root licenses at
their pinned revisions. Lace ID Portal exposes no repository license file or
workspace license metadata at its pinned revision, so it remains an
evidence-only source until a component issue resolves file-level provenance.
