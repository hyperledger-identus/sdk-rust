# Exact-diff architecture and security review

- **Reviewer:** Codex fresh contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `1d627801bda5e220857467b9376fcfc2f72fe908`
- **Reviewed implementation:** `7af4a6c91ef5e769cc4897d2946e005c9a98f713`
- **Scope:** issue #157, ADR 0084, OpenSpec contract, manifests, lockfile,
  DID/OID4VCI parsers, policy wrappers, tests and dependency source
- **Result:** ready; zero unresolved blockers

## Findings

1. **Verified — the public boundary remains Identus-owned.** No public
   signature names `fluent-uri` or one of its types. `Uri`, credential issuer,
   endpoint/reference and token types retain their existing owned strings,
   accessors, serde behavior and static errors. Candidate errors and component
   views are discarded immediately after validation.
2. **Verified — protocol policy remains local.** Byte ceilings execute before
   parsing. OID4VCI still requires HTTPS, a non-empty host, no userinfo, no
   fragment and the existing call-site query policy. URI-reference fields keep
   their non-empty/visible-ASCII and field-size rules. The candidate does not
   perform transport, trust, resolution or normalization.
3. **Verified — accepted and rejected grammar behavior is covered.** Existing
   independent RFC vectors and the 1,000-case `uriparse` differential remain
   green. Direct tests cover IPvFuture and malformed percent encoding; existing
   tests cover authority, userinfo, query/fragment, IRI rejection, exact
   spelling and the 4,096-byte DID ceiling.
4. **Verified — runtime duplication is reduced.** The local DID generic parser
   and OID4VCI IPvFuture substitution are removed. `uriparse`, `fnv` and
   `lazy_static` are absent from both crates' normal dependency graphs;
   `uriparse 0.6.4` remains only as the DID development oracle.
5. **Accepted residual — five package names enter the lock.** The candidate
   resolves through `borrow-or-share`, `ref-cast`, `ref-cast-impl` and a second
   `syn` major. The `syn` duplication is build-time-only and remains visible as
   a `cargo deny` warning. No native code, FFI, runtime, network or clock enters
   the graph.
6. **Accepted residual — transparent-reference unsafe is transitive.**
   `fluent-uri` and `borrow-or-share` forbid unsafe source. `ref-cast` uses
   sealed unsafe traits and macro-generated casts after checking transparent
   representation. This integration reaches immutable parser component views,
   adds no SDK unsafe block and exposes no dependency view. Mutable reachability
   requires a new review.
7. **Verified — provenance and licenses are explicit.** The reviewed package is
   the crates.io artifact for exact 0.4.1, checksum
   `bc74ac4d8359ae70623506d512209619e5cf8f347124910440dbc221714b328e`,
   matched to release commit `d9a6a20614f34b00476837eb8904fb01ca3e54df`.
   The MIT-0 transitive license is OSI-approved and was added to the global
   allowlist explicitly; the license gate passes without a crate exception.
8. **Verified — scope and rollback are focused.** `identus-core::Url`, `Did`,
   `DidUrl`, public APIs, wire representations, consumers and release policy do
   not change. Reverting this PR restores the previous parsers without data
   migration.

## Corrections made during review

1. Corrected the preliminary research link from a later repository commit to
   the exact VCS revision recorded by the published 0.4.1 artifact.
2. Added explicit malformed-percent and direct IPvFuture regressions at the
   OID4VCI boundary.
3. Added MIT-0 only after `cargo deny` rejected the previously unlisted
   transitive license, and documented the policy decision in ADR 0084.
4. Corrected the DID manifest with the repository TOML formatter after the
   first Nix pass found its alignment mismatch.

## Decision

The implementation satisfies issue #157 and ADR 0084. The transitive
transparent-reference unsafe and build-time `syn` duplication are bounded,
attributed residuals rather than blockers. The change is approved for guarded
archive and an issue-linked PR to `develop`.
