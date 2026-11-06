# Qualify the SD-JWT engine boundary

## Why

Milestone A3 needs selective-disclosure issuance, presentation, key binding,
and verification without inheriting an obsolete draft, a broad SSI framework,
or a dependency that violates the SDK's Rust 1.89.0 release floor. RFC 9901 is
stable, while SD-JWT VC is a separate profile that advanced from draft-19 to
draft-22 on 2026-10-27. The repository therefore needs a current, reproducible
build-versus-adopt decision before implementation.

## What changes

- Pin RFC 9901 and SD-JWT VC draft-22 independently by immutable content hash.
- Assess maintained Rust candidates at exact releases for protocol currency,
  behavior, dependency cone, MSRV, licenses, advisories, unsafe/native code,
  targets, maintenance, resource limits, diagnostics, and secret handling.
- Select a private dependency or a local engine behind Identus-owned types for
  the RFC 9901 base without making a candidate crate part of the public API.
- Define a separately versioned SD-JWT VC profile boundary and keep legacy
  `vc+sd-jwt` handling outside the canonical `dc+sd-jwt` core.
- Define official, clean-room, differential, cross-language, fuzz, property,
  benchmark, and offline end-to-end evidence required by implementation.
- Create bounded implementation issues; do not add a production dependency or
  runtime behavior in this research change.

## Capabilities

### Added capabilities

- `sd-jwt-engine-selection`: standards pins, dependency dispositions,
  architecture boundary, activation gates, and implementation evidence plan
  for RFC 9901 and SD-JWT VC.

## Non-goals

- No SD-JWT parser, issuer, holder, verifier, VC profile implementation,
  production dependency, public API, wire behavior, release, certification,
  downstream mutation, trust policy, key custody, HTTP retrieval, or UI.
- No claim that compile-only WASM, iOS, or Android evidence proves runtime,
  packaging, FFI, or device support.
- No migration of SDK-TS DTOs or errors into Rust domain types.

## Delivery

Issue #489 owns this research and the material build-versus-adopt decision.
The OpenSpec research and constraints pass readiness before canonical research,
ADR, roadmap, vector-plan, and child-issue records are added. A distinct local
architecture/security/dependency review, signed and DCO-bearing PR to
`develop`, and green required CI are mandatory.
