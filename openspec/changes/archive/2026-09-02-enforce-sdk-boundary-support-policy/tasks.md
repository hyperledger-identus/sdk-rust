## 1. Contract and Evidence

- [x] 1.1 Create focused GitHub issue #22 before implementation with immutable
  inputs, threats, dependency cone, target matrix and acceptance evidence.
- [x] 1.2 Define the dependency-boundary and support-policy requirements,
  design decisions and semantic preflight with no unresolved blocker.

## 2. Chain-Neutral Boundary

- [x] 2.1 Add deterministic direct-manifest and lockfile-closure rules for
  prohibited dependency identities, aliases, sources and escaping paths.
- [x] 2.2 Add positive and adversarial tests for normal, development, build,
  target-scoped, patch/replace, renamed, Git, path and transitive dependency
  cases.

## 3. Support Contract

- [x] 3.1 Add the normative machine-readable matrix and human compatibility
  policy for MSRV, etalon, hosts, targets, features, FFI and deferred budgets.
- [x] 3.2 Add drift validation against Cargo, Nix systems/toolchains/checks and
  flake-rooted gate reachability, exact effective workspace selection and
  unique policy identities; ignore commented Nix evidence and prove MSRV Crane
  wiring; bind gates to Crane operations and parse exact Cargo targets/features;
  then integrate the validator into the structural factory gate.
- [x] 3.3 Add separate MSRV gates for every implemented feature surface plus
  browser-WASM, Android ARM64 and iOS ARM64 checks through reproducible Nix.

## 4. Program State and Verification

- [x] 4.1 Repoint `IDR-002` and `IDR-003` to issue #22 and update blueprint,
  roadmap or ADR text where the new evidence supersedes a gap.
- [x] 4.2 Pass focused negative tests, Cargo gates, strict OpenSpec/factory
  checks and the full local Nix flake check without donor repository changes.
- [x] 4.3 Perform a distinct local semantic/code review, resolve every finding,
  sync current specs, archive the change and produce the readiness receipt.
