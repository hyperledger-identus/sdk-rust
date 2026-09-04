# Pre-implementation semantic, API, privacy, and performance review

- **Date:** 2026-09-05
- **Issue:** #79 under `IDR-008` / #20
- **Develop base:** `9b0d10790b9257f94fc9eecd2af99a82c592ca70`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Oxid's request/candidate domain is the strongest Rust seed, but its product
   IDs, labels, preview object, age threshold, and lifecycle enum mix reusable
   semantics with wallet policy. Only the bounded request/query/candidate
   intersection is adapted.
2. OpenID4VP Final makes format, unique query IDs, multiplicity, claims, and
   candidate selection real requirements. Its JSON/DCQL metadata, credential
   sets, claim sets, values, and protocol parameters belong to later protocol
   crates rather than this format-neutral core.
3. The credentials crate already owns format, issuer/type/schema descriptors,
   and segmented claim paths. Duplicating them would violate DRY and create
   inconsistent bounds. A one-way same-layer dependency is allowed by the
   crate-ring and preserves cohesion.
4. An unbounded raw request string, as used at the Oxid application port, is a
   transport artifact. The semantic request instead contains bounded validated
   queries and an opaque bounded challenge.
5. A challenge is meaningful across OID4VP nonce and Midnight proof contexts,
   but generation, freshness, replay storage, and comparison are protocol
   behavior. Exact opaque text/bytes preserve both without pretending they are
   interchangeable on the wire.
6. Predicate parameters do not have a shared safe type across DCQL, mdoc, and
   Midnight circuits. The generic core records reveal/predicate intent only;
   each profile retains its typed condition and proof semantics.
7. Optional issuer/type/schema filters are shared structural requirements, but
   matching and trust are not. `None` is the only unrestricted spelling;
   present empty/duplicate lists fail during construction.
8. Candidate construction alone cannot validate cross-query facts. A separate
   candidate-set constructor borrows the request and rejects unknown queries,
   wrong formats, unrequested paths, missing required paths, and duplicate
   query/handle pairs.
9. Candidate paths must be a subset of requested paths. Accepting extra paths
   would make accidental over-disclosure easier even though actual disclosure
   remains outside this slice.
10. Empty candidate sets are valid no-match evidence; empty request query sets
    are invalid. Distinguishing them removes sentinel objects and preserves
    normal search outcomes.
11. Verifiers, purposes, challenges, handles, filters, and paths can correlate
    a person or transaction. Custom aggregate Debug plus static data-free
    errors must avoid recursively formatting imported descriptor contents.
12. Bounds precede all pairwise scans. The largest scan is 64 by 64, small and
    predictable enough to avoid hash-map allocation while the API remains
    unpublished and measurable.
13. Midnight, Lace, Apollo, and NeoPRISM provide compatibility or negative
    evidence but no competing licensed generic Rust presentation core. Their
    repositories remain read-only and no dependency is justified.

Verdict: READY to implement after strict structural validation.

# Post-implementation architecture, API, privacy, and performance review

- **Reviewed production implementation:**
  `1304ec79d15423bbdddcf3ce52264e57485cbe3d`
- **Exact diff:** `develop@9b0d107...1304ec7`
- **Result:** passed with no unresolved finding

The exact production diff replaces only the quarantined presentation marker,
updates its inventory classification, and adds one permitted inward workspace
edge to `identus-credentials`. The lockfile records that local edge only. No
external package, version, feature, wire, storage, runtime, network, chain,
product, protocol, or donor dependency changed.

The API reuses credential-owned format, entity, type, schema, and segmented
claim-path types. It owns only bounded presentation roles: query correlation,
purpose, opaque replay challenge, disclosure intent, structural filters,
requests, opaque local credential handles, and request-validated candidates.
Every untrusted scalar and collection is validated at construction, before any
bounded pairwise scan. The maximum scan is 64 by 64 and no unbounded work is
accepted.

`PresentationCandidateSet` validates facts that individual candidates cannot:
request membership, exact format agreement, requested-path subset, required-
path coverage, and unique query/handle pairs. Empty sets remain valid no-match
evidence. The `multiple` flag belongs to later selection/submission policy, so
the candidate set may retain multiple available choices without claiming they
may all be submitted.

Custom Debug implementations expose only kinds, lengths, booleans, counts, and
the non-sensitive open format identifier. They do not recursively format
verifier, purpose, challenge, issuer, schema, claim path, query ID, or local
handle values. All local and shared error messages are static and data-free.
The core does not contain claim values, predicate parameters, proof material,
credential artifacts, keys, consent decisions, or trust judgments.

Focused tests, both native workspace feature matrices, strict Clippy, warning-
denied documentation, factory validation, and all 30 compatible
`aarch64-darwin` Nix checks passed. The flake covered pinned nightly, Rust 1.85
MSRV, native, Android, iOS, WASM, feature, documentation, test, formatting,
architecture, dependency, license, advisory, and factory lanes. Its release
workspace lane ran 323 tests successfully with 13 configured skips.

The manual release diagnostic on Apple Silicon with rustc 1.95.0 constructed
and request-validated 250,000 representative request/candidate pairs in
165.798792 ms, approximately 1,507,852 pairs/second. This is an observation,
not a portable threshold.

Read-only final receipts equal preflight: Oxid
`bfe3b481568dc738f0732c2b27548fab8721fd95` on `integration` with its existing
untracked `.claude/` and `.pi/taskflows/`; midnight-identity
`427f8571950c42967a18726cbcbefecc19ef8d79` on `develop` with its existing
dirty `third_party/midnight-did`; Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` on `main` with its existing
untracked agent/tmp directories; NeoPRISM
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` on `main`, clean; and Apollo
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` on `main` with its existing dirty
nested secp256k1 worktree. No downstream state changed.

Verdict: READY to synchronize canonical specs and publish as a signed/DCO,
issue-linked pull request to `develop`.

Canonical synchronization retained the existing governance-evidence scenarios
and added the new presentation activation requirement. OpenSpec generated a
placeholder purpose for the new canonical presentation specification; it was
replaced with the reviewed format-neutral capability intent before final
validation.
