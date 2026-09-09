# ADR 0104: use IOTA Identity as an isolated DID syntax oracle

- **Status:** Proposed
- **Date:** 2026-09-09
- **Issue:** [#235](https://github.com/hyperledger-identus/sdk-rust/issues/235)
- **Parent decision:** ADR 0069 and issue #164

## Context

The SDK needs independent parser evidence without shipping a second DID model.
Published `identity_did 1.5.1` provides a mature comparison implementation but
pulls its wider identity core/JOSE model and has no declared MSRV.

## Proposed decision

Evaluate exact 1.5.1 in a separately locked DID/DID URL syntax fixture. Retain
it only as an oracle if the bounded corpus provides meaningful independent
regression value at acceptable maintenance cost. W3C DID Core 1.0 and RFC 3986
remain normative. Production adoption is excluded.

## Required evidence

- immutable release/checksum/license provenance and exact graph;
- normatively classified intersections, shared rejections and divergences;
- redacted output plus unsafe/native/advisory evidence;
- host/WASM/iOS/Android compile observations;
- root/release isolation and explicit refresh/removal trigger.

## Consequences and rollback

No public behavior changes. The oracle may increase slow conformance cost but
cannot enter fast/runtime graphs. Rollback removes only the fixture, script and
report. This ADR becomes Accepted with the final disposition or Superseded if
the executable evidence does not justify retention.
