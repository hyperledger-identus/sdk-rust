# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/324
Constraint blockers: none

## Existing entries affected

`SDK-REL-001`, `SDK-REL-002`, `SDK-COMPAT-003`, ADR 0113, ADR 0120, and
`RELEASING.md` continue to prohibit representing preparation as publication or
turning temporary Rust 1.98.1 evidence into a release promise. Branch and
chain/product ownership constraints are unchanged.

## Introduced or changed constraints

The public handbook becomes required M3 approval evidence. Its source is
reviewed on `develop`; its deployment is an exact GitHub Pages artifact from
that branch. Pages publication cannot publish crates, tag a release, mutate
`main`, or weaken two-person release authority.

## Introduced or changed limitations

The first site documents only the three candidate crates as a release train.
Other implemented packages remain source-only experimental components and are
not implied to be released. The site links source/API-generation instructions
until a real registry release gives docs.rs a stable artifact identity.

## Consumer and product impact

Engineers and downstream evaluators gain a stable architecture and readiness
surface. No consumer manifest, runtime, protocol, ledger, wallet, custody,
certification, or support promise changes.

## Activation and rollback

Activation requires issue-bound preflight, deterministic Nix build, offline
links, workflow lint, factual review, signed/DCO PR, green exact-head CI and
review, protected merge, successful Pages deployment, and public URL
inspection. Rollback disables Pages and reverts workflow/site/Nix sources; it
does not alter Cargo artifacts or release evidence already retained elsewhere.

## Evidence

Issue #324 is the exact sponsor-directed publication record. M3 issue #326
keeps documentation publication separate from candidate release approval.

