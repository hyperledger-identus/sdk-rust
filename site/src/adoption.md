# Adoption before publication

Every implemented SDK-Rust package can be evaluated from source; packages that
are not published require this channel. A consumer pins the full commit
reachable from protected `develop`, chooses features explicitly, and commits
its lockfile.

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

The current DID candidate may be evaluated at the exact protected merge that
contains its verified candidate evidence:

```toml
identus-did = {
  git = "https://github.com/hyperledger-identus/sdk-rust",
  rev = "cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16",
}

identus-did-resolver-http = {
  git = "https://github.com/hyperledger-identus/sdk-rust",
  rev = "cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16",
  default-features = false,
  features = [ "openapi" ],
}
```

Select the resolver package only when the consumer needs the Axum HTTP adapter;
select `openapi` only when it needs generated schema support. The reserved
candidate identity `identus-did-v0.1.0-rc.1` is not a created tag or a registry
receipt. Do not add a misleading `version = "0.0.0"` constraint.

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

The crypto train's registry receipt and the DID train's exact source revision
are different evidence channels. Source evaluation provides no crates.io
checksum, stable SemVer support promise, LTS period, certification, or
production warranty.
