# Implement a bounded RFC 9901 SD-JWT engine

## Why

SDK-Rust has bounded JOSE, credential, presentation, and effect seams but no
stable RFC 9901 engine. A focused engine is now needed so Rust and future thin
language adapters can issue, selectively disclose, key-bind, and verify SD-JWT
without inheriting another SDK's DTOs, trust policy, runtime, or unsafe
resource behavior.

## What Changes

- Add an independently selectable, unpublished `identus-sd-jwt` crate for the
  stable RFC 9901 base format.
- Expose staged parsed, disclosure-processed, and cryptographically verified
  SDK-owned values so unverified input cannot be mistaken for verified
  evidence.
- Implement bounded issuance, decoy insertion, disclosure parsing and
  selection, presentation, key-binding input, and verification receipts.
- Inject entropy, time, digest, issuer signing/verification, and holder
  signing/verification through narrow effect ports; keep custody, trust,
  transport, persistence, replay policy, and consent outside the crate.
- Add redaction-safe errors and explicit byte, count, depth, retained-output,
  and aggregate-work limits before expensive parsing or allocation.
- Add provenance-bound RFC examples, clean-room negative vectors, property,
  fuzz, benchmark, differential, target, and offline issue-to-verification E2E
  evidence.
- Keep the evolving SD-JWT VC profile, HTTP, OID4VC integration, publication,
  and downstream adoption out of this slice.

## Capabilities

### New Capabilities

- `sd-jwt-rfc9901`: stable base issuance, selective disclosure, presentation,
  key binding, verification, staged states, resource bounds, and evidence.

### Modified Capabilities

- `sd-jwt-engine-selection`: activate the previously selected focused-engine
  path while preserving the separation between stable RFC 9901 behavior and
  the later SD-JWT VC profile.

## Impact

The workspace gains one additive crate depending inward on existing Identus
JOSE, cryptography, credential, presentation, time, and entropy abstractions.
No current wire behavior, released crate API, downstream repository, profile
identifier, trust decision, or publication state changes. Issue #568 owns the
slice; #569 remains blocked until this base is merged and evidenced.
