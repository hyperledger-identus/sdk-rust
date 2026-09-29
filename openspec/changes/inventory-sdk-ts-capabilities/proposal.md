# Inventory SDK-TS capabilities

## Why

SDK-TS is the most current Identus language SDK and the strongest starting
point for discovering presently shipped wallet capabilities. The existing
program survey is too broad to drive implementation: it groups behavior under
the historical Apollo, Castor, Pollux, Mercury, Pluto, and EdgeAgent labels,
does not distinguish complete behavior from draft or dependency-specific
behavior, and does not yet identify exact public, wire, persistence, runtime,
test, and consumer evidence.

SDK-TS is implementation evidence, not a specification. Its SD-JWT,
AnonCreds, DIDComm, OID4VCI, storage, and cryptography dependencies must not be
copied automatically. Accepted standards, current profiles, official vectors,
maintained Rust libraries, and SDK-Rust security boundaries remain higher
authorities.

## What changes

- Pin and inventory `hyperledger-identus/sdk-ts` release 8.1.4 at
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`.
- Record its npm exports, browser/Node runtime assumptions, plugin host,
  public/domain APIs, wire and persistence formats, dependencies, tests,
  fixtures, known consumers, and compatibility surfaces.
- Replace historical mythological module groupings with responsibility-based
  capability identifiers. Legacy names remain only as donor/source aliases
  and temporary language-SDK compatibility labels.
- Give every inventoried capability a preliminary disposition and evidence
  authority under ADRs 0162 through 0164.
- Normalize SD-JWT against RFC 9901 and a separately pinned current SD-JWT VC
  profile, and normalize AnonCreds against the current AnonCreds 1.0 profile
  and maintained Rust implementation options.
- Make SDK-TS discovery the input to the later SDK-Swift comparison; defer the
  older SDK-KMP inventory until TS and Swift establish the baseline and
  deviations.
- Select and specify one reversible non-secret Rust-backed canary without
  changing SDK-TS in this issue.

## Capability

### Modified capability

- `identus-platform-core-migration`: require ordered SDK-TS-led discovery,
  responsibility-based target naming, and current-standard/upstream
  normalization before language-SDK behavior becomes SDK-Rust backlog.

## Non-goals

This change does not edit SDK-TS, add a Rust runtime dependency, expose a new
binding, implement a missing feature, claim package/runtime support, remove a
legacy language-SDK API, deprecate a repository, or copy unreviewed fixtures.
It does not treat file counts, coverage, repository recency, or one passing
implementation as conformance evidence.

## Delivery

Issue #417 owns the discovery. Planning, research, constraints, delta
requirements, and semantic review are committed before inventory artifacts are
implemented. The completed change will include an ADR, a reviewable report, a
machine-readable SDK-TS capability inventory, roadmap updates, bounded child
issues, and a canary issue ready for its own OpenSpec preflight.
