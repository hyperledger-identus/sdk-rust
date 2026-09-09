# Verification receipt

Verification status: local and complete compatible-system gates passed
Verification date: 2026-09-09
Base: develop@808119d7f3b6d52c8fd89cfae68f0d61cb68d402
Evidence head: 3baee32de4d36579f88fee0fd9afcf21bb6039c7

## Boundary and host evidence

- `cargo test -p identus-uniffi-did`: 6/6 tests passed, covering DID and DID
  URL success, bounded input, stable redacted errors and panic containment.
- `scripts/check-uniffi-did-host.sh` passed directly and through
  `nix develop .#bindings`, including Rust test/Clippy/release build,
  two independent generation passes with full-tree equality, API snapshot
  comparison, Swift compilation/execution and Kotlin/JVM compilation/execution.
- The dedicated Nix shell supplied pinned Zulu Java 17.0.19 through
  `IDENTUS_JAVA_HOME`; the default development shell and fast PR lane were not
  enlarged.
- Generated artifact tree hashes were stable:
  Swift `04ac7bc453dcaeae854438a64ea0aa9f26fd28bc564a069f0f93e89a77a7d7d3`,
  C header `8659e7bb44171575dd3b01613b9a72630e7e142953c6b674c5651883adcc64d5`,
  modulemap `c8e2612cc4a01e1a66436fa2cb19c6847120e2a9fc2d9fdb3e554b185c5b8b2d`
  and Kotlin `66c0064923fbf720c7354e3f626c1b80adddfc9210b093d073ff728560664038`.

## Workspace and reproducibility evidence

- `cargo test --workspace --all-features` passed for every runnable test.
- `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check`,
  `cargo doc --workspace --all-features --no-deps`, `git diff --check` and
  `scripts/factory check` passed. The factory saw 19 packages and 53 OpenSpec
  items.
- The pinned Nix test matrix passed 673/673 tests with 22 configured skips.
  Targeted factory-contract, cargo-deny, cargo-audit, formatting, build,
  Clippy and test derivations passed.
- The separately locked generator package passed pinned Nix cargo-deny and
  cargo-audit checks. Root and generator lock SHA-256 values are respectively
  `0296877c4ff96da105ef2cdbc105146a999f69f95a9020933d389533adac4a41`
  and `c086a2b54f3cccad408802c270ae71fedc8e5dd419de7c91a5d2a70765c0548f`.
- Post-archive `nix flake check --print-build-logs` passed all 31 compatible
  aarch64-Darwin checks. This includes iOS, Android and WASM domain-library
  builds; primary/MSRV/etalon builds; all feature matrices; 673/673 main
  Nextest tests with 22 configured skips; docs, formatting, dependency policy,
  advisory and factory checks. The command reports x86_64 Linux as an
  incompatible local system; hosted CI supplies that independent gate.
- Workflow YAML, shell syntax, exact diff hygiene, authored-unsafe scans and the
  conformance dependency rule passed. Every branch commit is signed and carries
  a DCO trailer.

## Non-blocking diagnostics

- The host-installed cargo-audit 0.20.1 cannot parse the CVSS 4.0 record in
  RUSTSEC-2026-0073. The repository-pinned Nix cargo-audit fetched 1,242
  advisories, scanned 75 generator dependencies and passed.
- An exploratory all-targets Clippy invocation reaches the unchanged
  `manual_noop_waker` warning in a credentials test. The repository's exact
  configured CI Clippy command passes; this slice does not alter that test.
- The first post-archive flake run exposed Statix's repeated-key warning when
  the new shell shared `default.nix`. Moving the binding shell to its own
  imported module satisfied Nix lint while preserving the governed fuzz-shell
  source contract; both the exact failed gates and the full flake were rerun.

## Excluded claims

Mobile/device/package, React Native, browser/Node, publication, release and
certification remain excluded. Hosted Linux fast CI is still mandatory before
merge.
