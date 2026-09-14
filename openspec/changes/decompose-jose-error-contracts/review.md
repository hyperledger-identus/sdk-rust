# Pre-implementation review

- **Issues:** #271 umbrella; #280 JOSE delivery
- **Exact base:** `c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1`
- **Scope:** planning artifacts and immutable golden only
- **Result:** PASS
- **Blocking findings:** none

## Semantic and architecture review

The private code/kind/message record is the irreducible JOSE shape: five kinds
vary, while capability and local/public messages do not. The four 15/11/16/9
groups match observed responsibilities and cap review units at 16 records. The
plan keeps all 51 behavioral decisions while removing the second mapping site
and wildcard default. ADRs 0116/0117 are evidence, not cross-crate authority.

## Golden and compatibility review

An independent source parser matched all 51 rows exactly and in enum order:
51 enum variants, 51 public constants, and 51 code/message arms; no field,
row, uniqueness, or order mismatch. Kinds are 29 `InvalidInput`, 12
`VerificationFailed`, five `Internal`, three `Unsupported`, and two `Crypto`.
All local/public messages match and all sources are empty.

The golden is 55 LF lines, 13,353 bytes, SHA-256
`528b29913876710a2ee806e30fef044657f3c6c38e7e3efff860cf71060b9592`.
The plan preserves every public declaration, const bridge, `From`, derives,
display and source behavior, and adds no dependency/feature/wire/binding/API.

## Factory, security and target review

The third binding extends the existing data-driven validator and immutable
Git-blob path rather than copying security logic. Active/archive ambiguity,
mutation, path escape, incomplete state, and symlinks remain fail closed.
Static redacted records cannot retain input or secret data. Direct portable
target compilation remains evidence only.

Factory research/constraints/OpenSpec validation passed 68/68. Planning is
approved for signed+DCO commit and durable preimplementation receipt before
production edits.
