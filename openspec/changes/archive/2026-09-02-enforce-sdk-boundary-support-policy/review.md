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
- Exact-head hosted review found that an orphaned Nix module could still satisfy
  textual gate discovery. Required gates are now discovered only through the
  imported check-module graph, with an import-removal regression.
- Exact-head hosted review found that target gates did not machine-declare the
  entropy backend they claimed to compile. Compile-checked targets now declare
  and validate exact package-qualified feature selections.
- Exact-head hosted review found that Cargo patch and replacement tables could
  redirect neutral package names outside the repository without a lockfile
  source. The boundary guard now applies identity, Git/source and canonical-path
  checks to both override forms.
- Exact-head hosted review found that seeding discovery directly at
  `nix/checks/default.nix` could accept a check tree detached from `flake.nix`.
  Discovery now proves the root flake import before traversing check modules,
  with a flake-import removal regression.
- Exact-head hosted review found that an empty explicit package selection could
  treat `--workspace --exclude <package>` as complete workspace coverage. Gate
  comparison now requires explicit workspace mode, resolves the effective
  workspace package set and rejects every exclusion, with package-loss and
  implicit-default regressions.
- Exact-head hosted review found that duplicate host, target or feature keys
  were silently resolved using one entry. Policy indexing now rejects every
  duplicate normative identity, with a regression for each entry type.
- Exact-head hosted review found that a child module path retained as a Nix
  comment still counted as imported. Nix line and block comments are now
  removed before import and gate discovery; the import regression preserves
  the path only as a comment.
- Exact-head hosted review found that MSRV gates could use a correctly named
  Crane library rewired to the nightly toolchain. Structural validation now
  proves both the stable `msrvToolchain` binding and the `msrvCraneLib` override
  edge, with an exact nightly-substitution regression.

## Final result

| Dimension | Result |
| --- | --- |
| Boundary coverage | Direct aliases/packages, every dependency-table and override kind, donor Git/path locations and lockfile closure are enforced |
| Compatibility truth | Host-tested, compile-only, planned and unsupported states are separate and machine-validated |
| Actual target evidence | Rust 1.85, browser WASM, Android ARM64 and iOS ARM64 commands were observed in successful Nix build logs |
| Feature isolation | Workspace defaults plus crypto minimal/KMP and entropy minimal/deterministic/getrandom/all surfaces have exact nightly and MSRV package/feature gates without exclusions |
| Public/runtime impact | No runtime API, FFI, release, downstream migration or certification promise is introduced |
| Repository boundary | Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited or built |
| Blocking findings | 0 |

Verdict: READY for archive, signed commit, pull request and hosted Linux CI.
