# Verification receipt

Verification date: 2026-10-28
Issue: #545
Develop base: `f0edaf820b3bc12b45faae02b60c55db3d3040ff`
Planning contract: `d3a17eb3baf74652045f823dc988b20eeaf70711`
Preimplementation receipt: `9cfbcdfe860b2ee47ed2704e00d302ec6ce912c8`
Evidence source head: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`

## Focused protocol and E2E evidence

- `cargo test -p identus-oid4vp --all-features --test
  verifier_authorization`: passed all five clean-room integration cases.
- The scheme matrix covers the no-prefix pre-registered form plus
  `openid_federation`, `decentralized_identifier`, `verifier_attestation`,
  `x509_san_dns` and `x509_hash`. Unsupported and ambiguous first-colon forms
  fail closed.
- The E2E path verifies the Request Object, retains the exact verified JWK,
  authorizes the verifier through a caller port, asserts the signed direct-post
  destination, consumes the evidence and produces typed routing state.
- Metadata source rules, verifier-information references, protected-header
  trust-chain bounds, payload-substitution rejection, independent collection
  limits, string-versus-DCQL limit ownership, mismatch cases, redaction
  canaries and Send/Sync contracts pass.
- JOSE compact-conformance tests prove that the protected `jwt` evidence is
  bounded and retained without changing signature verification semantics.

## Fuzz and performance evidence

- `request_object_security` passed a 300-second ASan soak: 410,999 executions,
  1,569 new corpus units, 663 MiB peak RSS under a 1,024 MiB limit, with no
  crash or timeout.
- `direct_post_response`, `encrypted_direct_post_response` and
  `request_object_security` each passed a 4,096-run smoke after the review
  remediation.
- `cargo bench -p identus-oid4vp --bench request_object_security` measured a
  91,922 ns ordinary median and a 441,141 ns near-limit median across 21
  samples of 20 iterations. Both paths include verifier authorization.
- The corrected exact implementation head passed a 4,096-run
  `request_object_security` smoke with 3,130 covered edges, 5,246 features,
  173 MiB peak RSS and no crash or timeout.
- The quality registry binds property, fuzz, benchmark and differential
  obligations to exact source revisions; the generated quality plan is in
  sync.

## Workspace and factory evidence

- Full workspace all-feature and no-default-feature tests passed.
- Workspace all-target/all-feature Clippy passed with `-D warnings`.
- Workspace rustdoc passed with `-D warnings`.
- `scripts/factory check`: all 116 checks passed, including dependency cone,
  platform ledger, resource boundary, quality evidence and OpenSpec mutation
  guards.
- `nix flake check --print-build-logs`: passed on the local Darwin system. The
  release-profile nextest derivation ran 1,090 tests: 1,090 passed, 23 declared
  skips; all build, Clippy, rustdoc, MSRV and factory derivations passed.
- After the two explicit-limit corrections, focused tests and all-target,
  all-feature OID4VP Clippy passed with `-D warnings`; the exact implementation
  head also passed the final Codex CLI review with no actionable defects.

## Compiler, portability and dependency evidence

- Locked Rust 1.98.1 release builds passed for the host workspace and the
  portable `identus-oid4vp` surface on `wasm32-unknown-unknown`,
  `aarch64-apple-ios` and `aarch64-linux-android`.
- The Nix MSRV lane passed locked Rust 1.89.0 compile, build and no-run test
  evidence, including iOS, Android and WASM core targets.
- OID4VP adds only direct edges to existing workspace crates
  `identus-crypto` and `identus-did`; no new external package enters the
  production graph.

These are SDK source, compile, test, fuzz and benchmark claims. They are not
claims of production trust policy, live federation/DID/X.509 interoperability,
mobile/browser packaging, certification, downstream adoption or release.
Protected Linux CI must still validate the exact PR head.
