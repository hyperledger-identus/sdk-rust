# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@0630dc38f50119ac2ba4ca532dcd542935d079c7
Implementation head: a96d2c7bfcca229911ecf10e7aa02ba5783c9070
Reviewed head: a96d2c7bfcca229911ecf10e7aa02ba5783c9070
Specification commit: 722a9e0beb348600dae4c6e43ad09b54a2b942eb
Preimplementation receipt commit: f95a9a3cb1afaf510c4d5ea613f1050a84ee4745
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issues #364 and #366,
ADR 0147, OpenID4VCI 1.0 Final sections 6.2 and 8.2, RFC 6750 section 2.1,
the immutable preimplementation receipt, private serializer reuse, state
ownership, validation precedence, redaction, compatibility and
focused/workspace/portable verification.

## Findings

1. **Authority continuity — accepted.** The new method exists only on the
   correlated success and consumes it. Endpoint, token response and selected
   identifier come from the retained lineage/state; callers cannot provide a
   replacement metadata document, token, configuration or identifier.
2. **Selection and ownership — accepted.** One source-order index selects the
   existing unique zeroizing identifiers. Missing selection returns the
   existing static diagnostic. Success duplicates only the public endpoint and
   moves secrets into the existing zeroizing request representation; failure
   drops the consumed authority state.
3. **Reuse and cohesion — accepted.** Both compatibility constructors and the
   new request-bound path call one private serializer. Bearer grammar, proof
   validation, complete Authorization/body limits, deterministic JSON and
   redaction therefore retain one implementation and one error vocabulary.
4. **Compatibility — accepted.** The public API is additive and unpublished.
   No prior constructor, error, discriminant, stable code, wire shape,
   dependency, feature, lockfile, unsafe or stored-data contract changed.
5. **Resource/privacy boundary — accepted.** The method adds no unbounded
   input. Identifier retention was already bounded during Token Response
   parsing, proofs and complete output are governed by existing positive
   limits, and Debug contains sizes/counts rather than bearer/proof/dataset
   material.
6. **Architecture — accepted.** The slice proves structural request continuity
   only. It performs no HTTP and claims no token, issuer, dataset, proof or
   Credential trust, freshness or authorization. Response binding remains
   focused successor #366.
7. **Delivery integrity — accepted.** All three branch commits verify with
   good OpenPGP signatures and DCO sign-offs. The live roadmap points to open
   successor #366 and passed its GitHub-state audit.

## Decomposition decision

The exact change spans 20 paths and 766 added text lines, above the preferred
12-file guidance but below the 1,000-line guidance. Executable production work
is a 29-line public transition, a 16-line crate-private move helper and a
mechanical extraction of the existing serializer; 246 lines are focused
consumer-shaped tests. The remaining paths are the mandatory OpenSpec research,
specification, ADR and machine-readable inventory/roadmap evidence. Splitting
these artifacts would separate a security-relevant authority transition from
its governing contract. Response handling is already decomposed to #366, so no
further functional split is warranted.

## Residual limitations

- The access token, dataset identifier and proofs remain opaque and untrusted.
- The caller still owns HTTP origin, TLS, redirects, decompression, timeout,
  cancellation, retry, credential validation and secure storage.
- Credential Endpoint response binding remains issue #366.
- Publication and downstream adoption remain separate release/consumer work.

## Review decision

The implementation is bounded, cohesive, one-shot, redaction-safe, additive
and reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains.
