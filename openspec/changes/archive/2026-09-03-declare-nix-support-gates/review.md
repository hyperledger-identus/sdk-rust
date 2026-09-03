# Pre-implementation semantic and architecture review

- **Date:** 2026-09-04
- **Issue:** #24, follow-up to #23 / child of #20
- **Develop base:** `a05cb05575c7d99fc025f5be450da32c7a11a5c1`
- **Result:** contract is implementable with no unresolved blocker

## Findings

The transition preserves the compatibility policy and changes only its
execution representation. Keeping compatibility claims in
`sdk-support-policy.toml` and executable definitions in `gates.toml` avoids
mixing support statements with Nix implementation details while eliminating
the duplicated operation map. Every policy reference remains independently
cross-checked.

Generating the checks from typed fields removes the need for a partial Nix or
shell parser. The schema must reject unknown keys and contradictory selections;
otherwise misspelled fields could silently fall back to defaults. All existing
gates must be enumerated before the hand-written modules are removed, and a
full Nix evaluation/build must prove operation-specific Crane attributes.

The only retained Nix-source invariant is architectural reachability: the flake
must import the check root and the root must consume the manifest to publish
generated checks. It cannot be used to infer Cargo packages, features, target,
operation or toolchain. A removed manifest entry fails regardless of identical
text in comments, strings, interpolation or `_module.args`.

Timing is intrinsically runner-dependent. Process-cold means a fresh Python
process with filesystem caches uncontrolled; warm means repeated validation in
one loaded interpreter. Twenty samples and p50/p95 provide comparable
diagnostics. A broad pathology ceiling may catch accidental subprocess/Nix
evaluation in the validator, but the values are not product commitments.

No Rust public API, wire form, secret boundary, dependency cone, supported
target or consumer changes. No donor code or fixture is copied. The work does
not require crypto/security specialist approval, repository administration,
release authority or downstream mutation.

## Implementation review

- **Reviewed final implementation head:**
  `996465abe6fa68bb7d04b218f9a0f07b5f954afe`
- **Reviewer:** distinct local contradiction-focused pass after implementation
- **Result:** one schema-hardening finding identified and resolved; no remaining
  architecture, security, compatibility, provenance or delivery blocker

The manifest contains exactly the 23 policy-referenced Rust gates. Flake
evaluation emits the same 23 check names on both supported Nix systems, and
derivation inspection confirms equivalent Crane operations and effective Cargo
commands. The only deliberate command normalization is explicit `--workspace`
on the root Clippy and rustdoc gates plus long-form `--package`; both preserve
the virtual-workspace behavior while making package selection unambiguous.

The validator no longer extracts operations, packages, features, targets or
toolchains from Nix expressions. It validates a closed TOML schema, exact
policy references/operations, existing workspace packages/features, target and
feature surfaces, operation-specific trailing arguments, toolchain/artifact
coherence and the narrow flake/generator wiring. Thirty-three initial mutation
tests proved comments, quoted/interpolated text and dead `_module.args` cannot
stand in for a manifest gate.

The review found that a future manifest edit could still combine `lib` with
`all_targets`, combine `no_default_features` with `all_features`, or rely on an
implicit root package for an operation that accepts Cargo selection. Those
states are now rejected explicitly and covered by three additional mutations.
No current gate used a contradictory state, so the correction changes no
generated derivation.

The benchmark loads the selected root's validator for warm samples, launches a
fresh Python process for process-cold samples, requires at least 20 successes,
reports stable JSON and compares PR heads with the exact base using a deliberately
broad `2x + 5 ms` pathology ceiling on p50. P95 remains reported without gating
because a single scheduler outlier controls it at 20 samples. This is tooling
protection, not a product performance promise.

The full pinned local Nix matrix passed before review with all 27 compatible
checks. Repository-source and Cargo dependency boundaries are unchanged. Read-
only consumer receipts preserve their pre-existing local state, and reserved
`main` remains clean at `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.

## Hosted timing correction review

- **Reviewed correction head:**
  `f05f618271a433d612753a490d9184d26c46e003`
- **Trigger:** final-head hosted macOS warm p95 outlier
- **Result:** correction is bounded, tested and has no unresolved blocker

The failed rerun had a 7.259 ms warm p50 but a 22.977 ms p95, while the prior
hosted run, local runs and process-cold measurement passed. With 20 samples,
one delayed sample determines p95. Gating p95 therefore contradicted the stated
diagnostic intent and produced a flaky merge gate.

The correction continues to report ratios for all four measurements and
applies the existing broad ceiling only to warm and process-cold p50. Two pure
comparison tests prove that an isolated p95 outlier stays diagnostic and a
sustained p50 pathology fails. Ruff, 38/38 Python tests, factory checks and the
complete 26-check local Nix graph passed on the corrected code.

## Pull-request review corrections

- **Reviewed correction head:**
  `91ed0246e04914bbbb1eb4901a36ee8907564965`
- **Result:** both findings resolved with focused regressions and no remaining
  blocker

The hosted review identified two additional fail-closed edges. A live Nix
string outside `imports` could satisfy the generator reachability substring,
and a syntactically valid TOML scalar in a list field could survive schema
diagnostics and raise `TypeError` during policy evidence expansion. Both were
confirmed and fixed: reachability now resolves the actual `imports` attribute,
and malformed list, boolean and target values are normalized to safe sentinels
after recording their errors. Focused regressions prove the decoy is rejected
and malformed lists fail without a traceback.

A follow-up review identified that independent “manifest parsed” and “checks
published” token checks still allowed the manifest mapping to become unused
while an empty `generatedChecks` value was published. The validator now binds
the complete chain structurally: `gates.toml` to `manifest`, `manifest.gates`
through `listToAttrs` to `generatedChecks`, and `generatedChecks` to `checks`.
An unused-mapping mutation proves the disconnected graph fails.
