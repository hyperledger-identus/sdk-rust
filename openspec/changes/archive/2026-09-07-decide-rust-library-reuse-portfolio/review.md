# Architecture, research, factory and exact-diff review

- **Date:** 2026-09-07
- **Issue:** #151
- **Develop base:** `516d91a7c2a783e86f19f2172168a6d716212303`
- **Result:** no unresolved blocker

## Semantic findings

1. ADR 0061 draws the dependency boundary at closed algorithms and grammars.
   It does not delegate SDK policy, trust, transport, lifecycle or security
   states to a third-party framework.
2. The five approved production candidates each remove a concrete duplicated
   mechanism and remain private behind an existing Identus-owned facade.
   Every integration is isolated in a separate issue and reversible PR.
3. Conditional and spike candidates have objective entry conditions. Oracle
   candidates cannot enter runtime release artifacts through this decision.
4. The negative ledger uses finite reason codes, an explicit current
   alternative and a reconsideration trigger; it is a maintained decision
   record rather than an irreversible blacklist.
5. ADR 0062 replaces the ambiguous “slightly above average” criterion with a
   measurable stable-minus-three policy. Quarterly review does not mutate the
   floor automatically.
6. The ADR does not misrepresent present compatibility: Cargo, Nix and machine
   policy continue to promise Rust 1.85 until focused issue #154 proves and
   atomically implements Rust 1.95.
7. The MSRV decision balances actively maintained crate access with an
   approximately 18-week consumer qualification window, explicit exceptions
   and independent target gates.
8. The research report distinguishes declared MSRV from observed resolution,
   compile probes from integration/runtime evidence, and zero OSV results from
   a security audit.
9. The OpenSpec requirements are testable and fully covered by the ordered
   tasks. This is a new capability, so there is no canonical delta collision or
   concurrent active change touching it.
10. `research.md` is deliberately repository-owned rather than added to the
    pinned OpenSpec schema. The factory, proposal/apply adapters, semantic
    review prompt, intake templates and governance all point to the same
    contract.

## Exact-diff findings

1. No Cargo manifest, lockfile, Rust source, public API, wire format, runtime
   dependency, Nix toolchain or effective support-policy value changes in this
   PR.
2. `check-research-readiness.py` is offline and fail-closed for missing files,
   invalid class/status/date metadata, undeclared blockers, missing sections,
   absent dispositions and incomplete full-assessment evidence.
3. Draft records pass structural checks but fail `--require-ready`; ready
   records with blockers fail both paths. Six focused tests exercise the
   positive, negative, full and routine contracts.
4. `scripts/factory research-ready` validates the repository, selected change,
   strict OpenSpec contract and zero-blocker research record before returning
   success. Apply adapters stop when it fails.
5. The semantic-review prompt now reads and evaluates research evidence; it
   does not treat the machine shape check as dependency approval.
6. The issue portfolio #152 through #164 matches the ADR/report dispositions,
   prerequisites and sequencing. No rejected candidate received a production
   integration issue.
7. Full Nix evaluation ran 25 compatible checks. Rust 1.85 surfaces, current
   workspace/default and focused feature tests, WASM, iOS, Android, Clippy,
   docs, format, supply-chain policy, factory and text gates passed; the
   principal suite ran 587 tests successfully.
8. The inherited `scripts/__pycache__/` remains untracked and was excluded from
   the diff. Downstream repositories were inspected read-only and were not
   changed; pre-existing dirty state in midnight-identity and Lace was left
   untouched.

## Corrections made during review

1. Staged the new test source so Nix's Git-backed flake source included it,
   while explicitly unstaging inherited Python bytecode.
2. Added research evidence to the semantic-review prompt and factory receipt,
   then aligned its disposition vocabulary with the executable checker.
3. Removed Markdown trailing whitespace and aligned the governance state
   diagram.
4. Corrected the prompt to name `Research status: ready` and include blocker
   B9 in its reviewer guidance.
5. Hosted review identified three actionable gaps: incomplete machine evidence
   categories, noncanonical negative-ledger reason spelling and two archived
   relative links. The checker/tests, ledger and links were corrected before
   merge.
6. The hosted signature finding was disproved against both the local object and
   GitHub Git API: commit `da50c9d5ec00e663371ef62c14fbf85ae3d9c8fd`
   has a valid `gpgsig` and DCO trailer. The correction commit is subject to the
   same gates.

## Post-archive validation note

The repository-pinned factory validated all 45 canonical capabilities, and the
new `dependency-research-readiness` capability also passes the unpinned latest
OpenSpec validator. The latest CLI rejects 22 unrelated existing capabilities
whose generated Purpose text predates its new placeholder warning. That
out-of-scope historical cleanup is not represented as a failure of this
capability or silently folded into this PR.

Verdict: READY; the guarded archive and final pinned gates passed, so the work
may proceed to a signed commit and issue-linked pull request.
