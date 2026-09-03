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
