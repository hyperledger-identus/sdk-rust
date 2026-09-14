# Design: opaque HD-key secret exposure

## Context

`HDKey` and `EdHDKey` already erase the arrays they own on normal drop and
redact their own `Debug` output. Their public `[u8; 32]` fields nevertheless
allow an external field read to create an unguarded copy with no auditable
method call and no erasure owner. The existing candidate API baseline records
those four public fields.

The repository already uses `zeroize` 1.9.0, but returning its
`Zeroizing<[u8; 32]>` directly is insufficient for this public boundary:
`Zeroizing` derives `Debug`, so safe formatting would reveal the returned
bytes. The SDK needs a narrow facade that keeps the concrete storage private.

## Goals / non-goals

Goals:

- make the four long-lived secret fields opaque;
- make every SDK-created export copy visibly intentional and drop-zeroizing;
- keep safe formatting, serialization and binding surfaces closed;
- preserve arithmetic, metadata, errors, vectors, features, targets and
  dependency topology.

Non-goals are custody, locked memory, allocator/register/swap/dump protection,
new algorithms, public derivation, new FFI, publication, or consumer edits.

## Decision

### D1 — Private long-lived arrays

`HDKey::private_key`, `HDKey::chain_code`, `EdHDKey::private_key`, and
`EdHDKey::chain_code` become private. Internal derivation continues to borrow
the arrays in place. Public metadata fields remain unchanged in this slice.

### D2 — SDK-owned exposure value

Add `HdKeySecretBytes`, a fixed 32-byte value with a private array, derived
`Zeroize` and `ZeroizeOnDrop`, manual redacted `Debug`, and no `Clone`,
`Copy`, `Display`, Serde, `Deref`, `AsRef`, or binding annotation. Its only raw
view is `expose_secret(&self) -> &[u8; 32]`.

`HDKey` and `EdHDKey` each expose `expose_private_key()` and
`expose_chain_code()`, returning a fresh `HdKeySecretBytes`. The method call is
the audit boundary; the returned value owns the new copy and erases that copy
on drop. The borrowed view cannot outlive that owner. A caller can still make
another copy deliberately, which remains caller-owned and outside the SDK's
guarantee.

### D3 — No compatibility shim for public fields

Rust cannot deprecate a public field while preventing direct copying. A method
returning an ordinary array would preserve the original hazard under a new
spelling. Because the API is unpublished and pre-release, the four fields are
removed directly and the candidate API baseline is regenerated. This is
source-breaking relative to the internal `0.1.0-rc.1` review baseline, but it
does not break a released package or wire/persisted representation.

### D4 — Compile-time negative contracts

Trybuild cases prove external field access, `Display`, and Serde serialization
do not compile. Runtime tests prove the exposure wrapper's redaction and
explicit zeroization, the HD types' existing erasure contract, and byte-exact
published and Apollo-overlap vectors through the new explicit boundary.

## Threat and copy-boundary analysis

| Boundary | Before | After | Residual risk |
| --- | --- | --- | --- |
| Long-lived HD state | Public arrays can be copied by an ordinary field read. | Arrays are private and borrowed internally. | Compiler/platform copies remain outside Rust's semantic guarantees. |
| Explicit export | No auditable boundary or erasure owner. | Named method creates one redacted, drop-zeroizing owner. | Caller can deliberately copy the borrowed bytes. |
| Formatting | HD types redact; a raw copied array can be formatted. | HD types and exposure owner redact. | Bytes copied by a caller can be formatted by that caller. |
| Serialization | HD types do not serialize; public arrays can be serialized directly. | No field access and neither HD nor exposure type implements Serde. | A caller can explicitly serialize a deliberate copy. |
| FFI/bindings | Public Rust fields are visible to Rust consumers but are not exported by SDK binding definitions. | Opaque state and exposure owner have no binding annotation. | A later FFI export requires a separate security/FFI decision. |
| Drop | HD-owned arrays zeroize. | HD-owned arrays and SDK-created exposure copies zeroize. | Allocator, registers, swap, crash dumps, hostile hardware and caller copies are excluded. |

## Alternatives

- Return `Zeroizing<[u8; 32]>`: rejected because its derived `Debug` reveals
  the inner array and exposes a third-party concrete type as the facade.
- Callback-only access: viable, but callbacks do not define an erasure owner
  for SDK-created copies and are awkward for ordinary encoding/signing APIs.
- Borrow the long-lived field directly: rejected because a returned reference
  makes raw access less explicit and offers no export-copy owner.
- Keep deprecated public fields: impossible to enforce and inappropriate for
  a known secret-copy hazard in an unpublished candidate.

## Rollback

A source revert restores the old fields and baseline. No wire data, persisted
state, published artifact, or downstream repository requires migration.
