# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/162
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep product/backend dependencies and foreign
types out of generic crates and public APIs. `SDK-COMPAT-001` through
`SDK-COMPAT-005` retain exact Rust 1.98.1 and require honest target evidence.
`SDK-SEC-001` through `SDK-SEC-003` require no first-party unsafe, redacted
secrets/errors and bounded untrusted inputs. `SDK-DELIVERY-001` requires the
issue, specification, review and hosted gates. `SDK-LIM-001`, `SDK-LIM-003`,
`SDK-LIM-005`, `SDK-LIM-006`, `SDK-LIM-007` and `SDK-LIM-009` preserve
unpublished, consumer-owned, target, resource and fast/slow limitations.

## Introduced or changed constraints

- The executable candidate is exact published `aries-askar 0.4.6`, default
  features disabled, with only `sqlite`, in a separately locked fixture.
- Candidate types, errors, database URLs and key methods remain private to the
  fixture; root crates and locks do not change.
- The proof uses an ephemeral in-memory store and fixed research-only pass key.
- Conditional mutation must validate SDK-owned revisions inside an Askar
  transaction and fail closed on missing, malformed or conflicting state.
- No raw caller value, pass key or candidate diagnostic may cross Debug, error,
  log or FFI boundaries.

## Introduced or changed limitations

- One exact-record proof does not establish list/pagination, migration,
  multi-process, crash-recovery, performance, mobile runtime or product custody.
- SQLite/native compile evidence does not establish target runtime support.
- Encryption-at-rest behavior does not establish OS-backed key protection,
  backup, recovery, certification or secure deletion.
- Askar 0.5.0 source is reference-only until an immutable crate is published.

## Consumer and product impact

No current consumer or product changes. A future optional adapter may be
proposed only after this evidence and a separate production issue. Oxid,
Midnight and donor repositories remain unchanged.

## Activation and rollback

The research capability activates only when issue #162's evidence PR passes
local and hosted gates and merges into `develop`. Rollback removes research
assets only. There is no public/wire, data, release or downstream migration.

## Evidence

Issue #162 and parent #151 provide authority. Final evidence must record exact
source and crate provenance, license, MSRV, features, direct/resolved cone,
unsafe/native reach, advisories, targets, transaction/revision behavior,
redaction, maintenance, limitations, reconsideration triggers and commands.
