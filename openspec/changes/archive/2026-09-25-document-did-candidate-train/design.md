# Design

## Information architecture

Keep the existing first-crypto-train page intact but rename navigation/prose so
it is clearly one train rather than the whole release story. Add one dedicated
DID candidate page under the release section and add the two packages to the
crate index as candidate-only entries linking to their repository READMEs.

## Diagram

Add a tracked DOT source for the DID train. It shows the optional Axum adapter
depending on the featureless generic DID domain/ports crate, which in turn uses
published foundation crates. Chain methods, ledgers and products sit outside
the candidate boundary. The existing locked build renders SVG; generated
artifacts remain untracked.

## Source evaluation and claims

Examples use the exact protected merge revision from #385, never a branch,
pull-request ref, nonexistent tag or `0.0.0` version selector. Text distinguishes
the candidate version rendered in isolated archives from canonical source-only
manifests. API snapshots and normalized CycloneDX are review evidence, not a
stable compatibility promise, advisory result or signed provenance.

## Reconciliation

Update landing, start, release-train, crate-index, adoption, readiness and
limitations prose only where their global statements became stale. Preserve
the first crypto train's publication mechanics and approval history. M5 links
to #382/#384 and open successors #386/#387/#388.

## Verification and rollback

Build the complete site with tracked diagrams and offline link validation,
then inspect the generated DID page and SVG. Run Markdown and factory/OpenSpec
checks. All changes are static/reversible and do not mutate code or external
release state.
