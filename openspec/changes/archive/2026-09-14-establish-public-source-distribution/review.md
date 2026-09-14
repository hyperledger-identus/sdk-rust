# Local review

- **Review date:** 2026-09-14
- **Review angle:** reproducibility, source identity, release-claim boundary,
  downstream naming, fail-closed enforcement, and fixture isolation
- **Scope:** implementation diff after planning head `b6a6d499`
- **Result:** passed after the fixture integration finding below was resolved

## Resolved finding

The first implementation registered the checker in `check-factory.sh` but did
not initially copy the new guide and README into the factory's isolated
self-test fixture. That made the self-test fail for the right reason. The
fixture now copies and executes the complete source-distribution contract and
its mutation tests.

## Contract review

- The guide separates a symbolic instruction from the full historical
  NeoPRISM canary and never represents either as a release.
- The checker derives package identity and unreleased state from Cargo
  manifests rather than duplicating their values in an unchecked table.
- Exact public HTTPS source, full revisions, Cargo lock, Nix evidence, compiler
  floor, package names, and README discoverability all fail closed.
- Negative tests cover short/mutable revisions, private source, publication
  drift, package-name drift, lock-evidence loss, and discoverability loss.
- `identus-apollo` remains downstream-owned; no compatibility alias or new
  package is introduced.

## Security and compatibility review

No credential, network fetch, dependency, unsafe code, build script, runtime,
API, wire shape, feature, target, or consumer repository changes. The channel
reduces credential pressure by making anonymous immutable source the only
supported alpha path. Publication and production support remain explicitly
unauthorized.

No unresolved blocking finding remains.
