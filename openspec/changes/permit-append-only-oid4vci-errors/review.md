# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@21cfc28321f76f1d14a2483d536d302017674a18
Implementation head: c15ab7aece643f5250ccea2f4aacbe8bd5fe8a14
Reviewed head: c15ab7aece643f5250ccea2f4aacbe8bd5fe8a14
Specification commit: f0e7e80354a18b4b6d825ebbf305950f93ffe41b
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #346, ADRs 0119 and
0137, canonical delta, immutable golden binding, receipt, crate-local inventory
logic, synthetic mutation tests and all verification output.

## Findings

1. **Historical compatibility — accepted.** The fixture remains byte-identical
   at SHA-256 `2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
   All 171 variants remain the exact ordered prefix; no production declaration
   or discriminant changes.
2. **Live exhaustiveness — accepted.** The existing wildcard-free match remains
   the only router. A missing mapping still fails compilation, while uniqueness
   is checked over the complete live inventory.
3. **Mutation behavior — accepted.** Focused tests prove a unique suffix is
   accepted and baseline reordering or duplicate suffix entries fail.
4. **Feature ownership — accepted.** The old fixture is not claimed to cover
   future suffix entries. ADR 0137 requires the owning feature to test each new
   constant, code, kind, message, conversion and redaction property.
5. **Runtime and supply chain — accepted.** No production path, public item,
   manifest, dependency, feature, lockfile, unsafe/native, wire, target,
   downstream or release behavior changes.

## Residual limitations

- The v1 golden remains historical evidence only; it does not characterize a
  future suffix.
- Each future feature must provide independent suffix coverage and remain
  append-only.

## Review decision

The change removes an accidental development ceiling without weakening the
immutable baseline or exhaustive router. No unresolved correctness, security,
privacy, compatibility, architecture, dependency or delivery finding remains.
