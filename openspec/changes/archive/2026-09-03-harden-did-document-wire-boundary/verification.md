# Verification evidence

- **Issue:** #38 (child of #5 / `IDR-005`)
- **Develop base:** `87ecd9cb970da0e8b3938c62b1eddc8ce0b73995`
- **Reviewed implementation head:**
  `41a85cbe8522a98d9e4cda632012e9a8e2b30567`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command or lane | Result |
| --- | --- |
| focused DID hardening tests and unit scanner tests | passed; 21 tests |
| `cargo test --workspace --all-features` | passed |
| `cargo test --workspace --no-default-features` | passed |
| strict workspace Clippy, rustdoc and formatting | passed |
| `./scripts/factory check` | passed; 18 OpenSpec/spec items |
| `nix flake check --print-build-logs` | passed all 37 applicable checks |
| `git diff --check` and signed/DCO commit inspection | passed |

The Nix matrix independently exercised Rust 1.85 MSRV, default/minimal and
feature-specific builds, strict Clippy, rustdoc, WASM, Android ARM64, iOS ARM64,
formatting, text/TOML/Nix policy, supply-chain lanes, 245/245 workspace Nextest
tests and 85/85 KMP-compatible crypto tests. Nix omitted incompatible
`x86_64-linux` outputs on Darwin; hosted CI remains the Linux evidence gate.

## Focused coverage

LLVM source-based coverage was collected deterministically over the complete
`identus-did` suite. The changed security paths exceed the issue's 85% line
coverage target:

| Source | Lines | Regions | Functions |
| --- | ---: | ---: | ---: |
| `wire_json.rs` | 93.18% | 92.65% | 83.33% |
| `document.rs` | 92.32% | 91.24% | 87.67% |
| `uri.rs` | 91.38% | 88.98% | 91.30% |

The complete measured workspace subset recorded 92.61% line and 92.03% region
coverage. Raw profiles and the generated report were ephemeral local evidence;
the reproducible test sources are committed.

## Performance and allocation-shape evidence

The release diagnostic scanned and parsed 100,000 representative DID
documents in 1.564248041 seconds, approximately 63,928 documents per second on
this host. This is observational evidence, not a portable threshold.

The scanner makes two bounded CPU passes over raw input and retains decoded
names only for currently open objects. Its explicit ceilings are 256 KiB raw
input, 64 container levels, 16,384 values, 128 members per object and 128 KiB
of simultaneously live decoded name bytes.

## Conformance and dependency evidence

- 1,000 deterministic scheme/authority/path/query/fragment combinations were
  compared with NeoPRISM's pinned `uriparse` 0.6.4 oracle.
- Seven invalid URI classes and three explicitly classified profile/oracle
  differences are pinned, including the minimized non-unwinding regression.
- Seven security-relevant duplicate-map locations, decoded-name equality,
  object-local reuse, every scanner limit, malformed/trailing JSON and
  caller-data redaction are covered.
- 256 deterministic DID documents exercise contexts, aliases, methods,
  relationships, three endpoint shapes and extension trees across native and
  unique-name JSON round trips.
- `cargo tree -p identus-did --edges normal` excludes `uriparse`; the exact
  package appears only under `--edges dev`.

Sanitizer cargo-fuzz remains issue #35, and resolution-envelope scanner reuse
remains issue #41. Neither is silently claimed by this change.

## Repository boundary

Final read-only receipts equal the issue's preflight receipts:

- NeoPRISM: `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`, clean.
- midnight-identity: `427f8571950c42967a18726cbcbefecc19ef8d79`,
  pre-existing modified `third_party/midnight-did` submodule.
- Lace ID Portal: `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  pre-existing untracked `.pi-subagents/`, `.pi/` and `tmp/`.
- Oxid: `bfe3b481568dc738f0732c2b27548fab8721fd95`, pre-existing
  untracked `.claude/` and `.pi/taskflows/`.
- Apollo: `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`, pre-existing
  modified `secp256k1-kmp/native/secp256k1` submodule.

No donor or consumer repository, SDK `main`, live repository setting,
publication surface or release state changed. SDK `main` remains
`2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.

## Readiness receipt

Generated after the signed implementation head passed `factory receipt`:

```text
Change: harden-did-document-wire-boundary
Branch: codex/idr-005d-did-wire-hardening
Head SHA: 41a85cbe8522a98d9e4cda632012e9a8e2b30567
Develop merge base: 87ecd9cb970da0e8b3938c62b1eddc8ce0b73995
Factory contract: passed
Product-specific gates: not asserted; attach their outputs separately
```
