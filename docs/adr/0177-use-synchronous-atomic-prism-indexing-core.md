# ADR 0177: Use a synchronous atomic PRISM indexing core

- Status: Accepted
- Date: 2026-10-27
- Issue: https://github.com/hyperledger-identus/sdk-rust/issues/522
- Supersedes: the implementation detail left open by ADR 0174

## Context

NeoPRISM proves PRISM storage classification and indexing behavior, but its
library boundary exposes Tokio channels, async traits, Cardano metadata,
database-oriented repositories and unstructured errors. SDK-Rust must support
native, WASM, iOS and Android without selecting an executor or persistence
technology. It must also make atomic visibility, replay and resource use
testable without a node.

## Decision

Add two inward-facing crates:

- `identus-prism-vdr` owns bounded typed PRISM storage operations and finite
  local-first root traversal over a read-only lookup port.
- `identus-prism-indexer` owns ordered complete-batch planning and an atomic
  compare-and-set commit port. It invokes commit at most once per valid batch.

Core entry points are synchronous and finish within explicit limits. The SDK
does not poll a source, spawn a task, create a channel, sleep or retry. Batch
capacity is the backpressure boundary. Callers own async adaptation,
cancellation and retries between bounded calls.

One SDK batch represents one complete ledger transaction. Its public header
contains a bounded opaque transaction identity, declared PRISM-operation count
and next cursor; records carry a contiguous zero-based operation index. The
constructor rejects missing, repeated or cross-transaction records and rejects
the complete transaction when it exceeds the profile. The source adapter owns
extraction of the authoritative transaction boundary; the SDK verifies the
declared boundary before planning.

After structural batch validation, the indexer computes a domain-separated
SHA-256 transition digest over the complete ordered signed records and cursor
transition. Before reading mutable DID/VDR snapshots or planning, it probes the
store's immutable committed-transition receipt by digest. An exact receipt is
replay success, which keeps an ambiguous-acknowledgement retry reachable even
after state advanced. An absent receipt proceeds to normal planning.

The commit request contains the same digest and the complete ordered mutation
set. Atomically, the adapter first compares that digest with the previously
committed transition: an exact match is replay success even though its expected
cursor is now stale. Otherwise it performs expected-cursor compare-and-set and
reports conflict on mismatch. It guarantees all-or-nothing visibility. The
read-only probe is an optimization/correctness path for retries; the atomic
commit check remains the race-closing authority.

Before a VDR mutation is planned, the indexer supplies the owner DID state at
that exact ordered position. It begins with a caller-provided snapshot and
reduces DID create/update/deactivate operations in the same transaction before
later storage operations. `identus-prism-vdr` requires a live active secp256k1
VDR key named by `signed_with` and verifies the signature over the exact nested
operation bytes. A deactivated owner, missing/revoked/wrong-purpose key or bad
signature rejects the complete transaction. The protobuf comment naming an
issuing key is treated as stale donor text; the dedicated VDR key enforced by
NeoPRISM's processor and represented by `KeyUsage::Vdr` is the SDK profile.

Root resolution alone is insufficient for updates and deactivation. The VDR
lookup snapshot also supplies the current head for each root. A predecessor
must equal that head, after applying earlier operations in the same transaction;
otherwise the operation is a stale fork and rejects the complete transaction.

Default limits and absolute maxima are public typed configuration. The initial
profile will cap operation/object bytes at the already accepted PRISM envelope
limits, records at 1,024 per batch, total signed bytes at 8 MiB, ancestry at 200
nodes and lookup work at 4,096 nodes per batch. Values may be configured lower;
raising absolute maxima is a compatibility/security decision.

## Consequences

Portable adopters can use their native executor and store. Deterministic core
tests do not depend on time or scheduling. A transaction too large for the
profile is rejected rather than split, and cancellation cannot interrupt a
pure planning call; its finite maximum is therefore security-critical.

Async convenience, chain sources, persistence implementations, retry policy,
metrics exporters and service lifecycle remain downstream. NeoPRISM will need
a thin adapter around its current source and repository implementations.

## Alternatives rejected

- A Tokio engine: rejects portable executor neutrality and hides channel work.
- Public async traits: unnecessary for pure planning and force a future/runtime
  contract before supported adopters demonstrate a shared need.
- Begin/commit transaction objects with borrowed lifetimes: complicate object
  safety and FFI while allowing partial mutation before validation completes.
- Split commits for oversized transactions: violates atomic visibility.
- Copy donor repository types: leaks UUID, Cardano, SQL and string-error shape.
- Trust a caller-provided vector as a transaction: cannot prove completeness;
  the typed transaction header and contiguous operation indices are required.
- Treat any root-reachable predecessor as current: admits stale forks after an
  entry has already advanced.

## Review fallback

Independent CLI review is provider-neutral. Claude is preferred when available;
Claude absence, authentication failure or quota exhaustion triggers immediate
Codex CLI review of the same exact commit. Waiting for a provider reset is not
a valid blocker. The review artifact records provider, command and commit, and
all concrete findings are verified locally.
