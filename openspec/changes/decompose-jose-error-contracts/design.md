# Design

## Context and goals

JOSE behavior is correct, but 51 contracts are split between code/message and
kind mappings in one 214-line bridge. The goal is one explicit contract per
variant, no wildcard default, smaller responsibility-owned review units, and
exact characterization.

## Decisions

### JOSE-specific private record

Add a crate-private const record containing `ErrorCode`, `ErrorKind`, and one
static message. The const conversion supplies centralized `CAPABILITY`. It is
neither public, serialized, bound, allocated, nor returned.

### Four ownership modules and exhaustive router

Add compact/header, algorithm/key/registry/signing, proof/key/evidence, and
proof/policy/time/replay catalogues with 15/11/16/9 records. Keep public enum
and constants explicit. One private macro list maps all variants to records and
produces only a cfg(test) inventory; its match has no wildcard or literals.

### Independent immutable golden

Capture 51 rows from the exact base before implementation. Copy the bytes to a
stable JOSE test fixture only after the receipt. Extend the data-driven checker,
mutation suite, factory fixture, and Nix source filter with a third binding;
do not duplicate path-security logic. The planning artifact SHA-256 is
`528b29913876710a2ee806e30fef044657f3c6c38e7e3efff860cf71060b9592`.

The integration test independently enumerates all 51 variant/constant pairs
and checks ordered exact rows, discriminant order, constants, kinds, capability,
both display surfaces, `From`, const conversion, `Debug`, and both source
results. The internal generated inventory independently checks fixture keys.

### Compatibility and measurement

Require an empty base/head public API and manifest/feature/dependency/lock diff.
Success keeps 51 behavioral rows, reduces mapping sites from two to one,
reduces wildcard defaults from one to zero, caps catalogues at 16 rows, and
keeps the router free of code/kind/message literals. Report largest unit and
total relevant physical/nonblank lines without imposing a line-count gate.

## Risks and mitigations

- Coordinated fixture drift: fixed hash/provenance plus Git receipt-blob binding.
- Missing future variant: exhaustive router and independent 51-row inventory.
- Silent kind assignment: no wildcard and explicit kind in every record.
- Fragmentation: only four observed ownership axes; no shared framework.
- Overstated targets: direct compile evidence is explicitly limited.

## Migration and rollback

Commit ADR/OpenSpec/golden first, validate and commit the durable receipt, then
copy the fixture and implement source/tooling separately. Run compatibility,
quality, targets, factory/Nix, and two independent reviews before archive and
PR. Rollback restores the matches; no external state changes.
