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

## Candidate mechanics

The repository's candidate tool creates a temporary three-member workspace,
rewrites only the candidate manifests to `0.1.0-rc.1`, uses exact internal
requirements, and assembles every Cargo archive twice. It verifies normalized
contents, extracted archive closure, feature profiles, public API, SemVer,
SBOMs, checksums, tool versions, source identity, and limitations.

This is deliberately **not** publication. The canonical descriptor states
`publication = "prohibited"`, and the normal manifests remain unpublished.

Read [ADR 0113](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0113-prepare-isolated-unpublished-crypto-candidate.md)
and the [candidate descriptor](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/release/crypto-candidate.toml)
for the executable contract.
