# Semantic preflight review

This review evaluates the issue and specification before implementation. It
does not claim that `IDR-002`, `IDR-003` or any later component is delivered.

| Dimension | Result |
| --- | --- |
| Sponsor intent | Advances the first incomplete R0 rows before crypto convergence |
| Issue-first rule | Focused GitHub issue #22 exists before implementation |
| Source evidence | SDK base and immutable NeoPRISM etalon revision are recorded |
| Repository boundary | No donor build, code copy or consumer mutation is authorized |
| Dependency threats | Alias, target table, Git, path and resolved-closure bypasses are covered |
| Compatibility truth | Host tests, cross compilation and future/runtime support are distinct |
| MSRV | Stable `1.85.0` is tested separately from the pinned nightly |
| FFI and budgets | Unsupported/deferred states are explicit rather than inferred from placeholders |
| Public API | No runtime API, crate namespace or release promise changes |
| Rollback | Repository-only checks and policy can be reverted without data migration |

## Findings

- Blockers: 0.
- Resolved in design: broad substring denial could reject unrelated crates;
  the guard uses normalized explicit families and donor source identities.
- Resolved in design: cross compilation could be mistaken for platform
  certification; policy tiers forbid that interpretation.
- Resolved in design: the etalon nightly could conceal MSRV drift; an actual
  stable-floor build is independent and required.
- Follow-up: runtime browser/mobile validation belongs to consumer-shaped
  adapters after a real portable component candidate exists.
- Follow-up: FFI and quantitative budgets require focused future issues.
- Verdict: READY to implement issue #22 after strict structural validation.

# Final local review

A distinct post-implementation pass reviewed the complete staged delta against
issue #22, both OpenSpec capabilities and the canonical `IDR-002`/`IDR-003`
acceptance evidence.

## Findings resolved

- The inherited WASM derivation used `cargoBuildCommand`, an attribute ignored
  by the pinned Crane `cargoBuild` helper, so it had only performed a native
  build. All target gates now use `cargoExtraArgs`; a negative validator test
  prevents either known ignored custom-command attribute from returning.
- The first boundary implementation used Rust let-chains accepted by the
  etalon nightly but unavailable on the declared Rust `1.85.0` floor. The code
  was rewritten for MSRV, and the real stable-toolchain workspace build passes.
- The first Nix layout repeated attribute paths and the policy TOML was not in
  canonical format. Statix and Taplo exposed both defects; the structures were
  consolidated/formatted and their focused gates pass.
- The first support validator searched an entire Nix file for a policy evidence
  token. Neighboring mobile gates could therefore mask a swapped command. It
  now scopes evidence to the exact Crane derivation block, with a regression
  that swaps the Android and iOS targets.
- Hosted review found that exact donor URLs did not reject a neutral package
  hosted in another prohibited-family repository. Source validation now also
  checks the normalized repository basename, with Compact/Cardano/Pallas/PRISM
  family regressions.
- Hosted review found that a target gate could retain its triple while dropping
  a claimed package. The policy validator now compares every gate's exact
  `-p` package set to the machine contract, with a package-loss regression.
- Exact-head hosted review found that the single MSRV build covered only
  default features. Every declared feature surface now has an independent Rust
  `1.85.0` gate, including crypto minimal/KMP and all entropy combinations.
- Exact-head hosted review found that retaining a feature evidence token could
  conceal a changed package, default-feature mode or additional feature. The
  validator now compares the complete Cargo selection for every nightly and
  MSRV feature gate and proves each MSRV gate uses the stable Crane instance,
  with regressions for each drift class and toolchain substitution.

## Final result

| Dimension | Result |
| --- | --- |
| Boundary coverage | Direct aliases/packages, every dependency-table kind, donor Git/path locations and lockfile closure are enforced |
| Compatibility truth | Host-tested, compile-only, planned and unsupported states are separate and machine-validated |
| Actual target evidence | Rust 1.85, browser WASM, Android ARM64 and iOS ARM64 commands were observed in successful Nix build logs |
| Feature isolation | Crypto minimal/KMP and entropy minimal/deterministic/getrandom/all surfaces have exact nightly and MSRV gates |
| Public/runtime impact | No runtime API, FFI, release, downstream migration or certification promise is introduced |
| Repository boundary | Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited or built |
| Blocking findings | 0 |

Verdict: READY for archive, signed commit, pull request and hosted Linux CI.
