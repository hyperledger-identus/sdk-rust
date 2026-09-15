# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/168
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: gains repository-wide machine enforcement for implemented
  package coverage and boundary dispositions.
- `SDK-LIM-007`: changes from an incomplete audit to an exact residual outer
  preallocation and caller-budgeted work limitation.
- `SDK-SEC-001`: oversized mnemonic/passphrase failures remain redaction-safe.
- `SDK-ARCH-001`: transport, binding, wallet-adapter and protocol budgets stay
  with their owning layers.
- `SDK-REPO-001`: activation remains protected integration through `develop`.

## Introduced or changed constraints

Every implemented runtime package SHALL have one or more machine-readable
input-boundary rows. Each row SHALL name one ownership disposition, exact
evidence, consumer impact and review triggers. SDK-enforced rows SHALL name
explicit limits. A package classification change or missing coverage SHALL
fail the offline factory contract.

BIP-39 helpers SHALL reject unsupported entropy lengths before dependency work,
more than 24 words or a word above the longest accepted English word before
joining, and passphrases above 4,096 UTF-8 bytes before normalization or PBKDF2.
Standalone public JWKs SHALL reject more than 32 extension members, extension
depth above 16, more than 1,024 extension JSON nodes, or more than 65,536
aggregate UTF-8 bytes across extension keys and string values before retention.

## Introduced or changed limitations

Typed SDK validation cannot retroactively prevent allocation by serde, Axum,
UniFFI, JavaScript, or a caller that already constructed an owned value.
Primitive hash/HMAC/sign/verify inputs and generic async/storage adapter payloads
remain caller-budgeted where the SDK neither retains them nor owns a normative
protocol budget. These exact residuals replace the broad unaudited statement.

## Consumer and product impact

Existing valid BIP-39/KMP wallet inputs remain byte-compatible. Callers using a
passphrase above 4,096 UTF-8 bytes receive the existing redacted invalid-
mnemonic error and must choose an explicitly budgeted lower-level derivation
path. Consumers must continue bounding bytes before transport/deserializer/FFI
allocation and must budget primitive/port work for their protocol.
Existing ordinary JWK metadata remains compatible; extension documents beyond
the explicit experimental-facade budgets are rejected with a redacted JWK
error and require a higher-level format/profile decision.

Oxid, Midnight, midnight-identity, NeoPRISM, Lace and Apollo receive no source,
wire, storage, protocol, support, release, or migration claim.

## Activation and rollback

Activation requires the exact issue-linked planning receipt, complete inventory,
focused mutations and crypto boundary tests, distinct review, compatible local
Nix, and protected green CI. Rollback reverts the implementation and restores
the broad incomplete-audit wording; it does not authorize consumers to remove
their existing outer limits.

## Evidence

The issue, accepted constraints, bootstrap inventory, core precedent, BIP-39
specification, source audit, machine inventory/checker, mutation suite, exact
crypto boundary tests, factory/OpenSpec gates, full Nix, review and protected
CI form the evidence.
