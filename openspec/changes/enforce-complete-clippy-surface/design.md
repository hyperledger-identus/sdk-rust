# Design: complete Clippy evidence without widening fast CI

## Boundary

This change closes a static-analysis coverage gap. It does not alter runtime
behavior, public APIs, protocol semantics, resource limits, dependency
selection, target claims, or releases.

## Decisions

1. Keep `rust-clippy` and every existing explicit fast selector unchanged.
2. Add `rust-clippy-all-targets-all-features` to the declarative Nix gate
   manifest with primary Rust 1.98.1, primary artifacts, workspace selection,
   `all_targets = true`, `all_features = true`, and `-D warnings`.
3. Reference the new gate from each host's complete evidence set and invoke its
   exact Nix selector explicitly in the weekly/manual matrix. The subsequent
   `nix flake check` reuses the built result.
4. Extend the offline support-policy validator with a semantic gate-shape check
   and one selector per host. Do not accept comments or free-form Cargo text as
   execution evidence.
5. Apply Clippy's behavior-preserving fixes to the four reproduced findings.
6. Remove the test helper's lint exception through a small private value object.
7. Retain the two public limit constructors unchanged but replace their
   `allow` annotations with reasoned `expect` annotations and list them in the
   lint-exception registry. A future migration removes them only under a
   focused public-API decision.

## Failure behavior

Any warning reachable only from a non-default target or feature fails the new
Nix check. Drift in its operation, compiler/artifact provider, target/feature
flags, warning denial, host references, or weekly/manual workflow selector
fails the offline support-policy contract before hosted Rust work starts.

## Alternatives

A single wider required fast check is simpler but conflicts with the accepted
throughput policy. A handwritten workflow Cargo command would bypass the
declarative Nix gate registry and duplicate toolchain setup. A broad lint
allowance would hide future unrelated warnings.
