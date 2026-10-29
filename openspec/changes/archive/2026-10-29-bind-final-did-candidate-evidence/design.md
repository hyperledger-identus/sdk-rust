# Design

## Durable report

Add one repository-owned Markdown report under `docs/release/` and expose it
from the handbook release section. The report is a closed table of candidate
identity, package responsibilities, evidence classes, immutable receipts,
limitations, remaining approval, and downstream compatibility. Unknown or
pending evidence is displayed as pending, never inferred from an aggregate
artifact or an older run.

## Exact-revision flow

The PR changes only evidence documentation and its governing OpenSpec delta.
Local checks qualify the candidate and site without modifying canonical
manifests. After merge, the protected merge SHA becomes the evidence subject.
A complete natural or explicitly authorized slow run must execute that exact
SHA. The final GitHub receipt then links the run, artifacts, report revision,
NeoPRISM receipts, and independent approval.

## E2E knowledge boundary

The report links rather than copies NeoPRISM evidence. It explains that the
candidate branch consumed an immutable SDK source revision and that the
published-vs-candidate Preprod harness compared canonical operations plus
duration, CPU, RAM, restarts, cursor, and drain state. NeoPRISM remains owner of
Cardano, persistence, runtime, and network promotion.

## Failure behavior

Any failed, cancelled, rerun-only, wrong-branch, stale, or different-SHA slow
run leaves readiness pending. A successful aggregate receipt cannot override a
failed required lane. New advisory or target failures receive focused issues;
the report is not edited to waive them.

## Verification and rollback

Run release candidate, source-distribution, site, factory, OpenSpec, link, and
text checks locally. Obtain a Codex or Claude exact-head review before push.
After hosted fast CI is green, merge through the guarded path, then run/observe
the full exact-SHA slow evidence. The additive report can be reverted without
changing code, crates, tags, registry state, or downstream repositories.
