# Design: truthful live-control inventory

## Decision

Use `active-with-enterprise-deviation` as the closed machine-readable state.
Keep mutable GitHub identifiers and timestamps in a human receipt rather than
making the offline checker depend on a particular ruleset ID forever.

## Evidence boundary

The offline checker proves repository-local consistency only. The dated receipt
states which live API calls were observed and must be refreshed when rules or
enterprise policy change.

## Alternatives

`active` alone would hide the scanning deviation. Retaining
`external-action-required` would hide the protection that now gates every PR.
