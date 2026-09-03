# Verification evidence

- **Issue:** #39 (child of #5 / `IDR-005`)
- **Develop base:** `569c0572a75c18c9b3a7fdff8b93e8ec94137d4f`
- **Reviewed implementation head:** `df5fb16488fd1b9b7c9d8380e7d0afd536a34497`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command or lane | Result |
| --- | --- |
| `cargo test -p identus-did` | passed; 47 tests, three manual diagnostics ignored |
| `cargo test --workspace --all-features` | passed |
| workspace strict Clippy and rustdoc | passed |
| browser WASM, Android ARM64 and iOS ARM64 release builds | passed |
| Rust 1.85 workspace and feature-surface builds | passed |
| formatting, EditorConfig, text/TOML/Nix lint and `git diff --check` | passed |
| strict factory and OpenSpec validation | passed |
| dependency policy and advisory lanes | passed |
| `nix flake check --print-build-logs` | passed all 26 applicable local checks |

The full Nix matrix independently exercised the Rust 1.85 MSRV, default and
minimal builds, all feature surfaces, WASM/mobile targets, Clippy, rustdoc,
formatting, supply-chain policy, the 180/180 workspace Nextest run and the
85/85 KMP-compatible run. Incompatible `x86_64-linux` outputs are omitted on
Darwin and remain hosted-CI evidence.

The audit derivation emitted its known offline crates.io yanked-index lookup
noise, found no advisory failure and completed successfully. No dependency or
lockfile changed in this slice.

## Performance observation

Release-mode parsing of the representative full resolution result completed
50,000 iterations in 231.944375 milliseconds, approximately 215,569 results
per second on this host. This is an observation, not a portable timing
threshold; #41 owns calibrated drift measurement.

## Contract evidence

- Current W3C-shaped success, ordinary error, deactivation and serialized DID
  URL dereferencing results round-trip semantically.
- All nine W3C error URLs, an extension error URL, RFC 9457 problem members and
  every explicit legacy-keyword migration are covered.
- Metadata vectors cover UTC timestamps, versions, canonical/equivalent DIDs,
  proof/method extensions and PRISM/Midnight producer plus Lace/Oxid consumer
  shapes without importing downstream policy.
- Negative vectors cover every contradictory state, omitted required nullable
  fields, requested/document mismatch, cross-method claims, duplicates,
  malformed scalar values, reserved collisions and raw/aggregate limits.
- Open content covers DID document, verification method, service, URI and
  future JSON projections while rejecting null and incorrect typed projections.
- Native construction and JSON deserialization converge on the same immutable
  validation path with stable redaction-safe `did.invalid_resolution` errors.

## Iteration effort

The wall clock starts at issue #39 work on `2026-09-03T03:02:31Z` and ends
after exact-head review on `2026-09-03T03:26:25Z`: 23 minutes 54 seconds. It
includes specification, implementation, corrections and full local gate wait,
so it is an operational throughput measure rather than person-hours. The
planning estimate was 3–5 agent-hours; reuse of the existing DID document
validation foundation and cached Nix derivations kept elapsed time below it.

## Readiness receipt

Generated after the signed implementation head and evidence passed
`factory ready`:

```text
Change: add-did-resolution-results
Branch: codex/idr-005-resolution-result
Head SHA: df5fb16488fd1b9b7c9d8380e7d0afd536a34497
Develop merge base: 569c0572a75c18c9b3a7fdff8b93e8ec94137d4f
Factory contract: passed
Product-specific gates: not asserted; attach their outputs separately
```

## Repository boundary

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were read-only
evidence sources. No donor or consumer repository, SDK `main`, live GitHub
setting, publication surface or release state was changed. Hosted CI, merge and
final integration SHAs are recorded on the pull request and issue because they
occur after this local receipt.
