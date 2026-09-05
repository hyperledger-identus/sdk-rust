# ADR 0032: own wallet storage ports in the orchestration ring

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Related work:** issue #89, `IDR-010`, OpenSpec change
  `add-wallet-storage-ports`

## Context

SDK-Rust has reusable DID, credential, presentation and verification semantics,
but its consumers define incompatible persistence traits. The shared contract
crosses domain boundaries while concrete records, identifiers, encryption,
codecs, databases and product policy must remain downstream.

The blueprint anticipates a possible `identus-ports` package, while the current
workspace already has a quarantined `identus-wallet` package in the
orchestration ring. Creating a new bottom-ring package would require generic
storage records or dependencies pointing outward. Co-locating each trait in a
domain would scatter shared concurrency and pagination semantics and would not
provide one integration surface for wallet composition.

## Decision

1. Activate `identus-wallet` only for policy-neutral storage contracts.
2. Define five interface-segregated capabilities: secret, credential, DID,
   protocol-state and status-cache storage. Do not define a generic repository
   supertrait.
3. Use associated consumer-owned scope, key, value and index types so the
   wallet crate needs no DID, credential, presentation or protocol dependency.
4. Use object-safe boxed `Send` futures without selecting an executor or
   async-trait macro.
5. Standardize bounded opaque revisions/cursors, pagination, single-record
   conditional mutation, redacted receipts and static errors only.
6. Prohibit secret enumeration. Bound credential, DID and protocol recovery
   indexes to pages of at most 256 entries. Keep status cache exact-key only.
7. Promise no wire/stored representation, encryption, database, transaction,
   synchronization, retention, backup, custody or product behavior.
8. Keep production adapters outside this slice. Test-only consumer-shaped
   memory implementations provide behavioral evidence.

## Consequences

- Consumers gain one small dependency for compatible storage composition while
  retaining their native record and identifier types.
- The wallet package becomes implemented but remains experimental, unreleased
  and explicitly not a wallet product.
- Associated-type equality makes trait-object spellings longer, in exchange
  for a minimal dependency cone and no invented SDK persistence model.
- Single-record compare-and-swap is expressible; cross-record atomicity and
  synchronization remain future, evidence-driven contracts.
- `IDR-010` remains specified until independent in-memory and encrypted
  consumer adapters pass conformance.

## Rejected alternatives

- **One generic repository trait:** violates interface segregation and risks
  secret enumeration plus accidental semantic coupling.
- **Domain-local duplicated ports:** scatters shared behavior and complicates
  wallet composition.
- **New `identus-ports` crate now:** expands namespace/rulebook scope without a
  second accepted port-family need.
- **SDK-owned serialized records:** imports codec, migration and product-ID
  policy and creates a stored-data compatibility promise prematurely.

## Rollback

Before publication, revert issue #89 and return `identus-wallet` to its
quarantined marker. No stored data, wire protocol, release or consumer branch
changes are involved.
