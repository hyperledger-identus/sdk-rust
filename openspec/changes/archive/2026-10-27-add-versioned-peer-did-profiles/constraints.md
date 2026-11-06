# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/493
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 require inward dependencies, reusable crate
ownership and effect isolation. SDK-SEC-003 requires finite untrusted work.
Generic DID, target, error, unsafe-code, naming and release constraints remain
effective. ADR 0174 authorizes the separate method crate and explicit profiles.

## Introduced or changed constraints

- `identus-did-peer` depends inward on generic DID/core facilities; generic
  crates and unrelated method crates cannot depend on it.
- `numalgo4` and `numalgo2_compat` are separate public modules and value types.
  Cross-profile work fails before proportional decoding.
- Production dependencies exclude both oracle crates and donor DTO, secret,
  runtime, storage and agent packages.
- The crate is synchronous and effect-free. Short-form numalgo-4 resolution
  accepts caller-supplied long-form material through a finite call.
- Raw DID, encoded element, decoded document, key, service, collection and JSON
  traversal limits are public and checked at the narrowest safe boundary.
- Numalgo 4 verifies SHA2-256 over the exact encoded-document UTF-8 text before
  parsing JSON. Input documents omit root `id`; contextualization rewrites only
  typed local identifiers/references, not arbitrary extension strings.
- Numalgo 2 accepts only reviewed purpose codes and public multicodec families.
- Public errors redact DID, key, service and document bytes.
- No unsafe code, native dependency, panic on untrusted input, network, file
  system, private key, entropy, persistence, executor or global mutable state.

## Introduced or changed limitations

- Numalgo 4 is draft/proposal-bound; its pinned profile is part of compatibility.
- Numalgo-4 short form is not self-resolving. Without matching long-form
  material, the SDK returns a stable not-found category.
- Numalgo 2 is compatibility-only and does not imply algorithms 0, 1 or 3.
- Rust types/errors remain canonical; SDK-TS mapping belongs to #492.
- This issue does not mutate SDK-TS, NeoPRISM, a mediator, or a cloud agent.

## Consumer and product impact

Rust adopters gain portable current and compatibility peer DID profiles without
accepting TypeScript or VCX DTO ownership. Existing SDK packages remain
behaviorally unchanged. SDK-TS can later adopt Rust through #492; wallet,
DIDComm and agent products continue to own custody and orchestration.

## Activation and rollback

Activation requires exact issue #523 preflight, provenance and quality
declarations, focused/workspace gates, target/dependency evidence, independent
CLI review, green protected CI and an issue-linked PR. Claude is preferred;
Codex CLI is the immediate fallback for any Claude availability or limit
failure. Before publication the additive crate can be removed; published
compatibility changes require a superseding decision.

## Evidence

Evidence authorities are issue #523, ADR 0174, pinned DIF/WG sources, SDK-TS
revision, the two Rust oracle versions, OpenSpec/factory validation,
conformance/property/fuzz/differential/benchmark routes, target/dependency
receipts, independent CLI review and protected CI.
