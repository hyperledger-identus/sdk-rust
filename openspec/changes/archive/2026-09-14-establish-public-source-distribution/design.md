# Design: immutable public source as the alpha channel

## Boundary

The distribution contract describes how an external Rust build locates SDK
source. It does not add a package manager, release artifact, compatibility
facade or downstream adapter to the SDK.

## Decisions

1. Use the canonical public HTTPS Git URL and a full lowercase 40-hex commit in
   every Cargo dependency declaration.
2. Treat the exact commit plus the consumer's checked-in Cargo lock as the
   Cargo source identity. Nix consumers additionally retain their normal locked
   or fixed-output source evidence.
3. Publish one package table rather than duplicating near-identical snippets.
   The main example uses `identus-crypto`; the table explains the other four
   proven packages and their feature posture.
4. Keep `identus-apollo` explicitly downstream-owned as a compatibility facade.
5. Add an offline Python checker to make the documentation and manifest facts
   executable without fetching GitHub. The factory structural gate invokes it.
6. Use the already verified NeoPRISM revision as a historical canary, not as a
   moving recommended release.

## Offline checker

The checker parses repository files and fails when:

- a proven package is absent, renamed, given a non-`0.0.0` workspace version,
  or made publishable without a new distribution decision;
- the guide loses the canonical repository, full revision rule, Cargo/Nix lock
  requirements, Rust 1.98.1 floor or no-release warning;
- an example selects `branch`, `tag`, `develop`, a pull-request ref, or a short
  revision;
- the README no longer points consumers to the guide.

Focused tests exercise valid content and each negative class without network
access.

## Update procedure

A downstream upgrade records the old/new SDK SHA, selected packages/features,
consumer compiler/targets, lockfile delta and relevant test/conformance results.
The new commit must be reachable from protected `develop` and have green SDK
evidence. A source update is not inferred from a new branch head.

## Alternatives

Registry publication is the desired later distribution mechanism but crosses
namespace, release and compiler-matrix decisions. An SDK-owned Nix artifact is
unnecessary for the proven use case because NeoPRISM's Nix build already
consumes the Cargo Git dependency successfully.
