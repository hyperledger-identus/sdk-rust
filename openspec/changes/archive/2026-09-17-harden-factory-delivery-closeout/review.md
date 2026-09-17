# Local Review

## Scope and identity

- Issue: `#320`
- Base: `81593db3418d5b70990ec3c31ae8ddc3a4bc6edf`
- Reviewed implementation head: `5d32d87338dc1c969f079fba21f81b54c352b850`
- Review context: fresh post-implementation architecture, security, process,
  test, and documentation pass by the supervising Codex session.

## Findings

1. **Resolved — mutable supervisor wrapper race.** The bounded Pi canary edited
   `scripts/factory` while that wrapper was still interpreting the supervisor
   branch. The worker completed successfully, but the outer shell later parsed
   the new generation and failed. The dispatch now replaces the shell with the
   Node supervisor using `exec`; a static operational test preserves the
   property.
2. **Resolved — closure decoy could authorize cleanup.** Superseded closeout
   initially accepted a closing keyword embedded in arbitrary prose. The rule
   is now anchored to an explicit line-level closing directive and a negative
   decoy case is covered.
3. **No unresolved architecture blocker.** Delivery behavior is isolated in
   one factory module, depends on existing contribution-policy validation, and
   exposes pure validation seams for tests. Worktree cleanup remains in the
   existing lifecycle owner. No product crate, public Rust API, dependency, or
   wire format changes.
4. **No unresolved security blocker.** Mutation requires explicit execution,
   exact hosted identity, successful required checks, clean mergeability, a
   real multiline DCO body, and verified post-merge commit evidence. Files are
   bounded, UTF-8, regular, and non-symlink. Receipts use private immutable
   local storage. Cleanup refuses dirty, current, primary, locked, stale, or
   unregistered worktrees and preserves recovery refs.
5. **No unresolved process blocker.** The issue, planning-only commit,
   immutable preflight receipt, Pi canary, local review, target plan, and
   verification evidence exist before archive and PR creation.

## Decomposition and granularity

The target plan measured 20 changed paths and 1,619 inserted lines, so a
decomposition explanation is required. The slice remains independently
reversible and has one operational outcome: close the delivery lifecycle after
implementation. It is separated internally into planning, PR preflight,
guarded merge, superseded cleanup, and evidence commits. Splitting those into
independent PRs would leave partial mutation paths without their shared safety
contract. The 506-line delivery module is cohesive around one CLI boundary;
its hosted adapter and pure validators are already separated by function seams
and can be extracted later if a second delivery backend creates measured reuse.

## Residual limitations

- The guarded hosted merge path cannot be exercised against this PR until the
  PR exists and required checks pass. Deterministic injected-adapter tests cover
  its state machine first; this PR will then be the live canary.
- Full local Nix evidence was captured at `d5b481632fa3dc274ca17f76dbb41bff1378212a`;
  the final review-only
  closure-regex correction at `5d32d87338dc1c969f079fba21f81b54c352b850`
  was covered by the exact-head 47-test
  operational suite and will receive hosted exact-head fast CI.
- The Darwin Nix fixup phase emitted intermittent `audit-tmpdir.sh` child
  segmentation diagnostics, but Nix completed all checks successfully. This
  slice does not claim to repair the host/Nix diagnostic.

## Verdict

Approved locally with no unresolved blocking finding. Hosted exact-head fast CI
and review remain mandatory before the guarded merge command may execute.
