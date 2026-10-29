# The first crypto release train

The first published prerelease is a three-package closure at `0.1.0-rc.1`:

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
train keeps the first review cohesive. DID follows its own candidate-only
train; credentials, protocols, bindings, and wallet ports remain on their own
evidence-driven timelines.

The separate [DID candidate train](did-candidate.md) reuses the released
foundation without changing this train's immutable tag or publication receipt.

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

## Published API origin

The checked-in `identus-crypto 0.1.0-rc.1` public-API baseline describes the
artifact that was actually published. Its digest is immutable; later additive
development does not rewrite release history.

Current qualification generates API with the pinned toolchain and compares it
to that origin as a line multiset. Every published occurrence must remain.
Additional lines are compatible and appear in a bounded JSON report; a removed
or changed signature fails the candidate. Baseline initialization is rejected
while the descriptor identifies `published-origin`. A compatibility rejection
retains only its diagnostics report; it does not expose candidate packages, a
publication workspace, or a success receipt.

This distinction matters: an unpublished first candidate may establish or,
with exact evidence, reconcile its origin. A published train may only compare
against its origin or select a new version through a separately approved
release slice.

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
