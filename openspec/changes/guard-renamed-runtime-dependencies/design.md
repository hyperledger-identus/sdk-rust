# Design: canonical dependency identity in the runtime-edge guard

## Decision

The shared runtime-edge collector will examine every dependency table entry as
an alias plus declaration. For table declarations, a string `package` field is
the canonical candidate; otherwise the alias is the candidate. The collector
will retain the candidate only when it is a canonical workspace package name,
then deduplicate and sort the result before layer checks.

Cargo requires `package` when an alias differs from the dependency package, so
path parsing is neither needed nor desirable here. The existing repository
boundary guard separately validates dependency paths and package identities.

## Security and correctness boundaries

- Malformed/non-table dependency values fail through Cargo or are ignored in
  the same way as the existing TOML guard vocabulary.
- An alias cannot suppress an internal edge when `package` names a workspace
  crate.
- Multiple aliases cannot fabricate multiple edges or perturb exact leaf
  assertions.
- Target-specific dev/build dependencies remain excluded from runtime edges.

## Verification

Focused tests construct TOML with canonical, renamed, duplicated and
target-specific declarations. One regression proves an added renamed
`identus-core` edge changes the wallet-conformance dependency set and would
therefore fail its wallet-only invariant.
