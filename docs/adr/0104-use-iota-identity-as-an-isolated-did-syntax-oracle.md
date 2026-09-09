# ADR 0104: use IOTA Identity as an isolated DID syntax oracle

- **Status:** Accepted
- **Date:** 2026-09-09
- **Issue:** [#235](https://github.com/hyperledger-identus/sdk-rust/issues/235)
- **Parent decision:** ADR 0069 and issue #164

## Context

The SDK needs independent parser evidence without shipping a second DID model.
Published `identity_did 1.5.1` provides a mature comparison implementation but
pulls its wider identity core/JOSE model and has no declared MSRV.

## Decision

Retain exact 1.5.1 in a separately locked, unpublished DID/DID URL fixture as
manual reference evidence only. Do not add it to CI or any root/runtime graph.
W3C DID Core 1.0 and RFC 3986 remain normative. Production adoption is
excluded.

The 30-case corpus finds seven useful, classified differences, but
`identity_did` wraps the same `did_url_parser 0.3.0` already exercised by the
SDK's broader differential harness. Its 188-package all-target lock, unsafe
reach, four unmaintained dependencies, one unsoundness advisory and failing
WASM compile do not justify a second automatic syntax oracle.

## Required evidence

- immutable release/checksum/license provenance and exact graph;
- normatively classified intersections, shared rejections and divergences;
- redacted output plus unsafe/native/advisory evidence;
- host/WASM/iOS/Android compile observations;
- root/release isolation and explicit refresh/removal trigger.

## Consequences and rollback

No public behavior changes and no CI cost is added. The frozen fixture keeps
the decision reproducible without legitimizing the dependency graph. Refresh
only when a new issue proves a narrower graph, clean denied-advisory result and
WASM compilation; remove it if it stops providing reproducible evidence.
Rollback removes only the fixture, script and report.
