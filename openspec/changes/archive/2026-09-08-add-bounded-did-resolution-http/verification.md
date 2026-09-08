# Verification receipt

Verification status: passed
Verification date: 2026-09-08
Base: develop@d8de711f527d585818a87effe9ff02e74238f098
Reviewed implementation head: d31e8c395f10f9e90c1b46c9a9f0bca3e88cdb30

## Behavioral evidence

- `cargo test -p identus-did-resolver-http`: 15/15 tests passed.
- The in-memory suite covers all three representations; missing, wildcard,
  quality, specificity and repeated negotiation; malformed, duplicate,
  unsupported and bounded input; exact resolver options; all nine W3C errors;
  extension error; deactivation; projection mismatch; redaction; nesting;
  one-layer path decoding; and explicit 404/405 variance.
- Complete workspace nextest: 653/653 passed with 22 configured skips.
- KMP-compat nextest: 129/129 passed with one configured skip.

## Architecture and dependency evidence

- `cargo tree -p identus-did-resolver-http -e normal` confirms inward SDK
  edges and no Tokio, Hyper, listener, socket, TLS or native package.
- The integrated adapter cone adds 17 normal external package names and two
  development-only packages. Direct versions, checksums, features, licenses,
  maintenance and transitive unsafe paths are recorded in research and ADR
  0090.
- `cargo deny --locked check`: advisories, bans, licenses and sources passed;
  only existing unused-allowance and duplicate-`syn` warnings remain.
- Nix `cargo audit --deny warnings`: 161 locked dependencies checked against
  1,242 advisories with no vulnerability.
- Source and diff searches found no production unsafe or panic path.

## Compiler, target and repository evidence

- Focused all-target strict Clippy and format checks passed on Rust 1.98.1.
- `scripts/factory check`: 50/50 active change and canonical spec items passed.
- `git diff --check` passed and all three feature commits have valid signed,
  DCO trailers.
- Complete local `nix flake check --print-build-logs` passed all 27 compatible
  aarch64-Darwin checks, including build, strict Clippy, nextest, feature
  combinations, docs, formatting, dependency, advisory, factory, file lint
  and portable core target selectors.
- Existing core selectors passed for `wasm32-unknown-unknown`,
  `aarch64-linux-android` and `aarch64-apple-ios`. The host-only HTTP crate is
  intentionally excluded from those selectors.
- x86_64-Linux was omitted as locally incompatible and remains hosted `fast`
  evidence.

## Result

All specified local gates pass at the reviewed implementation head. Hosted
Linux `fast`, policy, DCO and file-hygiene checks remain the merge authority.
