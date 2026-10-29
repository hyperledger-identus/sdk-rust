# Design

## Decision

`crates/crypto/tests/fixtures/crypto/v1/vectors.json` becomes the canonical
packet. The Rust test resolves it relative to its own crate. Repository-wide
catalog, quality, Nix, and mutation evidence point to the same path.

```text
canonical crate-local packet
        |-- include_str! compatibility test
        |-- conformance catalog + mutation checker
        |-- quality-evidence declaration
        `-- cargo package -> extracted candidate test
```

The candidate tool already extracts the `.crate` files and runs
`cargo test --locked -p identus-crypto --all-features`. The regression therefore
strengthens the preparation tests to assert both package membership and success
without copying any repository fixture into the verification workspace.

## Invariants

- The pre- and post-move SHA-256 values are identical.
- No second `vectors.json` with packet id `crypto.compat.v1` exists.
- Candidate staging rejects symlinks and unapproved files as before.
- The package remains deterministic and the public API baseline is unchanged.
- Production code does not load the fixture.

## Risks

Moving a documentation fixture into a crate can make discoverability worse.
The catalog remains the human-facing index and records the exact crate-local
path, authorship, license, targets, and digest. A package-size increase is
intentional and bounded by the existing archive limit.
