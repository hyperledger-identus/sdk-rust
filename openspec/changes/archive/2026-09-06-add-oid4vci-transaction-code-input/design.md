# Design: bounded OID4VCI Transaction Code input

## Context

The SDK already distinguishes a Final Credential Offer `tx_code` object from
its absence and binds a validated offer to an eligible Authorization Server.
OID4VCI Final section 6.1 requires the later Token Request to carry a
`tx_code` string exactly when that offer object was present, including when it
was empty. Transaction Code material is bearer-adjacent and must not leak
through diagnostics, serialization, or failed construction.

The current Oxid profile supports only the no-code path and rejects offers that
request a Transaction Code. Lace's immutable Final fixture likewise exercises
the no-code path and records `tx_code: null` as legacy-negative evidence. This
change supplies the missing generic input state without editing either
consumer.

## Goals and non-goals

Goals:

- make input presence agreement a consuming, typed state transition;
- erase owned Transaction Code material on success and failure;
- independently bound caller input without copying it;
- retain issuer-advertised requirements for UI use through the predecessor;
- keep all success and failure diagnostics static and data-free.

Non-goals:

- form or JSON encoding, HTTP, Token Request/Response, client identity or
  authentication;
- deciding whether a Transaction Code is correct, fresh, secret, sufficiently
  random, safely delivered, or replay resistant;
- normalizing, trimming, comparing, logging, or exposing raw input;
- locally enforcing `input_mode` or `length` as server authentication policy;
- downstream adoption, publication, release, or `main` work.

## Decisions

### Consume the bound server state and optional owned input

`CredentialOfferWithPreAuthorizedServer::try_with_transaction_code_input`
accepts `Option<String>` and `TransactionCodeInputLimits`. It consumes the
predecessor and returns `CredentialOfferWithPreAuthorizedTokenInput` only after
all invariants pass. The success state owns both values and exposes the
predecessor by reference plus a presence-only query.

The raw Transaction Code remains private to the crate. A later request-builder
slice can use a crate-private accessor without adding a public secret-reading
surface now.

### Treat offer mode and length as UI guidance in this state

The Final specification says `input_mode`, `length`, and `description` help the
Wallet render an input experience; section 6.1 normatively requires parameter
presence and leaves wrong-code handling to the Authorization Server. This
transition therefore proves presence and resource safety, not code validity.
It rejects empty input as absence disguised as a value, but does not trim or
enforce advertised mode/length. Those requirements remain available through
the owned grant for UI validation and display.

Strict local mode/length enforcement was rejected because it would claim that
advisory metadata is an authentication oracle and could prevent submission to
an Authorization Server that remains the normative validator.

### Wrap before validating and bound decoded UTF-8 bytes

A supplied `String` is immediately wrapped in `Zeroizing<String>` before the
empty and byte-limit checks. All error paths then erase the owned allocation.
The limit is positive and defaults to 256 decoded UTF-8 bytes. Byte accounting
is deterministic and does not introduce Unicode normalization or character
semantics.

### Preserve the existing dependency and portability cone

The implementation uses `std` and the crate's existing `zeroize` dependency.
It adds no parsing, Serde, manifest, lockfile, feature, unsafe, HTTP, async,
crypto, DID, storage, chain, or product surface. Existing Rust 1.85, WASM,
Android, iOS, workspace, supply-chain, and Nix gates remain applicable.

## Public and error contract

Additive public types:

- `TransactionCodeInputLimits` with positive construction, a byte accessor,
  and a 256-byte default;
- `CredentialOfferWithPreAuthorizedTokenInput` with predecessor and
  presence-only accessors.

Additive consuming transition:

- `CredentialOfferWithPreAuthorizedServer::try_with_transaction_code_input`.

Add fieldless error variants and stable codes for invalid input limits,
required input missing, unexpected input, empty input, and oversized input.
Every Debug/Display/core-error bridge remains static.

## Verification

- Positive tests: required/provided, absent/omitted, exact byte bound, owned
  predecessor access, and zero-sized-content non-copy behavior.
- Negative tests: invalid zero limit, missing, unexpected, empty, and
  oversized multibyte input.
- Redaction tests: distinct canaries across input and predecessors on success
  and every new error surface.
- Focused tests in all-feature and no-default modes, strict Clippy/docs,
  workspace gates, factory preservation/archive, Rust 1.85, WASM/mobile,
  supply-chain, and full Nix checks.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

Normative source: OpenID4VCI 1.0 Final sections 3.5, 4.1.1, and 6.1, HTML
SHA-256 `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

Consumer evidence is behavior-only and Apache-2.0. Exact Oxid/Lace revisions,
paths, and hashes are recorded in issue #123 and the verification receipt; no
consumer source or fixture is copied.
