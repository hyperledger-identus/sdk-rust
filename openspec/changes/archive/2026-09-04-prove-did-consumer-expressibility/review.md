# Pre-implementation semantic and provenance review

- **Date:** 2026-09-04
- **Issue:** #65 (child of #5 / `IDR-005` and `IDR-006`)
- **Develop base:** `d3380abc1b2238cc9ccfe4071104c2243ec02ba7`
- **Reviewed contract:** OpenSpec `prove-did-consumer-expressibility`
- **Result:** no unresolved blocker

## Findings

1. The parent issue explicitly recommends a bounded consumer-compatibility
   slice and the canonical backlog still marks DID Core/ports incomplete.
2. Issue #50 is not triggered because two independent consumers have not
   recorded matching cancellation semantics; this change does not implement
   request coalescing.
3. Public integration tests are the smallest executable evidence because they
   test the consumer-facing API without adding a conformance API commitment.
4. The four cases intentionally cover different intersections and must not
   overstate method, runtime, transport, cryptographic or product compatibility.
5. Lace remains evidence-only. Independently authored standards-shaped input
   avoids copying code or fixtures from a repository whose license evidence is
   unresolved.
6. Existing strict constructors, limits, private-JWK rejection and public error
   redaction remain normative; compatibility evidence cannot weaken them.
7. No new crate, dependency, wire form or public API is needed by the reviewed
   design. Discovery of such a need would reopen semantic review before code.

# Post-implementation provenance, security and API review

- **Reviewed head:** `f1b7603ff319341180efbf692187f0349fd5be9d`
- **Review completed:** 2026-09-04
- **Result:** passed with no unresolved finding

The exact diff from the recorded `develop` base was re-read after focused,
workspace and full Nix verification. The implementation changes only a public-
API integration test, an architecture evidence matrix and OpenSpec artifacts;
Cargo manifests, lockfiles and production Rust are unchanged.

All four cases cite the immutable revisions from the accepted source matrix.
Their values are synthetic and independently authored from W3C fields and
public API observations. No donor source, fixture, credential, identifier or
private key is copied. Lace remains explicitly evidence-only.

The cases preserve the existing strict entry points and verify lossless open
fields, public-only key material, private JWK rejection, redacted malformed-DID
failure, explicit legacy-keyword migration and exact multi-method dispatch.
They do not claim method semantics, downstream compilation, runtime interop,
cryptographic verification, HTTP, VDR, storage, custody, trust or compliance.

No new dependency, feature, unsafe code, network access, chain type, runtime or
public API enters the diff. Issue #50 remains dormant and unmodified. The
change is independently reversible and does not touch `main` or a consumer.
