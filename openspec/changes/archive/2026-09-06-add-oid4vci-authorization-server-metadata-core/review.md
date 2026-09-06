# Pre-implementation architecture, API, standards and security review

- **Date:** 2026-09-06
- **Issue:** #119 under #7 / #20 / `IDR-023`
- **Develop base:** `1d94a634994ab4da7d5e902216e8f47106e4bbce`
- **Result:** contract was implementable with no unresolved blocker

## Findings

1. The observed consumer document is not complete RFC 8414 Authorization Server
   Metadata because it omits `response_types_supported`. A type named
   `AuthorizationServerMetadata` would overclaim conformance, so this slice owns
   only an explicitly named `AuthorizationServerMetadataCore` projection.
2. OID4VCI 1.0 Final delegates Authorization Server Metadata to RFC 8414 and
   adds `pre-authorized_grant_anonymous_access_supported`. The flag is optional,
   defaults to false, and must preserve advertised presence separately from its
   effective value.
3. RFC 8414 makes `grant_types_supported` optional with effective defaults
   `authorization_code` and `implicit`. Omission must therefore remain
   distinguishable from an explicit list; serializing invented defaults would
   falsify the received advertisement.
4. Issuer comparison is a caller-provided trust binding. Exact equality after
   safe HTTPS identifier parsing is appropriate here; discovery, aliasing,
   normalization policy, authorization-server selection and HTTP remain outside
   this value-layer component.
5. Authorization and token endpoints are retained only when advertised and are
   validated as HTTPS identifiers. Their conditional RFC 8414 requirements
   depend on broader response/grant metadata and cannot be claimed by this
   partial projection.
6. A present grant list must be non-empty, ordered and duplicate-free. Retaining
   order gives lossless projection semantics; rejecting duplicates prevents
   ambiguous capability policy in consumers.
7. Unknown metadata must be ignored semantically while exact input remains
   available for higher-layer processing. The bounded selective scanner avoids
   assigning policy to extensions and accepts arbitrary-magnitude unknown JSON
   numbers without coercing them through a host numeric type.
8. Existing scanner depth/node/string/collection limits are extended with
   authorization-metadata-specific defaults. This prevents unbounded work and
   allocation before product code evaluates an untrusted document.
9. Metadata may contain correlating endpoint and issuer values. Debug surfaces
   and errors must remain static/redacted, and retained input strings must
   zeroize on drop.
10. The smallest runtime cone remains `identus-core`, `serde_json`, `uriparse`
    and `zeroize`; no HTTP, async runtime, crypto, DID, credential, chain or
    product dependency is justified.
11. Consumer repositories are evidence-only. No donor code is copied, and Lace
    remains behavior-only evidence because its repository-level license posture
    is unresolved.
12. This slice cannot claim full RFC 8414 validation, metadata discovery,
    server selection, authorization/token messages, trust establishment or
    OID4VCI flow completion.

Verdict: READY to implement after ADR 0045 and strict OpenSpec validation pass.

# Post-implementation architecture, API, standards and security review

- **Date:** 2026-09-06
- **Reviewed production head:** `3591e03d663838c5b6c2236ea28f786c0e1188d5`
- **Exact diff:** `develop@1d94a634...3591e03d`
- **Result:** no unresolved finding

## Exact-diff findings

1. `AuthorizationServerMetadataCore` is a bounded, lossless projection and its
   public documentation explicitly disclaims complete RFC 8414 validation.
   Crate-level documentation carries the same boundary.
2. Issuer binding is exact and fail-closed. Issuer and optional endpoints reuse
   the existing safe HTTPS identifier policy without discovery, network I/O,
   trust inference, redirects or server-selection behavior.
3. The API exposes advertised grants separately from effective RFC defaults.
   Omission yields the ordered `authorization_code`, `implicit` default without
   causing exact JSON or advertised-state accessors to invent metadata.
4. The OID4VCI anonymous pre-authorized-access flag retains `Option<bool>` while
   its effective accessor defaults omission to false. Explicit false is not
   collapsed into absence.
5. Present grant lists reject empty, duplicate, non-string and oversized input.
   Endpoints reject non-HTTPS and unsafe identifier shapes; the expected issuer
   is independently validated before exact comparison.
6. Duplicate relevant keys, malformed nested JSON, trailing content, limit
   overruns and incorrect types fail closed through static error codes. Unknown
   fields remain uninterpreted, including an arbitrary-magnitude numeric fixture.
7. Exact input, issuer, endpoints and grant identifiers use zeroizing storage.
   Debug output omits caller values, and diagnostic canaries prove neither the
   parsed object nor errors disclose their input.
8. The dependency cone and lockfile are unchanged. The implementation adds no
   transport, runtime, crypto, chain, product or consumer dependency.
9. Nine focused tests cover consumer and standards shapes, both default paths,
   exact boundaries, invalid syntax/types/duplicates, malformed JSON and
   redaction. Both default-feature and no-default-feature lanes pass.
10. Factory, workspace and all 27 compatible local Nix checks pass, including
    Rust 1.85 MSRV, WASM, Android, iOS, strict lints, docs, supply-chain policy
    and the 470-test principal release suite. Consumer receipts match preflight.

## Implementation-review corrections

1. The initial crate-level documentation named only the earlier offer parsing
   surface. It was extended in `08c6acc` to name the new partial projection and
   explicitly disclaim full RFC 8414 validation and trust establishment.
2. The first exact Nix run found nonconforming left padding inside a multiline
   JSON test fixture. Commit `3591e03` aligned it with repository EditorConfig;
   focused tests and the complete 27-check Nix matrix then passed unchanged.

Verdict: READY for specification synchronization and pull-request review.
