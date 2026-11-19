# Complete peer DID donor, fuzz, and differential evidence

## Why

The TypeScript peer DID slice has three confirmed evidence defects from the
review of PR #604. Its donor ledger omits the source fixture's bare service-ID
deviation, its TypeScript fuzz claim relabels a Rust-core sanitizer campaign,
and its native differential runner skips resolved-document fields and all
negative cases.

## What Changes

- Bind donor dispositions to the exact SDK-TS test selectors and record why
  bare service IDs are intentionally omitted from the adapted positive case.
- Keep the existing sanitizer soak credited only to `identus-did-peer`, and
  add a deterministic adapter-entry/projection fuzz route with its own source-
  bound evidence.
- Extend the native portable-packet runner to compare identifiers,
  relationships, service fields, and every applicable stable Rust rejection.
- Mark TypeScript-only preconditions explicitly instead of attributing them to
  the native runner.

## Capabilities

### Modified Capabilities

- `sdk-ts-peer-did-method`: make donor transformation, fuzz ownership, and
  differential applicability exact and executable.

## Impact

This is test- and evidence-only. It changes no public API, peer DID wire value,
stable error category, production dependency, supported target, publication
state, external repository, release decision, or mdoc scope.
