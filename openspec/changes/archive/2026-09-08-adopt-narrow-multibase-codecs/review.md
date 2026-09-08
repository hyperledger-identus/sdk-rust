# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Implementation head: 2f604631e52cacd0bb49f99bb3160de640b40c01
Specification parent: 5b315ffde7ccf341b3a5f51735934d0fe579c315
Unresolved blockers: none

## Scope reviewed

The review inspected the complete specification-parent-to-implementation diff,
the resolved normal dependency graph, selected crate source paths, new
accepted/rejected tests, every changed fixture and the public DID document
facade. It also reconciled the implementation against issue #156, ADR 0085,
the OpenSpec delta and the repository constraint register.

## Findings

1. **Dependency cohesion — accepted.** The integrated lock adds only
   `bs58 0.5.1`; `tinyvec` was already locked and is not enabled in the
   DID normal graph. `multibase`, `base-x`, `base45`,
   `base256emoji` and their macro packages are absent.
2. **Public boundary — accepted.** No upstream type, error, alphabet or target
   trait is public. `VerificationMethod::public_key_multibase()` still
   returns the exact borrowed string and valid JSON remains unchanged.
3. **Resource safety — accepted after correction.** The first research draft
   retained 16 KiB. A release-mode worst-case probe demonstrated
   disproportionate Base58 work, so the spec and implementation now reject
   above 4 KiB before codec allocation. Whole-document and item budgets remain.
4. **Canonicality — accepted.** Only `z` and `u` dispatch. Both paths
   decode to non-empty bytes and compare the selected engine's re-encoding to
   the complete payload. Unknown prefixes, Base58 alphabet violations,
   Base64 padding/trailing-bit aliases and empty payloads fail closed.
5. **Semantic scope — accepted.** The helper name and public documentation say
   carrier rather than key validation. No multicodec, curve, length,
   cryptographic verification, did:key resolver or multihash claim is made.
6. **Diagnostics/privacy — accepted.** Candidate errors and input text are
   discarded. Native construction maps to the existing static
   `DocumentError::InvalidString`; raw JSON remains within the existing
   redacted invalid-document family. Decoded bytes are public key material.
7. **Unsafe boundary — accepted.** The SDK adds no unsafe block. The dependency
   has one scoped mutable-`str` output implementation; the SDK calls only
   owned string/vector APIs and exposes no dispatch path to that target.
8. **Compatibility fixtures — accepted.** Three DID/JOSE placeholders that
   were not Base58 were replaced with the Controlled Identifiers example.
   The JOSE verifier still proves that syntactically valid unsupported
   multibase material cannot be substituted for a supported JWK.

## Residual limitations

- Encodings other than `z` and `u`, and recognized carriers above 4 KiB,
  fail closed.
- Carrier validity does not prove supported key semantics.
- `bs58 0.5.1` declares no MSRV and its last source commit is from 2024;
  exact pinning, target gates and advisory monitoring remain necessary.
- Local Nix omitted x86_64-linux as host-incompatible; hosted `fast` is the
  merge authority for that system.

## Review decision

The implementation is focused, reversible and consistent with the specified
dependency-first boundary. No unresolved correctness, architecture, security,
privacy, compatibility or supply-chain blocker remains for local delivery.
