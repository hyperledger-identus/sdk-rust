# Verification

Verification date: 2026-09-25

## Exact identities

- Issue: #376.
- Develop base: `149a35e62fc54f91dc99a496feb884d5b2f92cce`.
- Planning commit: `f8e77570dfbb5c84a1c2434193dadfe5638b5feb`.
- Preimplementation receipt commit:
  `93a724fb24d3b33367cce2eefc6e3ca579f5da25`.
- Reviewed implementation head:
  `b76ff21b10e89b7006c8a005b1ab8c1424b32d59`.
- The receipt binds issue #376 and the exact planning head before any fixture or
  executable-test implementation.

## Focused and contract evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --test generic_interoperability_vectors`:
  3 passed.
- `cargo test --locked -p identus-oid4vci --all-features`: passed.
- `cargo test --locked -p identus-oid4vci --no-default-features`: passed.
- `scripts/check-oid4vci-conformance.py .`: 24 rows passed
  (`implemented=15`, `partial=5`, `unsupported=4`, `missing=0`).
- `scripts/check-ssi-upstream-backlog.py`: 30 rows passed.
- `scripts/factory backlog-live`: 30 backlog rows, 24 conformance rows and 9
  live issues passed.
- `git diff --check`: passed.

## Workspace, documentation and factory evidence

- `cargo test --locked --workspace --all-features`: passed; only declared
  diagnostic tests were ignored.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`:
  passed for the complete workspace.
- `./bootstrap.sh --check`: passed, including 86 OpenSpec/factory items and 55
  factory self-tests.
- `cargo tree --locked -p identus-oid4vci --edges normal --depth 2`: inspected;
  the slice changes neither `Cargo.toml` nor `Cargo.lock`.

## Portable-target evidence

The public crate graph checks with the pinned lockfile on all three relevant
targets:

- `cargo check --locked -p identus-oid4vci --target wasm32-unknown-unknown`:
  passed.
- `cargo check --locked -p identus-oid4vci --target aarch64-apple-ios`: passed.
- `cargo check --locked -p identus-oid4vci --target aarch64-linux-android`:
  passed.

## Read-only consumer receipt

No consumer command wrote or staged data. Oxid remained at
`183664aeca500c25d6d27a22fa402b4d40c649d3` with its four pre-existing status
entries. Lace ID Portal remained at
`d284b85bfcbb4a7e5d2200837703419c145c60f5` with its pre-existing `.pi/`
entry. The manifest pins earlier immutable evidence revisions and explicitly
records `importedBytes: false`.

## Routed evidence and exclusions

The protected pull request must still supply the independent Linux `fast`
gate for its exact candidate head. Fuzz/conformance, portable-target and
security lanes remain appropriate slow evidence; they are not duplicated as
required PR matrices. No live network, issuer application, human approval,
certification, consumer adoption, publication or product-level
interoperability claim was run or inferred.

## Hosted remediation evidence

The exact hosted `fast` run at candidate `6887fff3a27cbdaab5d005a0b1721a21a04c3ffd`
failed in `factory-contract` because its mutation test attempted to parse the
newly delivered row's `followup_issue=none` as an integer. The production
checker and live audit were green.

After making that test lifecycle-aware:

- `python3 scripts/tests/ssi-upstream-backlog-live.py`: 8 passed.
- `python3 -m py_compile scripts/tests/ssi-upstream-backlog-live.py`: passed.
- `scripts/tests/factory-contract.sh`: complete mutation suite passed.
- `git diff --check`: passed.

The next exact hosted candidate must still pass `fast`; no rerun or bypass of
the failed revision is accepted.

The next candidate passed the factory remediation and every non-test Nix gate,
then reported 865 passed and three failed tests. Each new interoperability test
failed at the same precondition because `manifest.json` was absent from the
Nix cleaned Cargo source. The Nix filter and `rust-source-contract` now retain
and assert only the exact ten-file `interop-v1` subtree. Before another hosted
candidate:

- `nix build .#checks.aarch64-darwin.rust-source-contract`: passed.
- `nix build .#checks.aarch64-darwin.rust-test`: 868 passed, 22 skipped; the
  three interoperability tests passed from the cleaned source.
- `nix build .#checks.aarch64-darwin.lint-nix`: passed after canonical Nix
  formatting.

The failed run remains immutable evidence and is not rerun.
