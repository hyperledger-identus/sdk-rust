# ADR: Conformance Fixture Schema Policy

**Status**: Accepted

**Date**: 2026-06-13

## Context

`sdk-rust` is intended to become the shared Rust core for Identus SDKs,
bindings, and future services. Conformance fixtures must therefore be stable
enough to drive Rust tests, wrapper tests, migration proofs, and integration
dashboards without leaking secrets or binding protocol semantics to a single
adapter implementation.

The repository already has the conformance layout:

```text
fixtures/conformance/
  static-model/
  vector/
  transcript/
  interop/
  infrastructure/
```

This ADR defines the minimum schema policy for DID/key/credential/protocol
fixtures before more behavior is ported.

## Decision

All checked-in conformance fixtures must be JSON by default and must include a
schema discriminator, ownership metadata, source evidence, redaction policy, and
expected validation outcome.

Every new fixture must be independently reviewable, deterministic, and safe to
run in `cargo test --workspace` unless it lives under `interop/` or
`infrastructure/`.

## Common Fixture Fields

Every fixture file must contain these fields unless an ADR explicitly grants an
exception:

| Field | Required | Purpose |
|---|---|---|
| `schema_version` | Yes | Versioned fixture schema id, for example `didcomm-transcript-0.1`. |
| `specification_ids` or `specification_id` | Yes | Conformance catalog ids covered by the fixture. |
| `source` or `source_uri` | Yes | Repository path, public specification URI, or reference-suite URI. |
| `owner_crate` | Preferred | Rust crate expected to consume the fixture first. |
| `case_id` | Preferred | Stable machine-readable case id when a directory contains many fixtures. |
| `description` | Preferred | Short human-readable intent. |
| `expected` or `expected_states` | Yes | Expected output, typed error, or protocol states. |
| `redaction_policy` | Yes | Statement that no private keys, tokens, claims, or production identifiers are present, or a reason why public test vectors are safe. |

Legacy seed fixtures may use `specification_id` instead of
`specification_ids`. New multi-protocol fixtures must use `specification_ids`.

## DID And Key Vector Fixtures

DID and key vectors must include:

- DID method and conformance id.
- Input material classification: public fixture, generated test secret, or
  published public vector.
- Derivation path when applicable.
- Curve and key encoding.
- Canonical string representation.
- Expected parser classification and resolver ownership.
- Negative cases for malformed method names, malformed identifiers, unsupported
  method-specific syntax, invalid DID URL fragments, and lossy normalization.

Secrets are allowed only when they are published public test vectors or
generated deterministic test material. They must be labeled as fixture-only and
must never be usable against production systems.

## Credential And Presentation Fixtures

Credential and presentation fixtures must include:

- Credential format or presentation format.
- Issuer, holder, verifier, subject, and audience identifiers as fixture values.
- Verification time and clock-skew assumptions.
- Status or revocation reference when relevant.
- Expected success or typed verification error.
- Disclosure, holder binding, domain, challenge, and audience expectations when
  applicable.
- Redacted or synthetic claims unless the source is a public specification
  vector.

Negative fixtures must cover expired credentials, not-before violations, wrong
audience, wrong domain/challenge, unsupported key purpose, status revoked,
tampered signature, malformed schema, missing holder binding, and unsupported
format.

## DIDComm Transcript Fixtures

DIDComm transcripts must include:

- `schema_version`.
- `specification_ids`.
- `source`.
- `participants` with fixture-safe DIDs.
- Ordered `messages`.
- `expected_states`.
- `redaction_policy`.

Each message should include:

- `id`.
- `type`.
- `from` and `to` where the protocol permits addressing.
- `thid` and `pthid` when threading or OOB parent linkage is relevant.
- Redacted `body` and `attachments` references rather than real claims,
  plaintext secrets, or production payloads.

Required DIDComm transcript families:

- OOB invitation and connectionless attachments.
- BasicMessage.
- Trust Ping.
- Discover Features.
- Routing and Forward.
- Coordinate Mediation 2.0 and planned 3.0.
- Message Pickup 3.0.
- Issue Credential 3.0.
- Present Proof 3.0.
- Report Problem 2.0.
- Revocation Notification.
- Legacy compatibility aliases observed in Identus source.

DIDComm transcript tests must first prove parsing and family classification,
then protocol state-machine replay, and only later pack/unpack round trips.

## OpenID4VC Transcript Fixtures

OpenID4VC transcripts must include:

- Issuer, wallet, verifier, authorization server, and trust anchor roles.
- Request URI, nonce, state, issuer metadata, credential configuration, and
  presentation definition or DCQL input where relevant.
- Flow kind: pre-authorized, authorization code, deferred credential, OID4VP
  direct post, SIOPv2, HAIP, or federation-backed trust.
- Credential format: JWT VC, SD-JWT VC, mdoc, W3C VC, AnonCreds bridge, or
  OpenBadges profile.
- Expected token, credential, presentation, or typed error state.

Negative fixtures must cover expired metadata, wrong issuer, bad audience,
wrong nonce/state, unsupported credential format, unsupported proof type,
invalid request object, invalid holder binding, missing authorization server,
and trust-chain rejection.

## Review And Evolution

- Fixture schema changes must be backward-compatible unless a migration task is
  checked into `specs/`.
- Behavior fixtures must reference at least one backlog task with acceptance
  criteria.
- Fixtures that require Docker, ledgers, databases, OID providers, or device
  adapters must live under `infrastructure/` and must not block the default
  workspace test path.
- `identus-conformance` owns metadata and schema guards.
- Protocol crates own behavior-specific fixture replay.
- Binding crates must reuse the same fixtures instead of inventing wrapper-only
  behavior vectors.

## Consequences

- Fixtures become a durable compatibility contract across Rust crates and future
  wrappers.
- Protocol work can start with transcript parsing and state replay before heavy
  infrastructure exists.
- Secret handling and redaction are reviewable in every fixture.
- Future fixture schema changes need explicit migration work rather than silent
  drift.
