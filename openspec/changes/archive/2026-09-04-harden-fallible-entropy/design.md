## Context

`identus-crypto` owns the `SecureRandom` port while
`identus-adapters-entropy` owns concrete implementations. The current port
returns an allocated vector and has no failure channel. The getrandom adapter
therefore calls `expect`; fixed-width key constructors call `expect`; and both
elliptic-curve generators panic after sixteen invalid scalar draws.

The rich `crypto::Error` already contains `SecureRandomFailure`, bridged to the
stable redacted `crypto.secure_random_failure` code. The workspace is
unpublished at version `0.0.0`, so correcting the port now is preferable to
preserving the unsafe shape in future bindings.

## Goals / Non-Goals

**Goals:**

- make every entropy failure observable without panic;
- prevent the entropy port from allocating based on caller-controlled length;
- keep entropy injection portable, synchronous, object-safe in spirit, and
  independent of an async runtime;
- preserve fixed algorithm behavior and stable redacted error semantics.

**Non-goals:**

- changing curves, signatures, derivation, BIP-39 parameters, key encodings,
  custody, zeroization policy, async cancellation, or entropy-provider choice;
- adding dependencies, concrete adapters to the domain crate, or downstream
  repository changes;
- providing a compatibility shim for unpublished APIs.

## Decisions

### D1 — A fallible caller-owned buffer port

`SecureRandom` exposes:

```rust
fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Error>;
```

The caller determines storage and length. Current cryptographic consumers use
fixed `[u8; 32]` buffers, so an adapter cannot be made to allocate an unbounded
vector by a large length argument. A zero-length slice is valid and succeeds.

Returning the existing domain error keeps the trait small and avoids a second
adapter-error type whose details would have to be erased immediately. Concrete
adapters map any backend failure to `Error::SecureRandomFailure`.

Rejected alternatives:

- `try_generate_seed(usize) -> Result<Vec<_>, _>` remains allocation-amplifiable;
- an infallible `fill_bytes` retains panic as the only backend-failure policy;
- associated adapter error types complicate injection and leak backend shape
  into the domain contract without useful recovery behavior.

### D2 — Random constructors return `Result`

`Ed25519KeyPair::generate`, `X25519KeyPair::generate`,
`Secp256k1KeyPair::generate`, `P256KeyPair::generate`,
`MnemonicHelper::create_random_mnemonics`, and
`MnemonicHelper::create_random_seed` return `Result<_, Error>`. Failures are
propagated directly; no wrapper or panic is introduced.

### D3 — Bounded EC rejection sampling remains bounded

secp256k1 and P-256 retain the existing maximum of sixteen scalar draws.
Provider failure returns immediately. Sixteen successfully filled but invalid
candidate scalars return `Error::SecureRandomFailure`; no candidate bytes or
curve detail enters the error.

### D4 — Deterministic adapter fills the entire slice

The test-only adapter writes its documented repeating byte sequence into every
element and returns `Ok(())`. It remains stateless and reproducible. The
production getrandom adapter delegates to `getrandom` and maps its opaque
failure to the existing crypto error.

## Risks / Trade-offs

- **Source compatibility:** all callers must handle `Result`. The workspace is
  unpublished and all in-tree callers are changed atomically.
- **Error specificity:** invalid-scalar exhaustion is categorized as secure
  random failure. This deliberately avoids a new stable code for a condition
  callers can only recover from by retrying with working entropy.
- **Partial buffer writes:** a failing backend may have modified part of the
  caller's buffer. Callers initialize fixed buffers to zero, return immediately
  on error, and never construct or expose a key from failed output.
- **Retry count:** retaining sixteen preserves current behavior and bounds work;
  changing statistical policy is outside this slice.

## Migration Plan

1. Land the specification and ADR.
2. Change the port, both adapters, constructors, tests, and documentation in a
   single PR.
3. Verify default/all-feature workspace builds plus native and browser-WASM
   adapter builds.
4. Archive the validated OpenSpec change into the canonical specifications.

Rollback is a source revert; there is no persisted or wire-format state.
