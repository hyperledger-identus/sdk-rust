# Rehearse SDK-TS DID facade adoption and rollback

## Why

The private Rust-backed DID facade has isolated syntax, document, resolution,
peer-method, and target evidence, but those capabilities still lack one pinned
consumer-shaped journey that runs against both the legacy and Rust providers
and proves terminal pre-initialization rollback.

## What Changes

- Add one revision-bound pairwise DID journey derived from the pinned SDK-TS
  8.1.4 donor inventory.
- Execute one provider-neutral assertion body against an injected legacy
  provider and the Rust-backed provider on Node.js, browser, React SSR, and
  hydrated React client targets.
- Cover DID/DID URL values, document projection, injected resolution and
  dereferencing, peer DID creation and local resolution, and request-shaped
  pairwise `from`/`to` values.
- Prove explicit provider selection and rollback before initialization without
  automatic fallback.
- Retain exact target receipts and reconcile TS1 consumer-proof and rollback
  evidence without changing the default SDK-TS route.

## Capabilities

### New Capabilities

- `sdk-ts-did-adoption-rehearsal`: define the pinned, provider-neutral,
  cross-target adoption and rollback journey for the existing DID facade.

## Impact

This changes only the private unpublished TypeScript package, tests, evidence,
and roadmap records. It does not mutate SDK-TS, activate a default route,
publish npm artifacts, accept support or performance budgets, add React Native,
implement DIDComm processing or credentials, or include mdoc.
