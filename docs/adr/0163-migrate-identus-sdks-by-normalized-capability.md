# ADR 0163: migrate Identus SDKs by normalized capability

- **Status:** Accepted
- **Date:** 2026-09-29
- **Decision authority:** issue
  [#415](https://github.com/hyperledger-identus/sdk-rust/issues/415)
- **Related:** ADR 0110, ADR 0162
- **Constraint impact:** material; replaces repository-parity and code-volume
  goals with evidence-gated capability disposition

## Context

Apollo, Castor, Pollux, Mercury, Pluto, and Agent are useful historical module
names, but their exact scope differs across TypeScript, Swift, and Kotlin. A
claim such as “Pollux parity” can hide different credential formats, error
behavior, storage assumptions, protocol versions, and tests. Migrating by file,
class, or entire repository either copies accidental structure or creates an
unsafe rewrite.

Some features should not be ported. They may be obsolete, draft-specific,
unused, superseded by a maintained library, platform-only, or inconsistent
with current security and product policy.

## Decision

Use a normalized capability as the smallest migration planning and delivery
unit. Before implementation, every capability record must identify:

- stable identifier and user outcome;
- immutable source repository/revision and current owner;
- relevant public, wire, persistence, and operational surfaces;
- specification/profile and dependency basis;
- authoritative and non-authoritative tests;
- current consumers and compatibility risk;
- target Rust crate, binding, and remaining platform owner;
- one disposition: `move-to-rust`, `retain-platform`, `replace-upstream`,
  `deprecate`, `drop`, or `defer`;
- security, privacy, resource, and maintenance evidence;
- migration phase, issue, owner, rollback, and evidence links.

Migration uses these gates:

1. **Inventory:** map source APIs, behavior, tests, dependencies, and consumers.
2. **Disposition:** decide what moves, stays, is replaced, deprecated, dropped,
   or deferred.
3. **Contract:** define Identus-owned Rust and binding types, errors, lifecycle,
   limits, and compatibility.
4. **Canary:** replace one bounded non-secret path in one consumer.
5. **Conformance:** execute shared positive, negative, boundary, mutation, and
   cross-language vectors.
6. **Capability wave:** migrate implementation and consumers with rollback.
7. **Default switch:** make the Rust-backed path default after real consumer
   evidence.
8. **Retirement review:** remove the duplicate implementation or repository
   only when its remaining responsibilities are zero or deliberately moved.

The preliminary repository disposition is:

| Repository | Default target |
|---|---|
| `sdk-ts` | thin TypeScript facade plus browser/Node packaging, plugin host, and platform adapters over WASM/native services as appropriate |
| `sdk-swift` | thin Swift facade plus Apple packaging, keychain/secure hardware, CoreData, networking, and lifecycle adapters over UniFFI |
| `sdk-kmp` | thin Kotlin facade plus Android/JVM packaging, coroutine, Ktor, SQLDelight, keystore, and lifecycle adapters over UniFFI/native artifacts |
| React Native surface | dedicated New Architecture/TurboModule package over a qualified native binding adapter; not inferred from browser WASM or Swift/Kotlin proof |

Repository retirement is a possible final disposition, not the starting
assumption. It requires no unique supported responsibilities, equivalent or
better replacement behavior, migrated consumers, release/migration evidence,
and an announced support window.

## Consequences

- Feature gaps and unnecessary legacy behavior become visible in one registry.
- Small stackable migrations can land independently and roll back cleanly.
- Similar module names no longer imply parity.
- Platform-native implementation is retained when it is the cohesive owner;
  “more Rust” is not itself a success metric.
- Inventory work precedes implementation and can intentionally conclude that a
  feature should be dropped.

## Alternatives rejected

- Migrate repository by repository: creates long branches and late compatibility
  discovery.
- Require one-for-one API parity: fossilizes accidental and obsolete APIs.
- Measure success by lines deleted: rewards movement without proving behavior.
- Port every feature before choosing dispositions: spends effort on behavior
  that should be removed or delegated upstream.

## Verification and rollback

The capability registry and per-SDK inventory must link immutable evidence and
remain reviewable without cloning the donor repository. Every implementation
issue names exactly one bounded capability or a cohesive set. Rollback restores
the previous facade route without changing persisted or wire state. A failed
canary blocks only its capability, not the whole program.
