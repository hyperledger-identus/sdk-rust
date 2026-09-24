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

### Requirement: Public, wire, and protocol boundaries remain unchanged

The v1 OID4VCI error surface SHALL remain exact, including its enum prefix,
derives, non-exhaustive marker, `CAPABILITY`, all baseline public
constants/paths, `From`, `Display`, `Error`, and
`pub const fn to_identus_error`. A later issue-scoped feature MAY append a
fieldless variant, public stable code and exhaustive router record only when it
preserves every baseline discriminant and contract and independently tests the
new contract.

Existing typed protocol error responses and Serde/wire shapes SHALL remain
unchanged unless the feature's own specification explicitly governs a delta.
No feature may use this evolution rule to weaken redaction, stable-code,
resource-bound or compatibility requirements.

#### Scenario: additive error leaves the baseline exact

- **WHEN** a feature appends an independently tested fieldless error contract
- **THEN** all v1 public items, discriminants, codes, kinds, messages,
  conversions and wire behavior remain exact

### Requirement: Maintainability evidence is truthful

The historical refactor SHALL retain its 171 baseline behavioral decisions,
one mapping site and zero wildcard defaults. The complete live decision count
MAY grow only through append-only independently tested feature contracts. Six
responsibility catalogues SHALL remain discoverable, no catalogue SHALL exceed
the standing 39-record review ceiling, and every total-line or record-count
movement SHALL be disclosed without claiming decision deduplication.

#### Scenario: cohesion survives additive evolution

- **WHEN** a new error contract is appended
- **THEN** its responsibility catalogue remains independently reviewable, the
  router remains one explicit wildcard-free decision site, and all baseline
  plus live decisions remain explicit
