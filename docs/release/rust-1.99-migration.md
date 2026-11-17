# Rust 1.99 source migration

The SDK-Rust `develop` source line moves from the published experimental
`0.1.0-rc.1` compiler contract to an unpublished `0.2.0-rc.1` source identity.
Exact Rust 1.99.1 is now the minimum supported Rust version, primary compiler,
and compatibility etalon for this source line.

This document is migration guidance, not a release announcement. It does not
authorize a tag, crates.io publication, `main` promotion, stable API claim, or
downstream repository mutation. The published `0.1.0-rc.1` artifacts and their
Rust 1.89.0 floor remain immutable historical evidence.

## Consumer action

Source consumers adopting a commit after ADR 0200 must:

1. install exact Rust 1.99.1 or consume the repository Nix toolchain;
2. refresh the SDK Git revision and `Cargo.lock` together;
3. update exact prerelease requirements for `identus-derive`, `identus-core`,
   and `identus-crypto` from `=0.1.0-rc.1` to `=0.2.0-rc.1` when consuming the
   repository source closure;
4. run the feature and target profiles used by that consumer; and
5. retain the old immutable revision as the rollback target until its own
   integration and end-to-end evidence passes.

Consumers that must remain on Rust 1.89 through 1.98 cannot adopt the `0.2.x`
source line. They should keep the reviewed `0.1.0-rc.1` registry artifacts or a
previous exact Git revision. The project does not maintain a second compiler
lane for the new source line.

## Compatibility boundary

The migration changes the compiler requirement and prerelease source identity.
It does not intentionally change SDK runtime behavior, public Rust item shapes,
wire formats, persistence formats, target tiers, or language-adapter behavior.
Any such change remains independently governed and reviewed.

## Rollback

Before any future `0.2` publication, rollback is a signed revert of ADR 0200,
the toolchain and policy pins, the three source manifest versions, and the
associated lock changes. A downstream rollback selects its previously reviewed
full SDK commit and restores the corresponding lockfile. No registry cleanup is
needed because this migration does not publish an artifact.
