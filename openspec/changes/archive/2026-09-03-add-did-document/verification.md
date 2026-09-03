# Verification evidence

- **Issue:** #37 (child of #5 / `IDR-005`)
- **Develop base:** `3e3e1117c11d33746a4fc0fd8808bceaede779ce`
- **Reviewed implementation head:** `f82645917da8a90746ce471a8ec8da76abd67596`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command or lane | Result |
| --- | --- |
| `cargo test -p identus-did` | passed; 37 tests, two manual performance tests ignored |
| `cargo test --workspace --all-features` | passed |
| focused strict Clippy and rustdoc | passed |
| browser WASM, Android ARM64 and iOS ARM64 release builds | passed |
| formatting, EditorConfig, text/TOML/Nix lint and `git diff --check` | passed |
| strict factory and OpenSpec validation | passed |
| dependency policy and advisory lanes | passed |
| `nix flake check --print-build-logs` | passed all 27 applicable local checks |

The full Nix matrix independently exercised Rust 1.85 MSRV, default and
minimal builds, all feature surfaces, WASM/mobile targets, Clippy, rustdoc,
formatting, supply-chain policy, the 170/170 workspace Nextest run and the
85/85 KMP-compatible run. Incompatible `x86_64-linux` outputs are omitted on
Darwin and remain hosted-CI evidence.

## Performance observation

Release-mode parsing of the representative full DID document completed
100,000 iterations in 1.100100334 seconds, approximately 90,901 documents per
second on this host. This is an observation, not a portable timing threshold;
#38 owns durable drift measurement.

## Contract evidence

- Valid and invalid URI vectors cover schemes, authority, IPv4/IPv6,
  IPvFuture, percent escapes, query and fragment boundaries.
- Document vectors cover every DID Core verification relationship, embedded
  and referenced methods, JWK and multibase material, service endpoint URI,
  map and mixed-set forms, and unknown extensions.
- Negative vectors cover private JWK members, material collisions, duplicate
  definitions, empty required sets, malformed wire forms, maximum byte and
  collection boundaries, extension depth and shared node budgets.
- Native construction and JSON entry points enforce the same invariants; a
  semantic JSON round trip preserves extensions and one-or-many cardinality.
- `Cargo.lock` is unchanged and `identus-did` has only existing workspace
  dependencies at depth one.

## Iteration effort

The wall clock starts at issue #37 creation on `2026-09-03T02:10:33Z` and
ends after exact-head review on `2026-09-03T02:39:42Z`: 29 minutes 9 seconds.
It includes specification, implementation, corrections and full local gate
wait, so it is an operational throughput measure rather than person-hours.
The planning estimate was 3–6 agent-hours; reuse of the existing DID syntax
foundation and cached Nix derivations kept actual elapsed time below that
range.

## Readiness receipt

Generated after the signed implementation head passed `factory ready`:

```text
Change: add-did-document
Branch: codex/idr-005-did-document
Head SHA: f82645917da8a90746ce471a8ec8da76abd67596
Develop merge base: 3e3e1117c11d33746a4fc0fd8808bceaede779ce
Factory contract: passed
Product-specific gates: not asserted; attach their outputs separately
```

## Repository boundary

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were read-only
evidence sources. No donor or consumer repository, SDK `main`, live GitHub
setting, publication surface or release state was changed. Hosted CI, merge
and final integration SHAs are recorded on the pull request and issue because
they occur after this immutable local receipt.
