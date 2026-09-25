# Exact-diff protocol, architecture and security review

Review status: completed
Review date: 2026-09-26
Implementation head: 9daede220561e40b7a6e66b69fba0555b8bc6bb6
Specification commit: 52e8e9b3245f17fb6287d718bbb2f11c757c3d74
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@c5543efc807c5bbcd8a9bd1d45f4282c10b58057...9daede2` diff,
issue #391, ADR 0156, exact candidate manifest and lock, adapter tests,
isolation checker, dependency ledgers, and local verification output.

## Findings

1. **Protocol authority — accepted.** OID4VCI and OID4VP 1.0 Final remain
   normative. Candidate behavior is explicitly evidence and cannot weaken SDK
   validation or redefine a wire contract.
2. **Layering decision — accepted.** Existing bounded `identus-oid4vci`
   remains production code. Full frameworks are oracles; the only reuse path
   advanced is the independently cohesive DCQL mechanic.
3. **Graph isolation — accepted.** The fixture is an unpublished nested
   workspace with an exact lock. No SIROS package or type enters the root
   workspace, release graph, SDK crate, or public facade.
4. **Resource and privacy boundary — accepted.** The adapter checks 16 KiB
   before parsing and maps UTF-8/candidate failures to two category-only
   errors. Verifier bytes and identifiers do not enter Debug output.
5. **Useful behavior — accepted.** Public candidate APIs select only a
   complete format-compatible, holder-bound credential and exclude missing
   claims and unbound credentials.
6. **Semantic mismatch — accepted as conditional-adoption evidence.** The
   candidate intentionally accepts a missing `meta` and dotted identifiers.
   Tests preserve both facts; future production use must add strict structural
   and identifier validation in an Identus-owned facade.
7. **Dependency and supply chain — accepted.** Exact `siros-dcql 0.3.0` has a
   12-package resolved cone, BSD-2-Clause provenance, Rust 1.82 declaration,
   no I/O/native/unsafe source, and clean deny/audit results.
8. **Targets — accepted with bounded claim.** Host primary and MSRV compile
   evidence passed, as did primary compile checks for WASM, iOS, and Android.
   The documents explicitly exclude linking, packaging, and runtime support.
9. **Maintainability — accepted.** The experiment is 160 lines of isolated
   adapter/test code and one checker. It does not duplicate a production
   implementation or create a permanent compatibility promise.
10. **Delivery scope — accepted.** No workflow, release, support policy,
    downstream repository, or production behavior changes.

## Residual limitations

- The fixture bounds bytes but does not define the complete future OID4VP
  structural limit table or strict identifier grammar.
- It does not test presentation construction, cryptography, trust, consent,
  transport, nonce handling, deployed interoperability, or runtime packaging.
- Production reuse still requires a named OID4VP consumer, accepted public
  facade, complete target/runtime evidence, and an ADR update.

## Review decision

The research is reproducible, isolated, reversible, and sufficient to retain
the current OID4VCI implementation while classifying SIROS DCQL as a promising
conditional private dependency. No unresolved protocol, correctness,
architecture, security, privacy, dependency, licensing, or delivery finding
remains for hosted review.
