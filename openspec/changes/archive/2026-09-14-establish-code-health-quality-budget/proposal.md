# Change: establish code-health quality budget

## Why

sdk-rust now has enough crypto, DID, presentation, wallet-conformance, and
OID4VCI behavior that concentration and repetition can obscure ownership for
both developers and agents. Issue #270 asks for a reproducible baseline and a
review contract before feature growth resumes. Raw line-count limits would be
easy to game and would confuse test evidence with shipped code.

## What Changes

- Add ADR 0115 defining evidence-led granularity, duplication, and touched-
  scope ratchet rules.
- Pin `rust-code-analysis-cli` 0.0.25 through the locked Nix development shell
  and add a deterministic repository audit/report command.
- Separate authored production, external-test, and syntax-recognized inline
  test populations. Only Cargo test/bench target trees are intrinsic; source
  test modules require syntax-proven reachability, including correct nested
  module context. Generated inputs require an exact path-and-marker allowlist.
- Bind the baseline to a policy-pinned revision, source fingerprint and report
  digest with cheap Git-tree verification and pinned weekly regeneration.
- Check in the exact-head baseline and classify every named issue #270 hotspot
  as `decompose`, `deduplicate`, `document-exception`, or `defer-with-owner`.
- Apply one low-risk production refactor: make DID cache and registry reuse one
  crate-private standard resolution-failure constructor in `resolution.rs`.

## Capabilities

### New capabilities

- `code-health-governance`: reproducible code-health evidence, hotspot
  disposition, and non-gameable touched-scope review.

### Modified capabilities

None. This change preserves all existing runtime and protocol requirements.

## Impact

- Affected tooling: default Nix shell, factory structural checks, and a new
  repository-local audit tool and tests.
- Affected source: private implementation only in `identus-did`.
- No public API, wire, feature, dependency-graph, input-bound, or error behavior
  changes; no work from #7 or #168.
