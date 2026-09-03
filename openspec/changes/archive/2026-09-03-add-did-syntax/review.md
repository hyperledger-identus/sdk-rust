# Semantic, parser and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #34 (child of #5 / `IDR-005`)
- **Develop base:** `49b2ca29b6d6a63a240f110fa89a2d500204152a`
- **Source revisions:** Apollo `ccee22b`; NeoPRISM `8becb22`;
  midnight-identity `427f857`; Lace ID Portal `804de0a`; Oxid `685f967`
- **Target maturity:** 75% for this lexical slice
- **Estimated implementation effort:** 2–4 agent-hours, excluding hosted CI
- **Started:** 2026-09-03T01:24:00Z
- **Reviewed implementation head:** `443bfa93984459a8a789f86721ba7a7afd778b31`
- **Result:** no unresolved blocker; suitable for exact-head hosted review

## Pre-implementation findings

1. **Ownership:** DID and DID URL lexical values are generic SSI primitives
   and belong in sdk-rust. PRISM and Midnight semantics belong in their method
   layers; wallet policy and user experience remain in Oxid.
2. **Ordering:** this small lexical boundary unblocks document, resolver and
   method-port work without importing those concerns. It is therefore a lower
   risk next step than starting JOSE or a complete DID document model.
3. **Provenance:** NeoPRISM supplies useful test behavior but delegates to
   IOTA Identity. midnight-identity's current checks are too shallow. Apollo,
   Lace and Oxid are compatibility evidence. The implementation will be
   standards-derived and copy no donor source.
4. **Architecture:** a self-contained single-pass parser avoids a large
   dependency and makes resource, error and serialization behavior explicit.
   Cached byte offsets make the frequently read component views free of
   allocation.
5. **Security:** byte limits are applied before scanning or allocation;
   errors expose invariant categories but never rejected input. Lexical
   acceptance remains explicitly separate from trust, method and resolution
   policy.
6. **Compatibility:** the API is additive in an unpublished crate, adds no
   dependency and remains compatible with wasm/mobile targets. Package naming
   and publication remain owned by issue #3.

## Post-implementation review

The complete `origin/develop...443bfa9` diff was reviewed afresh after the
focused and full Nix gates passed.

1. **Grammar fidelity:** the parser applies the exact lowercase DID prefix and
   method alphabet, DID Core identifier alphabet, complete percent escapes and
   RFC 3986 `pchar`/query/fragment alphabets. Exhaustive tests classify every
   raw ASCII byte, with delimiters tested according to their structural role.
2. **Offset safety:** cached indexes are discovered while scanning raw ASCII
   delimiters. Every successful offset is therefore a UTF-8 boundary and
   remains immutable with its owned string. Accessors cannot observe a stale
   offset because there is no mutation API.
3. **Resource behavior:** overall length is checked before grammar scanning;
   an embedded DID inside a DID URL is independently capped at 2 KiB. The
   parser is linear and dependency-free. Borrowed parsing allocates once only
   after success; owned parsing and `Did -> DidUrl` preserve the allocation.
4. **Delimiter behavior:** a DID URL is first scanned to `/`, `?` or `#`, then
   each suffix is parsed in its own grammar. Bare `Did` parsing rejects all
   three suffix delimiters. Empty query and fragment are stored as present
   offsets, distinct from absence.
5. **Redaction:** new local reason variants contain no string payload. Native
   and serde failures therefore cannot echo the rejected identifier, and the
   public bridge exposes only the two stable codes and static messages.
6. **Layering:** no dependency or Cargo feature changed. The API stays in the
   unpublished `identus-did` crate; method policy, documents, resolution,
   publication and downstream adoption remain outside the diff.
7. **Portability:** Rust 1.85 MSRV, default native, wasm32, Android aarch64 and
   iOS aarch64 builds all pass, together with clippy and rustdoc.
8. **Performance:** the release diagnostic parsed and allocated 500,000
   representative DID URLs in 87.887334 ms, about 5,689,102 parses/second on
   the local aarch64 Darwin host. This is observational evidence, not a flaky
   compatibility threshold.

## Corrections made during review

- Enforced the 2 KiB DID limit on the embedded DID prefix of a 4 KiB DID URL,
  preserving the invariant required by infallible `DidUrl::to_did()`.
- Replaced an unobservable "non-slash-led path" scenario with precise invalid
  component-byte language; without a slash, bytes remain part of the DID
  method-specific identifier rather than a distinguishable path.
- Replaced an empty-input substring assertion with a unique caller marker so
  the redaction test proves the intended property without a false positive.
- Added exhaustive ASCII-class tests and explicit lower/upper-case percent
  escape coverage after the initial focused suite passed.

Residual risk is explicit: lexical validity does not establish DID method
registration, identifier equivalence, resolution, document authenticity or
key authorization. Continuous generative testing is tracked by follow-up #35,
not hidden inside this bounded slice.
