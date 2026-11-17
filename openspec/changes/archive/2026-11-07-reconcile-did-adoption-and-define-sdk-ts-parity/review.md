# Exact-diff and evidence-integrity review

Review date: 2026-11-07
Reviewer role: independent post-implementation review
Verdict: approved after findings were resolved

## Immutable review inputs

- Candidate head: `a5fabed13101a664295cf9655c0909a5d3078515`
- Base and merge base: `f07719d606ade1f0387854535ca9ac029811cd3a`
- Branch: `codex/docs/issue-594`
- Authority: issue #594, issue #502, this OpenSpec change, ADR 0163, and
  `SDK-LIM-009`
- Review becomes stale if the candidate or base SHA changes.

## Acceptance checklist

| ID | Requirement | Evidence | Status | Limits |
| --- | --- | --- | --- | --- |
| AC-1 | Maintain one checked Rust/SDK-TS feature ledger over all normalized capabilities. | 24 exact machine rows joined to the pinned SDK-TS outcome and ownership inventory; human rows are checked field-for-field. | satisfied | No implementation route is activated. |
| AC-2 | Define browser, Node.js, React web, and React Native success independently. | Four exact target records and per-capability target states; React Native retains the ADR 0163 native TurboModule boundary. | satisfied | Runtime versions and native engine qualification remain future evidence. |
| AC-3 | Reuse applicable SDK-TS E2E journeys and separate Rust deviation evidence. | SC-03 and SC-04 define closed donor and Rust evidence dispositions with completeness and no-silent-skip rules. | satisfied | This change defines, but does not populate, the later test-level ledgers. |
| AC-4 | Define coverage, type-safety, reuse, maintainability, operability, and rollback criteria. | SC-05 through SC-08 are machine-locked and rendered exactly in the report. | satisfied | Performance remains measurement-only until budgets are accepted. |
| AC-5 | Preserve Rust-core evidence while isolating target failures under the single protected status. | SC-09 and the CI contract require core-first evidence, fail-closed impact routing, non-cancelling target jobs, success-only selected terminals, and one protected `fast` aggregator. | satisfied | No workflow is added or changed in this planning slice. |
| AC-6 | Prevent unsupported completion or delivery claims. | Baseline forbids delivered/excluded routes; schema v2 rejects milestone completion until executable SC-01 through SC-09 receipts exist; delivered rows require all dimensions, targets, and no missing proof. | satisfied | A later reviewed schema is required to represent completion. |

## Findings and resolutions

1. Browser/Node storage was initially recorded as browser-only. The target
   applicability now includes Node.js and matches the accepted donor inventory.
2. Target and success tables plus success semantics could initially drift while
   the checker passed. All target fields, SC-01 through SC-09 semantics, both
   human tables, and inventory ownership columns are now checked exactly with
   adversarial mutations.
3. A future active milestone could initially claim delivery while retaining a
   contradictory missing-proof sentence, or claim completion through excluded
   rows without executable criterion receipts. Missing-proof/status invariants,
   exclusion target consistency, and fail-closed schema-v2 completion rules now
   prevent those paths.
4. Selected CI job cancellation, skip, timeout, or absence was initially less
   explicit than failure. The contract now requires successful exact-head
   terminal results for every selected job.
5. Peer DID inventory and report text incorrectly described Rust support as
   absent. They now record the bounded numalgo-4 and opt-in numalgo-2 crate,
   fixtures, and quality evidence while retaining `partial` for missing
   TypeScript, consumer, per-target, and rollback proof.

No unresolved correctness, architecture, security, operations,
evidence-integrity, ownership, or scope finding remains.

## Review limitations

- The independent review was read-only and did not run hosted CI or target
  builds.
- This change defines future CI and runtime contracts; it does not claim those
  jobs, artifacts, packages, runtime matrices, or support tiers exist today.
- External source availability was not re-proved; immutable revisions and
  offline record consistency were checked.
