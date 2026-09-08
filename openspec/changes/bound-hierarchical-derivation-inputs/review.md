# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Implementation head: 6427303c6dca7965ea65b9b1f014e165ee3f46a3
Specification parent: 6f247815b7a1e6248826c2d8b4c9b9365927d6e4
Unresolved blockers: none

## Scope reviewed

The review inspected the complete specification-parent-to-implementation diff,
all derivation entry points, the path parser, stateful depth transitions,
Cardano typed-path consumers, public re-exports, boundary tests and the
constraint-register update. It reconciled the implementation against issue
#199, parent resource audit #168, Apollo parity issue #9, the OpenSpec delta,
BIP-32 and SLIP-0010.

## Findings

1. **Resource-first parsing — accepted.** Text length is checked before root
   comparison or segment processing. The parser streams segments into one
   bounded vector and rejects the 256th axis before attempting its syntax.
2. **Cryptographic work ceiling — accepted.** Every HD and Cardano path
   consumer preflights complete work before the first HMAC or curve operation.
   Individual stateful child operations reject a depth-255 parent before HMAC.
3. **Seed envelope — accepted.** Both BIP-32 and SLIP-0010 constructors enforce
   the standards-backed 16-through-64-byte seed range before HMAC. The Cardano
   V2 dependency's distinct constructor contract is unchanged.
4. **Compatibility — accepted.** All in-range vectors and wire output remain
   unchanged. Public additions are constants plus path introspection; new
   failures retain the SDK-owned, redacted `Error::DerivationFailed` family.
5. **Boundary precedence — accepted after improvement.** Exact-diff review
   added an otherwise-valid 4,097-byte path case and public exact-depth-255
   transitions in commit `6427303c6dca7965ea65b9b1f014e165ee3f46a3`, proving the ceiling rather than relying
   only on malformed over-limit input or internal helpers.
6. **Dependency and target surface — accepted.** No manifest, lockfile,
   feature, FFI, unsafe, build-script, MSRV or resolved dependency change is
   present. The private `ed25519-bip32` boundary remains encapsulated.
7. **Scope and cohesion — accepted.** The change does not mix wallet policy,
   serialization, public derivation or a fallible-builder redesign into a
   generic hierarchical-derivation resource fix.
8. **Documentation — accepted.** The OpenSpec requirement and `SDK-LIM-007`
   state the exact byte, axis, depth and seed boundaries and preserve the
   outer-allocation caveat.

## Residual limitations

- The source `str`, owned `String` or typed path may already have been
  allocated by a caller; outer transports still need their own input budgets.
- The source-compatible infallible programmatic append API can construct more
  than 255 axes. Every current cryptographic consumer rejects such a value
  before proportional cryptographic work.
- Local Nix omitted x86_64-linux as host-incompatible; hosted `fast` remains
  the merge authority for Linux.

## Review decision

The implementation is focused, fail-closed and consistent with the specified
resource and compatibility boundaries. No unresolved correctness,
architecture, security, privacy, compatibility or supply-chain blocker remains
for local delivery.
