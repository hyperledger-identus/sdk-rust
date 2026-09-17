# Adoption before publication

SDK-Rust currently offers a source-only evaluation channel. A consumer pins the
full commit reachable from protected `develop`, chooses features explicitly,
and commits its lockfile.

```toml
identus-crypto = {
  git = "https://github.com/hyperledger-identus/sdk-rust",
  rev = "FULL_40_HEX_REVIEWED_COMMIT",
  default-features = false,
  features = [ "hash" ],
}
```

The placeholder must be replaced with the exact reviewed revision. Do not use
`develop`, a tag that does not exist, a pull-request ref, or a short SHA.

## Adoption receipt

Record:

- old and new exact SDK revisions;
- selected packages and features;
- compiler and tested targets;
- Cargo and Nix lock/hash changes;
- dependency/feature-tree delta;
- unit, conformance, integration, advisory, and license results;
- public/wire compatibility and rollback.

The first planned M3 consumer is a minimal hash-only downstream canary. It is
owned and implemented entirely downstream: this repository accepts only
generic evidence and does not import consumer names, primitives, or policy.
NeoPRISM provides a broader source-pinned cryptography integration reference.

Source evaluation provides no crates.io checksum, SemVer support promise,
stable MSRV, LTS period, certification, or production warranty.
