# Pre-implementation architecture, API, standards, security and performance review

- **Date:** 2026-09-05
- **Issue:** #98 under #8 / #20 / `IDR-004`
- **Develop base:** `54f46e9b94ce3fe66f9e2c04c389051549ac39a4`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. RFC 9864 updates the original issue assumptions: fully specified JOSE
   `Ed25519` is the forward algorithm and polymorphic `EdDSA` is deprecated.
   Existing wallet interoperability still needs the old spelling, so it must
   be a distinct explicit compatibility suite rather than a default alias.
2. RFC 8725 requires three equal choices at operation time: protected-header
   algorithm, caller-selected key algorithm and allowed cryptographic suite.
   Dispatching directly from untrusted header text is insufficient.
3. The current `PublicKeyJwk` validates public structure and coordinate width
   but not algorithm binding. A borrowed wrapper is the narrowest additive
   state and avoids cloning extensions.
4. Optional JWK `alg` is retained as an uninterpreted extension today. Binding
   must reject a non-string or unequal value, while absence is valid because
   the explicit wrapper supplies the other binding mechanism.
5. Header `kid` cannot authorize or even uniquely select a key in this slice.
   Matching it would incorrectly absorb DID resolution/relationship policy.
6. A registry with exact, duplicate-free entries is the caller allowlist.
   A hard maximum of 16 makes attacker-influenced or misconfigured dispatch
   bounded; linear lookup is simpler and predictably faster at that size than
   adding a hash-map dependency.
7. Verification must consume exact received signing-input bytes. The registry
   cannot rebuild a header or payload and must pass the existing borrowed
   slice to the chosen capability.
8. `VerifiedCompactJws` proves only one registered cryptographic suite accepted
   one bound public key. Its API and docs must not imply DID authorization,
   JWT claims validity, issuer trust or freshness.
9. The existing Ed25519 path already uses strict verification and produces
   fixed 64-byte signatures. Reuse it directly.
10. The inherited P-256 path intentionally uses DER. RFC 7518 instead requires
    64 raw bytes, so additive primitive fixed-width methods belong in
    `identus-crypto`; adapting through DER inside JOSE would duplicate parsing
    and couple protocol code to a compatibility representation.
11. Both built-in signatures are exactly 64 bytes. Rejecting length before
    verifier invocation provides a shared static boundary and ensures DER can
    never be accidentally accepted as ES256.
12. A synchronous object-safe signer is sufficient for the known software,
    secure-element, HSM and agent boundaries at this layer. Async runtimes,
    transport, cancellation and retry belong to outer adapters; standardizing
    them now would be speculative.
13. The signer receives public message bytes only. Typed software adapters may
    borrow private-key objects from crypto but JOSE must expose no byte import,
    export or custody API.
14. `identus-jose -> identus-crypto` is an inward credential-semantics to
    domain-primitives edge allowed by the executable ring. ADR 0034's
    foundation-only first-slice decision must be superseded explicitly and the
    canonical crate-ring requirement updated.
15. External suite implementations are part of caller trust. Built-in suites
    provide reviewed behavior; the SDK cannot make an arbitrary trait
    implementation honest, so registration must remain an explicit act.
16. All provider, coordinate, signature, compact, payload and `kid` values must
    stay out of errors and Debug. Static error categories are sufficient for
    branching and telemetry.
17. RFC 8037 Appendix A supplies immutable legacy Ed25519 evidence. A fixed
    software-key round trip supplies the RFC 9864 `Ed25519` form, and an
    independent fixed P-256 round trip proves exact ES256 representation.
18. Portable target claims remain feasible: accepted RustCrypto dependencies
    already compile in the current crypto target matrix; no OS, network, async,
    chain or product dependency is introduced.
19. Workspace dependency inheritance would otherwise activate every default
    crypto feature under JOSE. The root internal edge must disable defaults and
    JOSE must request only `ed25519`/`secp256r1`. DID explicitly retains its
    current all-on compatibility edge because narrowing it exposes pre-existing
    ungated crypto integration tests; that cleanup is separate from #98.
20. Signature verification is expected to dominate the at-most-16 linear
    dispatch. Measure it as a release diagnostic without a machine-specific
    pass threshold.

## Implementation review amendments

1. Signing and verification initially consumed their prepared/unverified
   inputs. That made transient provider failures and attempts with another
   authorized key unnecessarily rebuild or pre-clone bounded wire state.
   Both operations now borrow input; only successful verification takes one
   bounded owned clone for the evidence wrapper.
2. The first exact-input verifier test reparsed a canonical builder value and
   did not prove preservation across received JSON whitespace/member order.
   The final case constructs a non-normalized protected header independently
   and asserts the verifier sees the exact received encoded segments.
3. The JOSE dependency initially inherited every crypto default feature. The
   root internal edge now disables defaults and JOSE requests only Ed25519 and
   P-256; a dependency-tree assertion is recorded in delivery evidence.
4. The external signer initially returned an arbitrary `Vec<u8>` and checked
   its length afterward. Every closed algorithm in this slice has a 64-byte
   wire signature, so the port now returns `[u8; 64]`; invalid widths and an
   unnecessary provider-controlled allocation are impossible by construction.
5. Registry lookup initially called `algorithm()` again on the selected trait
   object. Registration now captures the algorithm beside the capability, so
   even a stateful implementation cannot change its dispatch identity later.
6. The recommended registry initially used an exact capacity of two, which
   made the documented explicit legacy opt-in impossible on that instance. It
   now starts with the same two recommended suites while retaining bounded
   capacity for deliberate additions.

Verdict: READY to implement after strict OpenSpec validation and issue #98
reflect the RFC 9864 compatibility decision.

# Post-implementation architecture, API, standards, security and performance review

- **Date:** 2026-09-05
- **Reviewed source:** `develop@54f46e9b94ce3fe66f9e2c04c389051549ac39a4`
  plus the complete staged issue #98 change
- **Result:** no unresolved finding

## Findings and dispositions

1. Public state remains explicit: parsing and signing return
   `UnverifiedCompactJws`; only a successful registry operation can construct
   `VerifiedCompactJws`, which documents that DID authorization, claims,
   freshness and trust remain unproven.
2. The forward allowlist contains fully specified `Ed25519` and `ES256`.
   Deprecated `EdDSA` is a distinct Ed25519-only suite that requires an
   explicit registration and matching key binding.
3. Protected header, bound key, optional JWK `alg`, key family/curve and
   registered suite must agree exactly and case-sensitively before built-in
   cryptography runs. `kid` authorization correctly remains #99 work.
4. External signing receives only the exact public signing-input slice and
   returns `[u8; 64]` or a static failure. Private-byte import/export,
   provider errors, async runtime and transport policy do not enter JOSE.
5. The registry captures each suite's algorithm exactly once at registration.
   A regression with a stateful implementation proves dispatch does not trust
   later changes to that method.
6. Review found the recommended registry's initial capacity of two prevented
   its documented explicit legacy/custom extension. It now starts with only
   the two recommended algorithms but retains the hard 16-entry capacity; a
   regression proves deliberate extension succeeds.
7. Ed25519 uses strict verification. ES256 uses exactly 64 raw big-endian
   `R || S` bytes, rejects DER at the JOSE boundary, and reconstructs a
   validated P-256 point before verifying.
8. Additive `sign_fixed`/`verify_fixed` methods reuse the accepted RustCrypto
   primitive. Existing P-256 DER `sign`/`verify` behavior and tests remain
   unchanged and green.
9. Verification passes the original encoded header and payload segments to
   the suite without JSON reserialization. Signing passes exactly
   `JwsSigningInput::as_bytes()`; independent recorder tests prove both.
10. Registry capacity, duplicate detection and signature widths are bounded.
    All new errors and Debug views omit provider detail, keys, coordinates,
    signatures, compact text, payload and `kid` canaries.
11. Arbitrary external suites remain caller-trusted capabilities. The SDK
    cannot make a caller implementation honest; explicit registration and
    the conformance-backed built-ins are the correct trust boundary.
12. The JOSE dependency cone activates only Ed25519 and P-256. DID explicitly
    retains its prior all-on crypto compatibility because its integration
    tests are not yet individually feature-gated; narrowing that edge is a
    follow-up rather than hidden scope expansion.
13. Host all-feature/no-default suites and the pinned Nix MSRV, browser WASM,
    Android ARM64, iOS ARM64, supply-chain and lint matrix all pass.
14. The release observation of approximately 25.6k registry-mediated
    Ed25519 verifications per second establishes a baseline without a
    machine-specific threshold.
15. Oxid and Lace ID Portal remain at their pinned revisions with only their
    pre-existing untracked paths. No donor code or fixture was copied.

Verdict: READY for canonical synchronization, archive and issue-linked
delivery after the immutable factory receipt is recorded.

# Archive synchronization amendment

- **Date:** 2026-09-05
- **Result:** generated semantic-loss finding resolved

OpenSpec correctly created the new canonical signature-capability spec, but
its mechanical application of the crate-ring `MODIFIED` delta replaced the
complete existing layer-rule requirement with the abbreviated delta text.
That removed unrelated normative scenarios even though structural validation
still passed. The canonical requirement was restored byte-for-byte from the
reviewed implementation checkpoint, which already contains the intended
JOSE-to-crypto wording. The historical delta remains archived, the new
capability spec has a concrete purpose, and strict factory validation passes
with no unrelated canonical requirement loss.

Verdict: RESOLVED before the archive commit.
