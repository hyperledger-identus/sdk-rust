# Establish canonical language-adapter mappings

## Why

SDK-Rust must become the canonical domain and protocol implementation without
forcing existing TypeScript, Swift, or Kotlin callers to adopt Rust-native DTO
and error shapes in one step. Today there is no machine-readable contract that
distinguishes canonical Rust semantics from a bounded legacy language facade,
or that records loss, unsupported behavior, deprecation, rollback, and the
vectors proving a conversion.

Issue #505 establishes that boundary before any consumer mutation. The first
records describe generic DID/DID URL values and their stable errors for the
pinned SDK-TS baseline. They document compatibility; they do not implement a
TypeScript adapter or change canonical Rust types.

## What changes

- Add a closed, versioned TOML registry for canonical Rust-to-language DTO and
  error mappings.
- Seed DID, DID URL, invalid-DID, and invalid-DID-URL mappings for the pinned
  SDK-TS legacy surface, including explicit loss and unsupported cases.
- Add a strict offline validator, mutation tests, and deterministic Markdown
  rendering for engineering review.
- Integrate the mapping contract and generated documentation into the factory
  gate.

## Capabilities

### Added capabilities

- `language-adapter-mappings`: govern versioned compatibility between
  canonical Rust contracts and deprecatable language-specific facades.

### Modified capabilities

None. `identus-did` types and public error codes remain unchanged.

## Non-goals

This change does not add bindings, FFI, WASM exports, TypeScript/Swift/Kotlin
code, peer DID, protocol behavior, consumer builds, or runtime dependencies.
It does not promise that every legacy DTO shape is lossless or permanently
supported.

## Delivery

Issue #505 owns this change under parent #504 and milestone A1. It can merge
independently of #420 while referring to #420's stable vector-ID vocabulary.
Issue #422 consumes both contracts. SDK-TS canary #492 remains downstream.
