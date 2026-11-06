# ADR 0193: build a focused RFC 9901 engine behind Identus types

- **Status:** Accepted for issue #489 qualification
- **Date:** 2026-10-28
- **Decision authority:** issue #489 and milestone A3 under ADRs 0004, 0061, 0076, and 0169
- **Normative base:** RFC 9901
- **Profile pin:** `draft-ietf-oauth-sd-jwt-vc-22`

## Context

SDK-Rust needs RFC 9901 issuance, disclosure selection, presentation, key
binding, and verification plus a separately versioned SD-JWT VC profile. The
repository already owns bounded JOSE, crypto, credential, presentation, and
protocol seams, but no SD-JWT proof engine.

The current ecosystem offers useful independent implementations, but no exact
release passes every SDK gate:

- `affinidi-sd-jwt 0.1.8` is the closest cohesive RFC 9901 base and passes 106
  upstream tests plus a clean current RustSec scan, but requires Rust 1.95,
  exposes full credentials through Debug/Display, interpolates correlating data
  into errors, and does not bound the complete input/work surface;
- `sd-jwt-payload 0.5.1` is a useful payload oracle but replaces only part of
  the required behavior while resolving 46 packages even without its default
  hashing feature;
- `ssi-sd-jwt 0.6.0` resolves 218 packages, contains authored unsafe
  constructors, and its exact graph is rejected for current unsound and
  unmaintained RustSec entries;
- `reallyme-sd-jwt 0.4.1` demonstrates a strong bounded design but requires
  Rust 1.99 and a separate ReallyMe JOSE/crypto/proto stack;
- other assessed candidates are draft-era, stale, hard-wire one algorithm,
  include native/network/framework coupling, or use an incompatible license.

SDK-TS 8.1.4 remains migration evidence only. Its `vc+sd-jwt`, millisecond-time
tolerance, TypeScript DTOs, errors, DID coupling, and task runtime do not define
the Rust contract.

## Decision

1. Implement the stable RFC 9901 base as a focused, independently selectable
   Identus crate over existing `identus-jose`, `identus-crypto`, and injected
   effect ports. Do not import a historical component name.
2. Keep candidate crates out of the production graph. Use Affinidi, IOTA,
   Spruce, ReallyMe, and selected non-Rust implementations only as independent
   differential oracles with immutable versions.
3. Keep RFC 9901 and SD-JWT VC independently versioned. The first VC profile
   targets exact draft-22 and cannot silently follow a later draft or RFC.
4. Use canonical `dc+sd-jwt` output. Any `vc+sd-jwt` acceptance requires a
   separately versioned, opt-in, time-bounded compatibility adapter with a
   named consumer and removal trigger.
5. Expose only Identus-owned staged parsed, disclosure-processed, and verified
   states. Candidate types, raw signing keys, trust policy, storage, transport,
   consent, and custody never enter the public format boundary.
6. Require explicit byte/count/depth/work/retained-output limits, static
   redaction-safe errors, and official/negative/property/fuzz/benchmark/
   differential/offline-E2E evidence before the production capability merges.
7. Split implementation into dependency-ordered base, profile, and optional
   legacy-adapter issues. A successful base does not claim profile or migration
   completion.

## Consequences

The SDK accepts a small amount of standards-specific maintenance to preserve
its compiler floor, cohesion, security boundaries, and target policy. The
implementation can reuse existing tested JOSE and crypto capabilities without
duplicating general-purpose cryptography. External engines still reduce
conformance risk through differential evidence and can replace private
mechanics later without a public migration.

The SD-JWT VC draft may change during IESG evaluation. Exact profile metadata
and vectors make that change visible instead of silently altering behavior.

## Reconsideration

Reassess production adoption when an immutable release:

- supports the effective Rust floor and primary compiler;
- has a small, advisory-clean, permissively licensed, portable, non-native
  dependency cone;
- preserves all RFC 9901 valid and invalid behavior under SDK limits;
- exposes no candidate type or secret/correlating diagnostic across the facade;
- passes host, WASM, iOS, Android, fuzz, differential, and offline E2E gates;
  and
- demonstrates concrete correctness or maintenance deletion greater than the
  adapter and dependency cost.

Profile migration is reconsidered when draft-22 becomes an RFC or a later
revision materially changes normative behavior. Legacy `vc+sd-jwt` is
reconsidered only for a named consumer with measured migration need.

## Rollback

Before implementation, rollback removes this behavior-neutral decision. After
implementation, private mechanics may be replaced behind Identus-owned types;
wire/profile changes require an explicit capability revision and vector-backed
migration rather than an in-place reinterpretation.
