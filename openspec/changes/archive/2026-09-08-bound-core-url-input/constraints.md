# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/193
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` already requires an explicit resource limit for this materially
changed untrusted-input boundary. `SDK-COMPAT-001` requires the newly rejected
input class and public error variant to be recorded rather than hidden as an
implementation detail. `SDK-LIM-007` remains effective because issue #193 is
one child slice, not the repository-wide inherited-parser audit.

## Introduced or changed constraints

No new machine constraint ID is required. The effective behavior becomes:
`identus-core::Url` accepts at most 8,192 UTF-8 bytes on every validated
construction path. Changing that constant later requires a new compatibility
and resource-budget decision with named consumer evidence.

## Introduced or changed limitations

The `SDK-LIM-007` summary and scope remain unchanged. Its consumer-impact text
will stop naming `Url` as intrinsically unbounded after tests prove the limit.
The entry will instead state that outer limits are still required for source
allocations and for inherited surfaces without explicit evidence.

The new intrinsic bound does not cap allocation performed before validation by
a caller, transport, decompressor or generic serde deserializer. It does not
prove complete RFC 3986 parsing and does not create a network safety policy.

## Consumer and product impact

Callers with a syntactically accepted URL above 8,192 bytes must reject it,
store it outside `Url`, or propose a separately budgeted value type. Existing
values at or below the boundary are unchanged. No Oxid, Midnight Identity,
NeoPRISM or Lace repository is mutated or claimed migrated.

## Activation and rollback

Issue #193 is the decision authority during unpublished active development.
Activation requires focused boundary tests, full Nix validation, distinct
local review and green hosted CI before merge to `develop`. Rollback restores
the previous validator and the named `Url` clause in `SDK-LIM-007`; relaxing or
raising the limit is not an implicit rollback and needs a new decision.

## Evidence

Acceptance requires exact 8,192-byte success, exact 8,193-byte failure,
multibyte byte-count proof, all generated constructor paths, serde rejection,
stable redaction-safe error mapping, no dependency change, canonical spec and
constraint-index atomicity, full Nix and hosted CI.
