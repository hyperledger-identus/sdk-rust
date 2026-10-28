# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/558
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 require chain-neutral inward dependencies,
cohesive ownership, and isolated effects. SDK-SEC-001 and SDK-SEC-003 require
safe Rust and finite untrusted work. SDK-COMPAT-001 through SDK-COMPAT-005
govern stable, MSRV, target, and compiler evidence. ADR 0187 authorizes the
separate static method crate; ADR 0189 fixes its first public-JWK profile.

## Introduced or changed constraints

- `identus-did-jwk` depends inward only on generic SDK core/DID/crypto and
  existing feature-minimal JSON/base64 support; no third-party SSI facade or
  sibling method crate enters its production graph.
- Only canonical unpadded base64url carrying bounded duplicate-free UTF-8 JSON
  for Ed25519, X25519, P-256, or secp256k1 public JWKs is accepted.
- Creation and parsing revalidate key material through `identus-crypto` and
  reject known private parameters, `key_ops`, unsupported profiles, and
  incompatible explicit `use` values. Absent `use` retains the pinned method's
  five-relationship projection as an explicit interoperability exception.
- Method-specific text is at most 21,846 bytes and decoded JSON at most 16,384
  bytes on both creation and parsing, with explicit depth, node, member, and
  inherited extension budgets.
- Parsing preserves the supplied DID; deterministic creation does not imply
  one-key/one-DID uniqueness or canonical JSON for external identifiers.
- Public errors contain stable categories only, never identifier, JSON,
  extension, coordinate, or key content.
- Resolution is deterministic and effect-free with one fixed `#0`
  `JsonWebKey2020` method and the selected `use` relationship policy.

## Introduced or changed limitations

The pinned draft and strict first profile are unreleased compatibility. Private
JWKs, unsupported curves, JWK Set, key generation, signing, proof selection,
remote lookup, trust, persistence, networking, runtime, bindings, consumer
adoption, publication, and uniqueness claims remain absent.

## Consumer and product impact

Rust adopters gain an opt-in deterministic static method over SDK-owned values.
No existing crate changes behavior and no language SDK or product is migrated.
Any compatibility DTO belongs in a later adapter with its own migration issue.

## Activation and rollback

Activation requires exact #558 preflight, ADR 0189, provenance, all declared
quality and target evidence, immutable ledger/catalog entries, exact-diff CLI
review, a manifest/public-API dependency guard, a repository-diff stop-boundary
audit, a distinct registry E2E run, green protected CI, and an issue-linked PR.
Before publication,
rollback removes the additive crate and evidence without runtime migration.

## Evidence

Evidence authorities are issue #558, ADRs 0187 and 0189, the pinned method and
RFCs, attributed oracle fixtures, OpenSpec/factory receipts, quality
declarations, target/dependency gates, exact-diff review, and protected CI.
