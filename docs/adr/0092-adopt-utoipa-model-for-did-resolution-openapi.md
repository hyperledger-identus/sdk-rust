# ADR 0092: adopt the Utoipa model for DID Resolution OpenAPI

- **Status:** Accepted
- **Date:** 2026-09-08
- **Decision authority:** sdk-rust issue #205 under #10
- **Normative profile:** OpenAPI 3.1.0 and the pinned W3C DID Resolution GET binding
- **Dependency:** `utoipa 5.5.0`, default features disabled

## Context

The SDK's fixed Axum DID Resolution route implements bounded negotiation,
options and W3C response projection. NeoPRISM demonstrates demand for an
optional OpenAPI artifact, but its older macro-derived document couples DID
Core to Utoipa and describes behavior that predates the SDK's strict adapter.

A handwritten JSON document would avoid one package while replacing a typed
standard model with unchecked string-key construction. Utoipa's current model
can be used without its generator macros and adds only its own package to the
existing resolved serde/serde_json/indexmap cone.

## Decision

1. Add exact `utoipa 5.5.0` as an optional dependency of
   `identus-did-resolver-http`, with default features disabled.
2. Construct the document through Utoipa's typed public OpenAPI model; do not
   enable `utoipa-gen` or annotate `identus-did`.
3. Expose one feature-gated function returning `utoipa::openapi::OpenApi`.
   Third-party type exposure is limited to this already framework-specific
   outer adapter and permits direct host-document composition.
4. Describe only fixed `GET /{did}` behavior implemented in the same crate:
   common options, extension semantics, exact bounds, representations, status
   outcomes and `Vary: Accept`.
5. Keep the path mount-relative. Hosts own prefix composition; the SDK accepts
   no dynamic path input.
6. Guard semantic drift with deterministic serialized-structure tests and
   default/feature dependency checks.

## Consequences

- Hosts can reuse one typed document instead of copying NeoPRISM annotations.
- Default consumers and DID Core retain their current narrow graph.
- Feature users deliberately couple to Utoipa 5.x until the pre-release API is
  stabilized; a major Utoipa update requires focused compatibility review.
- Method-specific query option names remain open-ended and are described rather
  than falsely enumerated.

## Alternatives rejected

### Copy NeoPRISM's macro and derive pattern

This would add a proc-macro cone, spread OpenAPI annotations into DID Core and
carry an outdated query contract.

### Return handwritten JSON

This removes one small package but makes OpenAPI shape and required fields an
SDK-maintained string convention instead of using a maintained typed model.

### Accept a dynamic route path

The runtime intentionally returns a fixed state-closed router. A separate
documentation path input would need its own grammar, bounds and synchronization
policy while weakening composability.

## Verification and rollback

Acceptance requires feature/default Cargo graph inspection, deterministic
structure tests, exact response/parameter coverage, strict Clippy, workspace
tests/docs, factory checks and hosted CI. Rollback removes the optional feature,
dependency and function without changing runtime or stored data.
