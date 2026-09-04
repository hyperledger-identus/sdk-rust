## Context

The curve libraries already provide redacted debug output and drop-time
zeroization for their owned secret types under the selected features. The SDK
still creates independent `[u8; 32]`, `[u8; 64]`, and `Vec<u8>` intermediates,
and its two HD key structs own public raw arrays while deriving `Debug` over
them. These SDK-owned buffers need an explicit lifecycle contract.

The change is repository-local at
`develop@688f29f399b5d9d46faa0e934182a1925c7d9120`. It adapts no donor code and
does not touch any consumer repository. The implementation dependency is
`zeroize` 1.9.0 from crates.io, checksum
`e13c156562582aa81c60cb29407084cdb54c4164760106ab78e6c5b0858cf64e`,
already locked transitively and licensed MIT OR Apache-2.0.

## Goals / Non-Goals

**Goals:**

- remove private key and chain-code bytes from safe formatting;
- erase SDK-owned secret-bearing buffers on normal drop;
- keep every existing derivation, mnemonic, signature, and random-generation
  output byte-identical;
- state the best-effort limit accurately and test the enforceable contract.

**Non-goals:**

- new algorithms, scalar-validation changes, Cardano/Khovratovich derivation,
  BIP32 path changes, key handles, secure storage, custody, FFI, or JOSE;
- protection against caller-created copies, operating-system swap, crash
  dumps, compiler/platform defects, or physical attacks;
- donor ports, downstream adoption, publishing, releases, settings, or `main`.

## Decisions

### D1 — Direct zeroize dependency

`identus-crypto` declares workspace `zeroize = 1.9` with derive support as an
optional direct dependency activated by every secret-bearing curve or
derivation feature. It uses `Zeroizing<T>` for temporary owned buffers plus
`Zeroize` and `ZeroizeOnDrop` on long-lived HD key values. Relying on a
transitive dependency would make the security behavior accidental and
unavailable to crate source; enabling it for `--no-default-features` would add
an unnecessary dependency to the representation-only build.

### D2 — Redacted HD debug output

Manual `Debug` implementations name the type and show only `depth` and child
index. Private key and chain code are omitted with a non-exhaustive marker.
The raw fields remain public in this compatibility-preserving slice, so callers
retain explicit access and remain responsible for copies they create.

### D3 — Zeroize owned HD state

Both HD key types derive `Zeroize` and `ZeroizeOnDrop`. Explicit zeroization
clears private key, chain code, depth, and child index. Clearing metadata is
not required for secrecy, but it prevents a manually zeroized value from
appearing usable and follows the derive contract uniformly.

### D4 — Zeroizing intermediates without output changes

Curve and mnemonic entropy arrays, HD derivation data and HMAC output, and the
PBKDF2 result array use `Zeroizing`. Public results are constructed exactly as
before. Returned raw secret arrays or vectors necessarily become caller-owned
copies and are outside the callee drop boundary.

### D5 — Evidence matches what safe Rust can prove

Tests assert redacted formatting, compile-time implementation of `Zeroize` and
`ZeroizeOnDrop`, explicit erasure, and byte-identical published vectors. They
do not inspect freed memory, because doing so would require unsound or unsafe
access and would overstate what the language and optimizer guarantee.

## Risks / Trade-offs

- Drop-time erasure is best effort and does not erase copies introduced by the
  compiler, allocator, operating system, or caller.
- Keeping public raw fields preserves source compatibility but permits callers
  to make unmanaged copies. A future opaque secret-handle or custody port is a
  separate API and product-boundary decision.
- Manual debug output is an intentional behavior change for diagnostics and a
  security improvement; public derivation metadata remains visible.

## Migration Plan

1. Land this specification and ADR before implementation.
2. Add the direct dependency, lifecycle wrappers, and focused tests.
3. Run focused feature checks, workspace gates, target/Nix gates, and a
   distinct security-focused review.
4. Synchronize the canonical crypto spec, archive the change, and deliver a
   signed/DCO issue-linked PR to `develop`.

Rollback is a source revert. No persisted or wire-format data changes.
