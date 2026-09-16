# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/298
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003` governs the changed byte-to-retained-text input boundary.
- `SDK-LIM-007` remains effective but loses only the infallible
  `HexStr`/`Base64UrlStrNoPad` encoding clause after complete evidence.
- Pre-entry allocation, native DID cleanup, caller-budgeted work, direct JOSE
  retained-input variants, and `Multihash` remain unchanged.

## Introduced or changed constraints

Every public byte-to-`HexStr` or byte-to-`Base64UrlStrNoPad` construction path
SHALL be fallible and SHALL reject before encoded allocation would exceed 4,096
bytes. A private encoder MAY serve bounded parser output and fixed-width
internal key material but SHALL NOT be publicly reachable.

## Introduced or changed limitations

No new runtime limitation is introduced. The intentional pre-release source
migration is temporary compatibility work, not an effective product support
claim. Allocation performed by a caller before SDK entry remains outside typed
resource enforcement.

## Consumer and product impact

The blanket `From<AsRef<[u8]>>` source surface is removed before publication.
Callers migrate to `try_from_bytes`/`TryFrom`; successful values and wire text do
not change. The named local consumer repositories have no direct sdk-rust calls
at the assessed revisions. This is not a support-tier or release activation.

## Activation and rollback

Activation requires complete caller/API evidence, exact-limit tests, strict
workspace and target gates, fresh review, signed PR, and green exact-head CI.
Rollback restores the compatibility clause together with the infallible path.

## Evidence

Issue #298, ADR 0125, the OpenSpec research/inventory, exact boundary tests,
public API evidence, input-boundary inventory, strict target gates, review, and
exact-head CI are the required evidence chain.
