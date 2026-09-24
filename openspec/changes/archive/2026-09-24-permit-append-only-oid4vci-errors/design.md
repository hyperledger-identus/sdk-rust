# Design

## Frozen prefix and live inventory

Parse the immutable v1 fixture into its 171 ordered variant names. Build the
complete live variant-name inventory from the same exhaustive router list used
today. Assert that the live inventory is at least as long as the baseline,
that its first 171 names equal the fixture exactly, and that every live name is
unique.

This preserves the original discriminants and contracts without requiring the
historical fixture to describe future additions. The wildcard-free match still
causes compilation to fail for an unmapped variant.

## Future extension rule

A feature appends its public enum variant and router entry after the complete
existing inventory, adds its protocol-cohesive private record, and independently
tests its public constant, code, kind, message, conversions and redaction. It
does not rewrite the v1 fixture or checker.

## Risks and mitigations

- Baseline insertion/reorder: exact-prefix equality fails.
- Duplicate live variant: complete uniqueness assertion fails.
- Missing router entry: wildcard-free compilation fails.
- Untested new contract: feature acceptance requires independent tests; the
  historical fixture is not misrepresented as live coverage.
- Golden drift: immutable hash/provenance/blob checks remain unchanged.

## Rollback

Restore complete equality with the historical fixture and remove ADR 0137 and
canonical governance changes. No production behavior changes in either state.
