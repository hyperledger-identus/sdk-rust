# ADR 0012: compose DID methods with an immutable registry

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** issue #44, child of #5 / IDR-006
- **Related work:** #43, #10, #41 and #45–#47

## Context

The SDK has validated DID values and object-safe resolution/dereferencing ports
but no chain-neutral composition point for multiple DID methods. Downstream
repositories consequently inject or expose one resolver at a time. A wallet
supporting PRISM, Midnight and generic methods needs deterministic exact routing
without owning another copy of the SSI abstraction.

## Decision

1. `identus-did` owns `DidMethodBinding`, `DidMethodRegistryBuilder` and
   `DidMethodRegistry`.
2. A binding requires a resolver and may independently provide a dereferencer.
3. The builder rejects duplicate ownership and more than 64 methods. The built
   registry shares an immutable `BTreeMap` through `Arc` for deterministic,
   lock-free concurrent reads and cheap clones.
4. The registry implements the existing query ports. Exact unknown methods map
   to W3C `methodNotSupported`; missing dereferencing on a known method maps to
   `featureNotSupported`.
5. Setup failures use a redaction-safe `RegistryError` under stable code
   `did.invalid_method_registry`; invoked queries retain one W3C result channel.
6. Introspection exposes names and capability support, not adapter objects.
7. Runtime mutation/plugins, fallback, cache/clock, algorithms, HTTP and DID
   Registration remain #45–#47 and #10 or later independently reviewed work.

## Consequences

- NeoPRISM and Midnight implementations can coexist without importing their
  method/VDR types into the SDK.
- Lace and Oxid can inject the same `DidResolver` seam whether composition has
  one method or many.
- Registry reconfiguration constructs a new value; no request observes partial
  mutation or precedence changes.
- Dynamic plugin systems must live outside this deterministic foundation and
  publish a stable snapshot into it.

## Provenance

NeoPRISM `d6ad1ec`, midnight-identity `427f857`, Lace ID Portal `804de0a` and
Oxid `bfe3b48` were inspected as immutable evidence. Apache-2.0 provenance is
present for NeoPRISM, midnight-identity and Oxid; Lace remains evidence-only.
No donor source or fixture is copied.

## Rollback

Revert issue #44's pull request. The API is unpublished, owns no persisted
state and changes no downstream repository.
