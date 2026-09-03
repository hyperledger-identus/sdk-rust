# Verification evidence

- **Issue:** #34 (child of #5 / `IDR-005`)
- **Fuzzing follow-up:** #35 (child of #5)
- **Develop base:** `49b2ca29b6d6a63a240f110fa89a2d500204152a`
- **Reviewed implementation head:** `443bfa93984459a8a789f86721ba7a7afd778b31`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Executed gates

| Command | Result |
| --- | --- |
| `cargo test -p identus-did` | passed; 16 unit plus 10 DID syntax tests, one manual diagnostic ignored |
| `cargo clippy -p identus-did --all-targets --all-features -- -D warnings` | passed |
| `cargo build -p identus-did --target wasm32-unknown-unknown` | passed |
| `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-did --no-deps` | passed |
| `cargo test --release -p identus-did --test did_syntax parser_throughput_diagnostic -- --ignored --nocapture` | passed; 500,000 parses |
| `./scripts/factory validate add-did-syntax` and `git diff --check` | passed |
| `nix flake check --print-build-logs` | passed all 26 applicable local Darwin checks |

The Nix matrix included Rust 1.85 MSRV, workspace release build, workspace
nextest (159/159), KMP compatibility nextest (85/85), WASM, Android, iOS,
clippy, rustdoc, Rust/text/TOML/Nix formatting and lint, `cargo-deny`, RustSec
audit and the factory contract. The x86_64 Linux system is incompatible with
the local Darwin evaluation and is exercised independently by hosted CI.

## Grammar, allocation and performance evidence

- The conformance suite classifies every ASCII byte in method,
  method-specific-id, path, query and fragment positions and separately tests
  valid lower/upper-case percent escapes.
- W3C example syntax plus PRISM, Midnight, web and key-shaped consumer inputs
  round-trip exactly. Those fixtures do not assert method registration or
  method-specific semantics.
- Native and serde construction agree; malformed delimiters, escapes,
  whitespace, Unicode and values above the 2/4 KiB limits fail with redacted
  errors.
- Pointer checks prove `TryFrom<String>` and `From<Did> for DidUrl` retain the
  supplied allocation. Component pointers fall inside the single owned string.
- The release diagnostic completed 500,000 representative parse-and-allocate
  operations in 87.887334 ms: approximately 5,689,102 parses/second. No
  environment-sensitive pass/fail threshold is introduced.

## Effort evidence

- **Estimate:** 2–4 agent-hours excluding hosted CI.
- **Actual through implementation, review and complete local gates:** 20
  minutes 3 seconds wall-clock (`01:24:00Z`–`01:44:03Z`).
- **Hosted CI/review/merge:** measured separately on the pull request because
  it occurs after this immutable local receipt.

## Provenance recheck

No dependency was added and no donor or downstream repository was modified.
Source revisions and license decisions remain recorded in the design, ADR and
issue. The full fuzzing maturity increment is explicit in issue #35.

Hosted CI, review, merge and final integration SHAs are recorded on the pull
request and issue because they occur after this immutable local evidence.
