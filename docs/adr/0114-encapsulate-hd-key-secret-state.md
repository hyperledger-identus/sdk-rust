# ADR 0114: encapsulate HD-key secret state before publication

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-14
- **Issue:** [#269](https://github.com/hyperledger-identus/sdk-rust/issues/269)
- **Supersedes in part:** ADR 0022's temporary public-field compatibility
- **Depends on:** ADR 0113
- **Review no later than:** before any `identus-crypto` publication candidate

## Context

ADR 0022 made SDK-owned HD buffers zeroizing and formatting-safe but retained
four public `[u8; 32]` fields for compatibility. An ordinary field read copies
those arrays beyond the owning HD value's erasure lifecycle. PR #267 made that
surface visible in the first unpublished `0.1.0-rc.1` API baseline.

No release or authorized downstream adoption depends on those fields. Keeping
them for a future migration would convert a known secret-lifecycle flaw into a
public commitment.

## Decision

1. Make the private-key and chain-code fields of `HDKey` and `EdHDKey` private.
2. Add one fixed 32-byte SDK-owned exposure value with private storage,
   `Zeroize + ZeroizeOnDrop`, redacted `Debug`, and no `Clone`, `Copy`,
   `Display`, Serde, generic dereference, `AsRef`, or binding annotation.
3. Give both HD types explicitly named private-key and chain-code exposure
   methods. Each method creates one exposure owner; raw bytes are available
   only as a borrow explicitly requested from that owner.
4. Preserve public metadata, derivation algorithms, vector bytes, errors,
   features, target policy and dependency cone.
5. Prove field opacity and excluded `Clone`/`Display` surfaces with compile-fail
   tests. Prove absent Serde and implicit raw-access surfaces through exact
   source and public-API inventory. Prove redaction, explicit erasure and exact
   outputs with runtime tests.
6. Replace the unpublished `0.1.0-rc.1` API rendering intentionally. Classify
   field removal as a Rust source-breaking change, accepted before publication
   under issue #269; it creates no wire, persisted-data or released SemVer
   break.
7. Do not add a compatibility shim returning ordinary arrays. If a future
   downstream needs legacy field-shaped access, it belongs in a separately
   authorized consumer compatibility facade, not the canonical crypto type.
8. Do not add a raw extended-state constructor in this slice. Field readers can
   migrate to the named exposure methods. Struct-literal callers may reconstruct
   only when they retain the original seed and supported derivation path;
   private-key/chain-code/metadata rehydration is an unsupported pre-release
   limitation pending a separate security and API decision.

## Copy and threat boundary

The long-lived HD owner and every SDK-created exported copy erase their own
storage on drop. The exposure borrow cannot outlive its owner. Safe formatting
of either owner is redacted, and neither owner serializes or enters generated
bindings.

This does not prevent a caller from deliberately copying, formatting or
serializing bytes after invoking the explicit raw view. It also does not claim
to erase compiler/register/allocator copies, swap, dumps or hardware state.
Those limits are part of the API contract rather than implied custody claims.

## Consequences

- Accidental field reads no longer create untracked secret copies.
- Raw access is visible in code review and its first copy has a defined erasure
  owner.
- Existing field reads do not compile and must migrate to the named exposure
  methods.
- Existing struct-literal construction does not compile and has no raw-state
  import replacement. Seed-plus-path reconstruction remains available; raw
  extended-state rehydration is unsupported and deferred.
- The facade remains small and backend-independent, with no new package,
  algorithm, unsafe/native code or feature.
- Tests and documentation become more verbose around vectors because secret
  access is intentionally explicit.

## Alternatives rejected

- **Return `zeroize::Zeroizing` directly:** it derives byte-revealing `Debug`
  and exposes the implementation dependency as the concrete facade.
- **Borrow the HD arrays directly:** no export owner and insufficiently
  explicit raw access.
- **Callback-only exposure:** viable but awkward for ordinary consumers and
  does not improve ownership of a consumer-requested export copy.
- **Deprecate fields:** cannot prevent copying and carries unsafe compatibility
  into a future release.
- **Adopt `secrecy` now:** a new generic dependency and abstraction is not
  justified for two fixed-size access paths already backed by `zeroize`.

## Verification and rollback

Focused vector, redaction, zeroization and compile-fail tests; exact source and
public-API inventory; candidate checks; supported feature/target builds;
factory/Nix gates; and a distinct security/API review verify the decision.
Rollback is a source revert; there is no released artifact or consumer
migration to unwind.
