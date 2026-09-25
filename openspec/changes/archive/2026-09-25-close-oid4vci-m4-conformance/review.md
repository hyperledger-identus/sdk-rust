# Exact-diff architecture and evidence review

Review status: completed
Review date: 2026-09-25
Base: `develop@7f23129a245072e34ea5ddf1e601e14a4b493ed5`
Implementation head: `70a1a31fb41aafb66cee38384a37929b0cf98e79`
Reviewed head: `70a1a31fb41aafb66cee38384a37929b0cf98e79`
Specification commit: `03ee4c2c2e94e2c3066a2959c2f9f5c10218c49e`
Preimplementation receipt commit: `9ade5fed678445ce179339858b72d8da1d67f384`
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #372, issue #7,
OpenID4VCI 1.0 Final sections 4 through 12, the consumer-evidence provenance,
the closed matrix schema, checker failure modes, factory-fixture integration,
backlog transition and focused/workspace/portable verification.

## Findings

1. **Normative coverage — accepted.** Every top-level Final section from 4
   through 12 is represented, along with a separate consumer-profile row.
   Section identifiers support arbitrarily deep numeric subsections without
   accepting sections outside that range.
2. **Claim strength — accepted.** `implemented`, `partial`, `unsupported` and
   `missing` are evidence labels rather than compliance scores. The report
   explicitly excludes HTTP provenance, trust, credential correctness,
   storage, product policy and official certification.
3. **Evidence integrity — accepted.** Implemented and partial rows require
   existing implementation, canonical-spec and test files. Paths must be
   relative, normalized, regular, non-symlink repository files and cannot
   escape the checkout. Duplicate identifiers and evidence paths fail closed.
4. **Gap ownership — accepted.** Required missing rows cannot claim local
   execution evidence and require a focused live issue. The functional seam is
   #375; the qualified generic consumer-vector seam is #376. IDR-023 advances
   to #375 and remains in progress, so this report cannot falsely close M4.
5. **Provenance and licensing — accepted.** The inspected Oxid and Lace ID
   Portal revisions, hashes and product-specific profile are recorded without
   copying fixture bytes. The Portal license ambiguity and Midnight-specific
   vectors force `reference-only` treatment until #376 resolves both concerns.
6. **Factory enforcement — accepted.** The matrix, report, checker and mutation
   tests are required factory assets. The synthetic repository copies the
   exact canonical OID4VCI specifications used as evidence before exercising
   the integrated checker.
7. **Compatibility — accepted.** The slice changes no Rust source, manifest,
   lockfile, feature, dependency, wire, stored-data or public API contract.
   The exact `identus-oid4vci` dependency cone is unchanged.
8. **Delivery integrity — accepted.** Every reviewed branch commit has a valid OpenPGP
   signature and DCO sign-off. The preimplementation receipt binds the exact
   planning commit, base SHA, issue and branch before implementation.
9. **Review hardening — accepted.** Dedicated section 10 and 11 dispositions
   cannot be collapsed into generic section coverage, and the live backlog
   audit requires every matrix follow-up owner—including #376—to remain open.

## Decomposition decision

The shipping addition is one cohesive evidence system: a 24-row matrix, its
human interpretation, one 180-line offline checker and seven mutation tests.
It deliberately does not implement either discovered gap. Splitting the
checker from the matrix or report would permit evidence drift; #375 and #376
already isolate the two independent follow-up concerns.

## Review decision

The change is bounded, reproducible, fail-closed and honest about unsupported
and missing behavior. No unresolved correctness, security, privacy,
compatibility, architecture, dependency, provenance or delivery finding
remains in this closeout slice.
