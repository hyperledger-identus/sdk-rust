# Verification evidence

- **Issue:** #24, child of #20 and follow-up to #23
- **Develop base:** `a05cb05575c7d99fc025f5be450da32c7a11a5c1`
- **Reviewed implementation head:**
  `996465abe6fa68bb7d04b218f9a0f07b5f954afe`
- **Reviewed hosted-timing correction head:**
  `f05f618271a433d612753a490d9184d26c46e003`
- **Local platform:** macOS 26.2, aarch64-darwin, Python 3.14.3
- **Result:** every applicable local gate passed; hosted Linux and macOS
  benchmark steps passed

## Contract and mutation evidence

- `./scripts/check-support-policy.py` passed against the complete 23-gate
  execution manifest.
- `python3 scripts/tests/support-policy.py` passed all 36 mutations, including
  missing, duplicate and unknown data; operation, package, feature and target
  drift; dynamic/interpolated Nix and dead `_module.args` decoys; contradictory
  selections; and Cargo-selection smuggling through trailing arguments.
- The same suite passed two timing-policy regressions plus generator-import
  decoy and malformed-list regressions, for 40/40 total tests.
- `nix flake show --all-systems` emitted the same 23 named Rust gates for
  x86_64-linux and aarch64-darwin.
- Representative old/new derivations preserved their Crane operations,
  toolchains, source sets, artifacts and effective Cargo selections. Root
  Clippy and rustdoc make their existing virtual-workspace selection explicit;
  package flags use the equivalent long form.

## Timing diagnostics

The benchmark used 20 successful samples per mode. Warm samples repeatedly
called a loaded validator; process-cold samples launched a fresh Python process.
The broad `2x + 5 ms` ceiling is applied to p50 as a sustained-pathology
detector, not a compatibility or product performance promise. P95 is always
reported but does not gate because one scheduler outlier controls it at 20
samples.

| Environment | Measurement | PR head | Develop base | Ratio | Result |
| --- | --- | ---: | ---: | ---: | --- |
| Local macOS | warm p50 | 6.733 ms | 6.090 ms | 1.106x | pass |
| Local macOS | warm p95 | 7.162 ms | 6.775 ms | 1.057x | pass |
| Local macOS | process-cold p50 | 43.479 ms | 50.823 ms | 0.855x | pass |
| Local macOS | process-cold p95 | 45.861 ms | 52.201 ms | 0.879x | pass |
| Hosted Ubuntu | warm p50 | 10.763 ms | 10.161 ms | 1.059x | pass |
| Hosted Ubuntu | warm p95 | 19.624 ms | 10.280 ms | 1.909x | pass |
| Hosted Ubuntu | process-cold p50 | 56.027 ms | 64.567 ms | 0.868x | pass |
| Hosted Ubuntu | process-cold p95 | 62.473 ms | 66.020 ms | 0.946x | pass |
| Hosted macOS | warm p50 | 6.508 ms | 5.486 ms | 1.186x | pass |
| Hosted macOS | warm p95 | 7.565 ms | 9.209 ms | 0.821x | pass |
| Hosted macOS | process-cold p50 | 57.565 ms | 59.636 ms | 0.965x | pass |
| Hosted macOS | process-cold p95 | 84.605 ms | 72.663 ms | 1.164x | pass |

The hosted rows come from PR #60 `nix-checks` run `33807123452` against its
synthetic merge ref and the exact develop base. Both benchmark steps passed
before Nix installation and reported no material regressions.

A final-head macOS rerun produced a warm p95 outlier of 22.977 ms over a
7.259 ms p50 and failed the original p95 ceiling. The immediately preceding
hosted run and local runs passed, while process-cold p95 improved. This exposed
runner noise rather than a sustained regression. The gate now reports all four
ratios but applies its ceiling only to p50, with two focused regressions proving
that isolated p95 noise passes and sustained median slowdown fails.

## Full repository gates

- `./scripts/factory ready declare-nix-support-gates` passed with all four
  artifacts complete, every task checked and all 18 strict OpenSpec items
  valid.
- The pre-archive factory receipt recorded branch
  `codex/issue-24-declarative-gates`, reviewed head
  `996465abe6fa68bb7d04b218f9a0f07b5f954afe`, develop merge base
  `a05cb05575c7d99fc025f5be450da32c7a11a5c1` and a passing factory contract.
- Canonical `nix-tooling` and `sdk-support-policy` specifications were synced
  and validated before archival. OpenSpec therefore archived with
  `--skip-specs` after its updater correctly detected those requirements were
  already present.
- `./scripts/factory doctor`, change validation, `./scripts/factory check` and
  `scripts/tests/factory-contract.sh` passed.
- Ruff lint/format, nixfmt, deadnix, statix, Taplo, actionlint, markdownlint and
  `git diff --check` passed.
- `nix flake check --print-build-logs` passed twice on the exact reviewed code,
  covering all 27 compatible aarch64-darwin checks.
- Workspace Nextest passed 251/251 tests and crypto KMP passed 85/85. MSRV
  1.85, feature, WASM, Android, iOS, dependency, advisory and factory gates all
  passed.
- Local Nix correctly omitted incompatible x86_64-linux execution; hosted
  Ubuntu CI provides that independent full gate before merge.

## Compatibility and repository isolation

- No Rust public API, serialized/wire behavior, dependency cone, secret
  boundary, supported surface or consumer behavior changed.
- Apollo remained at `ccee22b` with its pre-existing dirty submodule;
  NeoPRISM remained at `d6ad1ec` and clean; midnight-identity remained at
  `427f857` with its pre-existing dirty submodule; Lace ID Portal remained at
  `804de0a` with its pre-existing untracked agent/temp paths; and Oxid remained
  at `bfe3b48` with its pre-existing untracked agent paths.
- No downstream repository was edited, staged or built for this factory-only
  change.
- Reserved sdk-rust `main` remained clean at
  `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.

## Review conclusion

The distinct contradiction-focused exact-head review found three invalid
selection combinations that the first implementation accepted. The validator
now rejects them and their regressions pass. No unresolved architecture,
security, compatibility, provenance or delivery blocker remains.

The pull-request review then found two additional fail-closed edges: a live
string could imitate the generator import, and a scalar list field could cause
a traceback after recording its schema failure. Structural import resolution
and normalized invalid values close both gaps; their focused regressions pass.
