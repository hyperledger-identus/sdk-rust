# oid4vci-error-contracts Specification

## MODIFIED Requirements

### Requirement: Exact pre-refactor behavior is immutable

The compatibility oracle SHALL remain the planning golden captured from
`develop@6217384f85ff72003a7b94482bf7879f4587be02` with exactly 171 unique
ordered rows and SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
It SHALL pin baseline constant identity/visibility, code, kind, capability,
local/public/full display, discriminant order, and source. Stable and planning
copies SHALL remain byte-identical, fixed-hash, and receipt-blob bound.

The 171 baseline variants SHALL be the exact ordered prefix of the complete
live exhaustive inventory. Every live inventory entry SHALL be unique. A new
variant SHALL append after the existing inventory and SHALL NOT rewrite the v1
fixture. Ambiguity, baseline insertion/reorder/removal, schema/type drift,
coordinated fixture mutation, path escape, or symlinked components SHALL fail
closed.

#### Scenario: every baseline error remains exact after additive evolution

- **WHEN** the live catalogue is compared with the v1 golden
- **THEN** the first 171 entries remain ordered, public, exact-message,
  source-free, and distributed as 167 InvalidInput plus four Unsupported
- **AND** any live suffix contains only unique append-only variants

The authorization-code server-binding capability SHALL append static public
rows for an absent Authorization Code grant, an Authorization Code
server-hint mismatch, unsupported Authorization Code grant capability, and a
missing Authorization Endpoint. Existing ordered rows and every existing
row's variant, constant, code, category, message, Display text and help URL
SHALL remain exact.

#### Scenario: new server-binding diagnostics are append-only

- **WHEN** the authorization-code server-binding capability is delivered
- **THEN** its four diagnostics follow the previous canonical inventory rows
- **AND** the historical prefix remains byte-for-byte semantically unchanged
