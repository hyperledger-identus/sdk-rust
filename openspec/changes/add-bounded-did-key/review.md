# Exact-diff architecture and security review

Review date: 2026-10-28
Issue: #557
Develop base: `48c61920b8553d5401375338a5b217421b4c3c10`
Planning contract: `3dc45b207aa8e7ee892bc804489765ab9f913940`
Preimplementation receipt: `1391996538be16da4f32afb54ced680f1bd9578d`
Implementation under review: `02584a585d204d4f8b5be256802b35c5f4bb0fb8`
Reviewer: Codex CLI, used under the repository's Claude-or-Codex fallback rule

## Resolved findings

### R-1 — X25519 accepted non-contributory public keys

Severity: P1 before correction

The first independent review found that direct X25519 creation and parsing
accepted low-order public keys. Such material cannot contribute a secret in a
Diffie-Hellman exchange and must not be represented as usable key-agreement
material.

Resolution: the crypto primitive now exposes an allocation-free
`is_contributory` classifier, and the did:key profile rejects the complete
low-order set at both creation and parsing boundaries. Unit, conformance and
E2E regressions cover zero, order-one and ordinary public keys.

### R-2 — DID JSON-LD omitted the Multikey context

Severity: P2 before correction

The initial document projection used the DID Core context alone even though it
emits `Multikey` verification methods with `publicKeyMultibase`.

Resolution: DID JSON remains context-free, while DID JSON-LD deterministically
emits the DID Core and Multikey contexts. Serialization and E2E tests freeze
both representations.

### R-3 — X25519 aliases were not rejected

Severity: P1 before correction

The next review found that X25519's high-bit and field-modulus aliases could
produce distinct did:key identifiers for the same coordinate.

Resolution: the crypto primitive now exposes `has_canonical_encoding`, and the
method rejects high-bit aliases and coordinates greater than or equal to the
field modulus before relationship projection. Focused regressions retain the
canonical and rejected cases.

### R-4 — Ed25519 aliases and weak points were not rejected

Severity: P1 before correction

An exact-diff follow-up found the analogous Ed25519 risk: non-canonical point
encodings and weak points could cross the public-key boundary.

Resolution: the crypto primitive now exposes allocation-free canonical and
weak-point classifiers. Direct creation and parsed identifiers reject both
classes; ordinary keys retain their exact canonical bytes. Unit, conformance
and differential tests cover the boundary.

### R-5 — Generic resolution cloned an oversized method identifier

Severity: P2 before correction

The resolver originally converted the complete generic DID before applying the
method-specific 128-byte identifier ceiling. A caller could therefore force an
avoidable allocation before the narrow profile rejected the input.

Resolution: `DidKeyResolver` now preflights the method-specific identifier
length before cloning the generic DID. A deterministic E2E regression proves
early `IdentifierTooLong` rejection.

### R-6 — The Nix clean source omitted the attributed vector packet

Severity: build blocker before correction

The full clean-room Nix gate found that the isolated crate build did not retain
the fixture consumed by the compile-time conformance test.

Resolution: the Nix source filter now includes the exact versioned did:key
vector path, and the Rust source contract asserts its presence. The focused
source-contract derivation passes.

## Architecture review

- `identus-did-key` is a narrow, effect-free DID-method crate layered on
  `identus-core`, `identus-crypto` and `identus-did`; it introduces no HTTP,
  storage, runtime, custody or product dependency.
- Direct public-key inputs are SDK types. Third-party key, multibase or JSON
  types do not cross the public API.
- The profile owns exact codec, varint, base58-btc, public-key and relationship
  rules. Generic DID values and opt-in registry composition remain owned by
  `identus-did`.
- Resolution is deterministic and local. The method is not installed globally;
  consumers explicitly compose the resolver they want.
- Private keys, generation, derivation, signing, implicit Ed25519-to-X25519
  conversion, I/O, persistence and bindings remain outside this slice.

## Security review

- Method-specific input and decoded-payload ceilings are positive, public and
  enforced before expensive work or avoidable cloning.
- Only canonical unsigned-varint and base58-btc encodings are accepted. Codec,
  key length, curve encoding and unusable key material fail closed.
- Public errors and debug surfaces contain structural information only; no
  private-key or caller-controlled key bytes are exposed.
- The implementation adds no `unsafe`, ambient I/O, filesystem, environment,
  global registration or panic-based validation path.
- Property, attributed-vector, fuzz and E2E routes exercise identical public
  boundaries rather than privileged test-only parsers.

## Final review status

All independent findings above are resolved. A final Codex CLI review of exact
implementation head `02584a585d204d4f8b5be256802b35c5f4bb0fb8`
reported no actionable defects. The review sandbox could not acquire Cargo's
build lock, so independently run focused, workspace, fuzz, benchmark,
portability and Nix evidence remains the executable verification authority.
