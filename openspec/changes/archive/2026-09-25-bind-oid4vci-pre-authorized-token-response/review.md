# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-25
Base: `develop@46a94e882159492a9125f1468bf362bee9374d57`
Implementation head: `f55e7067d4ce6d621f129023a934e37dde1b8c4b`
Reviewed head: `f55e7067d4ce6d621f129023a934e37dde1b8c4b`
Specification commit: `930519add7c889ba8faa6a6ee060e6909736402c`
Preimplementation receipt commit: `8f4a670`
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #375, OpenID4VCI
Final sections 6.2/6.3, RFC 6749 sections 5.1/5.2, request ownership and drop
order, status/header/body precedence, exact offer/server lineage, error
contracts, shared private HTTP grammar, public API, conformance/roadmap claims,
tests and portable builds.

## Findings

1. **Least-authority lineage — accepted.** The request preserves matched
   issuer/server metadata and ordered offered configuration IDs, but the
   grant-bearing Credential Offer is explicitly dropped. Its exact JSON—and
   therefore its Pre-Authorized Code—never becomes response lineage.
2. **Secret lifetime — accepted.** The request owns the Pre-Authorized Code and
   optional Transaction Code only in one `Zeroizing<String>` form. The
   response transition consumes the request and drops that form before status,
   header or body validation. Invalid remote input cannot recover the request.
3. **Branch integrity — accepted.** Exact `200` and `400` choose success and
   error parsers before any other response semantic. Body shape cannot switch
   branches. The current unauthenticated request does not infer an authenticated
   `401` path.
4. **Resource and cache boundary — accepted.** Content-Type, Cache-Control and
   Pragma have independent positive limits and fixed validation order before
   existing bounded body parsers. Required directives use the previously
   tested strict private grammar.
5. **Cohesion and coupling — accepted.** Only the value-free private header
   failure enum and validator are shared. Flow-specific public limits,
   lineage, outcomes and diagnostics remain independent; existing
   Authorization Code tests prove behavior compatibility.
6. **Diagnostics and privacy — accepted.** Eight append-only errors have
   unique static codes/messages and no input fields. Request/outcome Debug
   exposes structural counts and booleans only; token cores retain their
   existing redaction and zeroization boundaries.
7. **Dependency and portability — accepted.** No manifest, lockfile, feature,
   unsafe/native or dependency change exists. The exact crate cone is
   unchanged and WASM/iOS/Android compile checks pass.
8. **Claim strength — accepted.** The matrix advances only this required row.
   It continues to exclude HTTP origin, TLS, client authentication, trust,
   token validity/storage, retry, issuance and product behavior. IDR-023 and M4
   remain open under #376.
9. **Delivery integrity — accepted.** Planning preceded implementation, the
   receipt remains valid, every branch commit is signed and DCO-bearing, and
   the full workspace/factory/portable evidence is green locally.

## Decomposition decision

The diff planner reports 29 paths and 1,628 text lines. Most are the complete
OpenSpec contract, tests, ADR and synchronized conformance/roadmap evidence.
The runtime behavior is one cohesive typed transition: request construction
must retain exactly the lineage consumed by the response binder, and private
validator extraction is required to avoid duplicate token-envelope grammar.
Splitting it would leave a secret-bearing request with incomplete authority or
an unbound public continuation. Independent fixture/provenance work is already
isolated in #376.

## Review decision

No unresolved correctness, security, privacy, compatibility, architecture,
dependency, provenance or delivery finding remains in issue #375.
