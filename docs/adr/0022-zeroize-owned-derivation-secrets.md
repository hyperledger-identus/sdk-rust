# ADR 0022: Zeroize and redact owned derivation secrets

- Status: Accepted
- Date: 2026-09-05
- Issue: #69

## Context

`HDKey` and `EdHDKey` own raw private-key and chain-code arrays and currently
derive `Debug`, which renders those secrets. Crypto construction and derivation
also create independent entropy, HMAC, and PBKDF2 intermediates that are not
erased when their normal Rust scope ends. Curve backends already protect their
owned secret types, but those guarantees do not cover the SDK-owned copies.

## Decision

Declare `zeroize` 1.9 as a direct workspace dependency. HD key values implement
`Zeroize` and `ZeroizeOnDrop` and use manual debug implementations that omit
private key and chain code. SDK-owned temporary entropy, HMAC input/output,
mnemonic entropy, and PBKDF2 output use `Zeroizing` guards.

The public raw HD fields and secret-returning APIs remain unchanged in this
compatibility-preserving slice. Callers own and must protect any copies they
obtain. Tests prove trait availability, explicit erasure, formatting
redaction, and unchanged vectors; they do not inspect freed memory or claim
protection beyond best-effort owned-buffer erasure.

## Consequences

- Safe formatting no longer discloses HD private key or chain-code bytes.
- SDK-owned secret intermediates receive drop-time best-effort erasure on both
  success and early return.
- Existing algorithms, outputs, APIs, features, errors, and target policy stay
  unchanged except for intentionally redacted HD debug text.
- This does not provide custody, secure storage, key handles, operating-system
  memory locking, crash-dump protection, or erasure of caller/compiler copies.
- The dependency cone gains no new locked package because `zeroize` 1.9.0 was
  already present through the selected curve libraries.
