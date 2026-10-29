# Exact-diff factory review

- **Review date:** 2026-10-29
- **Issue:** #291
- **Develop base:** `c429ac494a6eaeb2e22c0b38a5b1d70c0eb6ff62`
- **Reviewed implementation:** `33331cd6db77f87598a2461fe6a296faccdb2ac8`
- **Reviewer:** Codex CLI, fresh base-to-head review
- **Result:** no unresolved blocker, high, or medium finding

## Resolved findings

The review loop found and corrected six policy gaps before the final pass:

1. closing keywords hidden in prose were initially under-detected;
2. repository-qualified references and the complete GitHub keyword family
   needed exact handling;
3. established Dependabot/Renovate exemptions and historical receipt recovery
   needed preservation;
4. deferred sidecars needed to be read from the immutable pull-request head;
5. terminal metrics needed to recognize deferred linkage without accepting an
   unrelated repository qualifier; and
6. terminal metrics needed to validate the exact-head sidecar rather than
   trusting mutable pull-request text alone.

Each finding received a focused regression. The final Codex CLI pass reported
no actionable defect and independently reran the 66-test factory operations
suite successfully.

## Architecture and security result

- Closing remains the ordinary delivery path; deferred linkage is explicit,
  narrow, and cannot represent completion.
- Protected-base code treats the pull-request head only as bounded untrusted
  Git data. It does not check out or execute head-owned code.
- Deferred evidence is duplicate-key-safe, size bounded, closed-schema, bound
  to the exact issue and immutable head, and revalidated at metrics publication.
- Guarded delivery rejects later closing text across title, body, commits,
  merge body, and observed merge commit while preserving historical recovery.
- The change affects repository policy and documentation only. SDK APIs,
  dependencies, targets, wire formats, releases, and GitHub settings are
  unchanged.

Final disposition: no unresolved architecture, security, compatibility, or
factory-operability finding.
