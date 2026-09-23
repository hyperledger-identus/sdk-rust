# The first release train

The proposed first candidate is a three-package closure at `0.1.0-rc.1`:

```text
identus-crypto =0.1.0-rc.1
    ├── identus-core =0.1.0-rc.1
    └── identus-derive =0.1.0-rc.1

identus-core =0.1.0-rc.1
    └── identus-derive =0.1.0-rc.1
```

`identus-derive` is included because it generates foundational type behavior at
compile time. `identus-core` gives the crypto crate stable chain-neutral error,
metadata, URL, and clock contracts. `identus-crypto` is the independently useful
consumer capability that justified preparing the closure first.

## Why not release the whole monorepo?

The workspace also contains implemented experimental components and
quarantined placeholders. Releasing all of them together would convert package
presence into accidental API, support, and namespace promises. The isolated
train keeps the first review cohesive and leaves DID, credentials, protocols,
bindings, and wallet ports on their own evidence-driven timelines.

## Candidate and publication mechanics

The three canonical package manifests carry `0.1.0-rc.1`; every unrelated
workspace member retains the `0.0.0`, `publish = false` default. The candidate
tool creates a temporary three-member workspace, uses exact internal
requirements, and assembles every Cargo archive twice. It verifies normalized
contents, extracted archive closure, feature profiles, public API, SemVer,
SBOMs, checksums, tool versions, source identity, and limitations.

Candidate preparation is deliberately **not** publication. The descriptor
states `publication = "release-gated"`: upload occurs only from a signed exact
tag through the protected `crates-io` environment, with an independent
maintainer approval and an immutable publication receipt. The namespace-
creating train uses a short-lived bootstrap token; every later train uses
crates.io trusted publishing through OIDC.

The credential-bearing job repeats the signed-tag, exact-SHA, and protected
`develop` ancestry checks after environment approval. Candidate artifacts are
scoped to the workflow run rather than an individual attempt, so a failed job
can be retried without rebuilding or substituting evidence. Existing packages
and release assets are reused only when their checksums and release identity
match exactly; disagreement fails closed.

Read [ADR 0134](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0134-activate-protected-crates-io-release-trains.md),
[ADR 0113](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0113-prepare-isolated-unpublished-crypto-candidate.md),
and the [candidate descriptor](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/release/crypto-candidate.toml)
for the executable contract.
