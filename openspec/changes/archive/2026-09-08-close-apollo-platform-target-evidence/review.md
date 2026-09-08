# Distinct local review

## Scope reviewed

- Exact diff from
  `origin/develop@244ded689a29a5e6606eeb36aede14149d585071` through the
  current branch.
- Issue #213, the Apollo parity ledger/checker/renderer, its mutation tests and
  the additive OpenSpec contract.
- `sdk-support-policy.toml`, `nix/checks/gates.toml`, ADR 0081,
  `SDK-LIM-003` and `SDK-LIM-009`.
- Hosted slow run 34286965807 at the exact base SHA, including the successful
  Linux/macOS jobs and explicit successful WASM, iOS and Android gate lines.
- Discussion #178 target receipt comment 18358071.

## Findings

No blocking finding remains.

1. The previous iOS and Android names in the parity manifest used stale
   `arm64` spellings. The support policy and generated Nix graph use
   `rust-build-ios-aarch64` and `rust-build-android-aarch64`; the manifest now
   matches the executable names.
2. The validator derives toolchain, package, feature, tier, gate and limitation
   expectations from the canonical support policy. The existing support-policy
   checker independently binds that policy to the Nix gate manifest, avoiding
   a duplicate hand-maintained build contract.
3. Mutation tests reject host-gate substitution, stale revisions, non-Actions
   receipts, package drift, limitation drift and malformed target arrays. A
   malformed array initially exposed a possible fail-open exception path; the
   implementation now reports validation failures without crashing.
4. The first local Nix lint run caught noncanonical TOML alignment. The file
   was formatted with the pinned Taplo and the factory, text and TOML checks
   then passed.
5. All three target-specific Nix derivations passed locally on
   `aarch64-darwin`. Hosted slow run 34286965807 completed successfully at the
   exact recorded SHA; its macOS log names all three gates with successful
   check marks.
6. The evidence retains the exact existing compile-only limitations and does
   not imply linking, runtime, bundling, FFI, packaging, storage, performance,
   certification, release or downstream adoption.
7. No crate code, dependencies, public APIs, consumer repository or fast CI
   topology changed. An ADR is therefore not required for this routine
   evidence-enforcement change.

## Non-blocking observations

- GitHub Actions log retention is finite. The run identity, exact revision,
  executable gate graph and local mutation contract remain durable, while a
  later release receipt should archive any evidence required beyond hosted log
  retention.
- The parity manifest retains its earlier behavioral baseline separately from
  the newer portable-target closing SHA. This is intentional: vector/source
  evidence remains immutable while the target receipt identifies its own exact
  build revision.
- Language bindings remain unsupported and tracked by #215/#163; compile
  evidence cannot close that separate work.

## Verdict

The change is cohesive, reversible and fail-closed against the misleading
host-only substitution named by issue #213. It is ready for Discussion receipt,
safe OpenSpec archive and an issue-linked PR to `develop`.
