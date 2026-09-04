# ADR 0023: open format-neutral credential envelope

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #71
- **Decision scope:** first `IDR-007` credential-semantics slice

## Context

Credential formats differ in encoding, proof placement, selective-disclosure
material, identifiers, metadata, and verification semantics. They still need a
common holder-side boundary for preserving bounded encoded bytes without
making a validity or trust claim. The workspace already contains an unreleased
`identus-credentials` placeholder. Oxid provides useful source evidence, but
its current model includes product IDs, Midnight format variants, inspection
state, and disclosure policy that do not belong in the first generic slice.

## Decision

1. Activate the existing `identus-credentials` credential-semantics crate
   rather than add another package. Its name remains experimental until
   namespace and release governance make a public commitment.
2. Represent credential formats with an open, bounded, case-sensitive
   `CredentialFormat` newtype. Do not use a central enum or executable registry
   in the envelope core.
3. Represent encoded payload, detached proof, and private format material with
   distinct bounded owned types. Preserve bytes exactly and expose them through
   borrowed accessors only.
4. Redact all artifact formatting. Give private material explicit, rejected-
   input, and drop-time zeroization without `Clone` or ordinary equality; do
   not claim storage encryption, custody, PII deletion, or erasure of
   caller/platform copies.
5. Keep the envelope non-validating and free of serde/codecs, DID requirements,
   status, trust, storage, protocols, chain dependencies, and product policy.
6. Use static, capability-scoped stable error bridges and keep every package
   `publish = false`.

## Consequences

- Format adapters receive a small reusable integration type now, while later
  credential and presentation work can layer metadata and evidence without
  re-owning raw artifacts.
- Syntactically valid format identifiers may still be unsupported; adapter
  selection owns that decision.
- Encoded credentials and proofs may contain personal data but are not
  automatically zeroized. Their persistence and deletion lifecycle belongs to
  downstream storage/privacy policy; safe formatting remains redacted.
- Oxid, midnight-identity, Lace ID Portal, and NeoPRISM remain unchanged.
- The slice is independently revertible before publication. Any future crate
  rename, wire wrapper, handler registry, or consumer adoption requires its own
  issue and compatibility evidence.
