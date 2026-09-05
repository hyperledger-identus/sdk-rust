# Design: credential verifier execution

## Context

Issue #87 advances `IDR-009` from
`develop@e22fa8eab93026ba763e581873f4e5e233aaf035`. Earlier slices established
an open credential-format token, bounded opaque artifacts, metadata/schema and
status semantics, and a complete six-stage verification report. They
deliberately did not execute verification.

Oxid exposes an async `CredentialVerificationPort` over raw bytes and returns a
product inspection containing a wallet credential identifier, metadata and a
report. Its Midnight adapter proves the asynchronous DID-resolution need, but
also performs product-specific format probing and historically maps malformed
credentials through an operation error. Midnight status verification proves
that a concrete adapter may depend on ledger observations, proof runtimes and
acceptance inputs. Those implementation details are evidence for this seam,
not code to move into the SDK.

## Provenance and isolation

No donor code or fixture is copied. Exact revisions, file digests, licenses and
pre-existing worktree states are recorded in issue #87. Oxid,
midnight-identity, Lace ID Portal, NeoPRISM and Apollo remain read-only.

## Decisions

### D1 — One asynchronous, object-safe capability

`CredentialVerifier` is an `#[identus::port]` trait with one `verify` method.
It returns a boxed, `Send`, lifetime-borrowing future so runtimes and async-trait
macros are not selected by the domain crate. Concrete adapters inject DID,
crypto, status, schema, clock or network dependencies into themselves.

The port returns `CredentialVerificationResult`, whose success is the existing
canonical `VerificationReport`. The crate adds only the build-time
`identus-derive` dependency required by the repository's port marker.

### D2 — The request is a least-authority borrowed view

`CredentialVerificationRequest<'a>` borrows the envelope's validated
`CredentialFormat`, `CredentialPayload` and optional
`CredentialDetachedProof`. It has no public field mutation and exposes no
`CredentialPrivateMaterial`. Construction from `&CredentialEnvelope` copies
only references. Debug output reports the format and byte lengths, never
credential bytes.

Private material is holder-side opening or disclosure input. Giving it to a
general verifier would increase authority without satisfying IDR-009.

### D3 — Invalid evidence is a report, not an operation failure

If parsing, structure, issuer/key, proof, time, status or schema evidence is
invalid or incomplete, an adapter returns a canonical report with Failed or
NotChecked stages. The error channel contains only three data-free operational
classes:

- `UnsupportedFormat` when no exact adapter binding exists;
- `Unavailable` when an injected dependency cannot currently execute; and
- `Internal` when the adapter cannot safely produce a canonical report.

They map to static `credential.verification_*` errors. Unsupported maps to
`ErrorKind::Unsupported`; unavailable and internal map to
`ErrorKind::Internal`. No variant carries payload, proof, format, endpoint or
downstream error data. `VerificationFailed` is intentionally not used because
failed credential evidence is a successful report result.

### D4 — Exact setup-time binding, immutable runtime dispatch

`CredentialVerifierRegistryBuilder` binds an exact validated
`CredentialFormat` to one `Arc<dyn CredentialVerifier>`. It rejects a duplicate
format and a sixty-fifth entry through static `CredentialError` variants.
`build` freezes the map behind an `Arc`, making the registry clone-cheap and
immutable. `CredentialVerifierRegistry` implements `CredentialVerifier` and
dispatches by exact format-token lookup.

The registry exposes count, emptiness, exact support and deterministic lexical
format iteration for composition diagnostics. It never inspects payload or
proof bytes, guesses a format from a prefix, applies fallback order or retries
another adapter after selection.

### D5 — Bounded setup, zero artifact copying on the hot path

The registry holds at most 64 entries in a `BTreeMap`. Setup owns one format
key and one shared verifier per entry. A request and dispatch borrow all
credential artifacts and do not clone their byte buffers. Exact lookup is
`O(log n)` with a small fixed upper bound. An ignored release diagnostic polls
a ready dummy future through the production registry and reports throughput
without a machine-dependent threshold.

## Risks and trade-offs

- Boxed futures allocate per verification call. This is the existing
  runtime-neutral SDK convention and avoids freezing one executor or async
  macro; concrete CPU-heavy adapters dominate that cost. A future GAT/RPITIT
  migration needs separate compatibility evidence.
- One verifier per exact format prevents implicit fallback. Composition roots
  can register a deliberate multiplexer as that format's verifier when needed.
- `Unavailable` is intentionally broad. Retryability and diagnostics remain in
  product/application layers rather than crossing the redaction-safe boundary.
- A verifier can still implement trust policy internally, but the generic API
  neither requests nor reports it. Conformance and downstream adapter review
  must preserve that separation.
- The registry does not schedule individual stages. Stage-specific execution
  ports require at least two independent concrete consumers before enlarging
  this API.

## Migration and rollback

The API is additive and unreleased. There is no stored or wire migration. A
focused revert removes it. Concrete Midnight/Cardano/W3C adapters, downstream
adoption, stage-specific orchestration and publication remain separate issues.
