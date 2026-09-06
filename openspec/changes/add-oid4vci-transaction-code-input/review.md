# Semantic contract review

## Scope reviewed

- Issue #123 and exact base `b22ba18cde2b6e420499ee8922ab1bd2f3e2ad1e`.
- OpenID4VCI 1.0 Final sections 3.5, 4.1.1, and 6.1 at immutable HTML
  SHA-256 `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Existing grant, server-binding, error, limit, and redaction contracts.
- Read-only Oxid and Lace evidence listed in issue #123.

## Findings

1. **Resolved — avoid overstating Transaction Code validity.** The draft
   boundary proves exact input presence and resource safety only. Advertised
   mode/length remain UI guidance; the Authorization Server remains the code
   validator.
2. **Resolved — erase failed input.** The design requires wrapping the owned
   string before empty/size checks so rejection paths zeroize it.
3. **Resolved — avoid a premature public secret accessor.** The prepared state
   exposes presence only; later in-crate request construction may borrow raw
   material privately.
4. **Resolved — keep protocol scope bounded.** Token serialization, HTTP,
   client identity/authentication, responses, replay, and downstream adoption
   are explicit non-goals.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No unresolved blocker remains before implementation.

# Post-implementation exact-diff review

- **Date:** 2026-09-07
- **Reviewed production head:** `3fd92d97ca3cc3f8c9c518fb8dc491d3960fd18f`
- **Exact diff:** `develop@b22ba18c...3fd92d97`
- **Method:** fresh architecture/API/standards/security/resource review after
  focused, workspace, and full Nix gates
- **Result:** no unresolved finding

## Exact-diff findings

1. The only public construction path consumes the server-bound predecessor;
   successful construction therefore preserves the established offer,
   metadata, selected Authorization Server, grant-support, and Token Endpoint
   invariants.
2. A present `tx_code` object, including `{}`, requires input, while an absent
   object rejects input. The transition does not reinterpret `input_mode`,
   `length`, or `description` as code-validation policy.
3. Caller-owned input moves into `Zeroizing<String>` before presence, empty,
   and byte-bound validation. Rejected owned values and successful retained
   values consequently share the same erasure boundary without cloning.
4. The positive configurable limit defaults to 256 decoded UTF-8 bytes. Byte
   length is checked exactly, including multibyte values, with no trim,
   normalization, parsing, or data-dependent allocation.
5. The success state exposes its predecessor and a presence bit only. It has
   no public raw-secret accessor or serialization contract; later in-crate
   Token Request work can consume the private field without widening this API.
6. Debug output contains only the type and presence bit. Five new fieldless
   errors map to stable, static `oid4vci.*` contracts, and canary tests cover
   the direct and bridged Debug/Display surfaces.
7. Eight focused tests cover positive, negative, exact-bound, multibyte,
   predecessor, advisory-metadata, limit, and diagnostic branches in both
   feature modes.
8. No manifest, lockfile, feature, parser, existing limit, dependency, HTTP,
   runtime, crypto, DID, storage, consumer, chain, or product code changed.
   The normal dependency cone is unchanged.
9. ADR 0047, blueprint, backlog, and OpenSpec consistently describe an input
   presence/resource proof—not correctness, authentication, replay safety, or
   a serializable Token Request.
10. Focused/workspace Cargo and all 27 compatible Nix checks pass, including
    Rust 1.85, WASM, Android, iOS, strict lints/docs, supply-chain policy, and
    the 487-test principal suite. Consumer receipts match preflight exactly.

Verdict: READY for specification synchronization and pull-request review.
