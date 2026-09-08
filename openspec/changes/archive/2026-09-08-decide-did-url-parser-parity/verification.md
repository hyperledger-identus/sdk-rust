# Verification receipt

Verification status: passed
Verification date: 2026-09-08
Base: develop@9f0c2d7215d64dff6c3eee9844c5e049807fa0b4
Evidence head: 2e3d49165ec7c79e2d113ddcda24b047fe7dd26b

## Differential evidence

- The committed 30-case attributable corpus reproduced 7 classified
  candidate mismatches.
- The deterministic 17,284-case generator reproduced 95 mismatches: 34
  leading-byte, 35 trailing-byte, 22 percent-pair and 4 terminal-colon cases.
- Corpus assertions verify expected SDK acceptance, rejection and component
  views before comparing the candidate; the candidate is not treated as the
  conformance oracle.
- Owned-input evidence confirms SDK allocation reuse and candidate copying;
  mutation evidence confirms candidate setters can violate parse invariants.

## Candidate and dependency evidence

- Exact `did_url_parser 0.3.0` ran 19/19 upstream tests on Rust 1.98.1.
- `--no-default-features --features alloc` passed for the host,
  `wasm32-unknown-unknown`, `aarch64-linux-android` and `aarch64-apple-ios`.
- Strict candidate Clippy failed only three
  `mismatched_lifetime_syntaxes` warnings; this diagnostic is recorded and the
  candidate is not part of an SDK gate.
- Minimal normal graph: three packages including the candidate,
  `form_urlencoded` and `percent-encoding`; current advisory audit passed.
- Candidate's published development lock audit reports RUSTSEC-2026-0097 via
  old test-only `proptest 0.10.1` and `rand 0.7.3`; no SDK runtime dependency
  or lockfile was introduced.
- Crates artifact SHA-256 and byte-identical release source, license, release
  revision, signature state, unsafe use and repository metadata are pinned in
  the research record.

## Repository evidence

- Isolated harness `cargo run --locked --release`: passed with the exact
  curated and generated totals above.
- Isolated harness `cargo clippy --locked --no-deps -- -D warnings`: passed.
- `cargo fmt --all --check` and `git diff --check`: passed.
- `scripts/factory ready decide-did-url-parser-parity`: passed with research
  and constraints ready and no declared blocker.
- Complete local `nix flake check`: all 37 compatible aarch64-Darwin
  derivations passed, including compiler, format, strict Clippy, nextest,
  feature, docs, dependency, advisory, factory and portable-target checks.
  x86_64-linux was omitted as host-incompatible and remains hosted-CI evidence.
- Exact-diff review found no production crate, root manifest, release lockfile,
  API, wire-format or diagnostic change, and no tracked harness `target` file.
- All three evidence commits have valid maintainer signatures and DCO trailers.

## Result

All specified local decision gates pass at the immutable evidence head. The
archive commit will contain only review/receipt/canonical synchronization;
hosted Linux `fast`, policy, DCO and hygiene checks remain merge authority.
