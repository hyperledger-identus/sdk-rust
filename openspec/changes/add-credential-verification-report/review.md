# Pre-implementation semantic, API, security, and performance review

- **Date:** 2026-09-05
- **Issue:** #73, child of #6 / `IDR-009` and #20
- **Develop base:** `4e4cf5a8b4fb66e289629d287b4b87c63d5fc2bf`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Metadata/schema and verification are separate backlog capabilities. This
   slice should not reproduce Oxid's aggregate record or join IDR-007b to
   IDR-009a.
2. Oxid's structural, issuer, proof, temporal, status, and schema stages are
   reusable. Its trust stage is product policy and must be excluded.
3. A caller-supplied aggregate outcome creates a contradictory-state problem.
   Deriving the outcome removes the invariant instead of repeatedly checking
   it.
4. A six-element array is preferable to a vector/map/set: completeness is
   partly type-level, validation is bounded, lookup is direct, and no report
   collection allocation is required.
5. Fixed canonical ordering is an acceptable experimental API constraint. A
   new normative stage changes outcome semantics and deserves a compatibility
   decision.
6. Failed and not-checked states both need machine reasons. NotChecked without
   a reason would collapse unsupported, unavailable, skipped, and inapplicable
   evidence.
7. Bounded lower-ASCII reason codes permit stable namespaces without allowing
   dynamic error prose or secrets into portable records.
8. Operational verifier errors and mode-specific evidence payloads remain in
   adapters. The report records their contracted projection only.
9. A manual release diagnostic is appropriate for this small hot path; a wall-
   clock CI threshold would be noisy and is not acceptance evidence.
10. Donor and consumer repositories remain read-only. No build, branch switch,
    dependency repoint, generated artifact, or source deletion is needed.

Verdict: READY to implement after strict structural validation.

# Post-implementation semantic, API, security, and performance review

- **Reviewed production implementation:** `aee8789c37841d63470369d0860172e1ae6a8e62`
- **Exact diff:** `develop@4e4cf5a...aee8789`
- **Result:** passed with no unresolved finding

The exact production diff adds one private module to the existing credential
crate and extends its existing error boundary. `Cargo.toml`, workspace
features, the lockfile, and the normal dependency graph are unchanged. The
public dependency cone remains `identus-core` and the previously governed
`zeroize` dependency; there is no chain, resolver, clock, network, storage,
serde, format codec, policy, product, or donor dependency.

The report owns exactly six stages in a fixed array. Construction scans that
array once, rejects any non-canonical name, tracks failed/not-checked state in
the same pass, and derives the only aggregate outcome. Because callers cannot
provide an outcome and cannot provide fewer or more than six array elements,
the public constructor cannot create a contradictory or incomplete report.
Lookup uses a closed enum-to-index mapping without a map or search.

Reason validation examines the borrowed bytes before the only successful
allocation. It accepts only the bounded machine-token grammar, and invalid
input is neither retained nor reflected through local or shared errors.
Accepted codes are deliberately observable evidence rather than free-form
diagnostics. The report contains no proof bytes, credential claims, keys,
operational errors, or other secret-bearing payload.

Trust remains outside the taxonomy and types. Tests demonstrate both a valid
report rejected by a product trust flag and an invalid report accompanied by a
trusted-issuer flag. Midnight- and Cardano-shaped status projections use the
same core report without importing either implementation.

Focused, workspace, and all 26 compatible `aarch64-darwin` Nix checks passed.
That includes Rust 1.85 MSRV, pinned nightly, native, WASM, Android, iOS,
feature, test, documentation, formatting, conformance, factory, dependency,
license, and advisory lanes. The release diagnostic observed approximately
41.4 million all-passed report constructions per second on this host; no time
threshold is encoded.

Read-only final receipts equal preflight: Oxid
`bfe3b481568dc738f0732c2b27548fab8721fd95` on `integration` with its existing
untracked agent directories; midnight-identity
`427f8571950c42967a18726cbcbefecc19ef8d79` on `develop` with its existing
dirty `third_party/midnight-did`; Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` on `main` with its existing
untracked agent/tmp directories; and NeoPRISM
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` on `main`, clean. No downstream
state changed.

Verdict: READY to synchronize, archive, receipt, and publish as a signed/DCO,
issue-linked pull request to `develop`.
