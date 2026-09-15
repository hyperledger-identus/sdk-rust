# Review

## Scope and identities

- Base: `develop@59a8db6b7c34439f4f84d762bc1fd56c6ecba9d0`
- Planning head: `af364960fe776bcd7a3568e1e92c9a756fe2ce69`
- Implementation head: `c2a95c3c940c4e6e629f6f2101edb365f3e54261`
- Review lenses: toolchain ownership, evidence truth, ambient-state isolation,
  failure behavior, dependency/public boundary and change granularity

## Findings

1. **Resolved P2 — toolchain-discovery wording exceeded behavior.** Parser
   0.52.0 still probes Cargo/rustup metadata and computes an unused `nightly`
   value even when it receives `--rustdoc-json`. The implementation prevents
   compiler execution, not the startup probe. ADR, design, specification and
   constraints now make only that precise guarantee.
2. **Verified — bootstrap scope is one subprocess.** The base environment is
   copied only for `cargo rustdoc`; parsing, SemVer, SBOM, packaging, closure
   builds and receipt version commands use the unmodified environment.
3. **Verified — failure is closed and disposable.** The fixed target directory
   is under isolated stage scratch, the exact expected regular JSON file is
   required, and failed work cannot be atomically renamed to completed output.
4. **Verified — dependency and support boundaries are unchanged.** No manifest,
   lock, Nix package, Rust version, target claim, API, wire model, unsafe/native
   code, credential or publication authority changes.
5. **Verified — clean runner evidence.** A full clean exact-head candidate run
   succeeded with an empty `RUSTUP_HOME`; the committed API baseline did not
   change.
6. **Deferred to the next sequential repair — Android license acceptance.** The
   same canary proved PR #290's absolute command discovery but exposed a
   non-interactive Google Play system-image license prompt. It is independently
   cohesive and does not belong in this candidate API implementation.

## Result

No unresolved architecture, security, privacy, compatibility, dependency,
correctness or operations finding remains in this repair. Hosted exact-head CI
and post-merge execution remain authoritative delivery evidence.
