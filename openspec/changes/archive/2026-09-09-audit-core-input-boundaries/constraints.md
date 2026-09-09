# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/246
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` requires explicit evidence for reviewed untrusted-input
boundaries. `SDK-LIM-007` remains effective because only `identus-core` is in
scope and allocation before URL validation remains caller-owned.

## Introduced or changed constraints

No constraint ID or effective behavior changes. The canonical specification
will require an auditable inventory tying each public core input surface to its
existing byte/range/work bound and deterministic evidence.

## Introduced or changed limitations

`SDK-LIM-007` will identify `identus-core` as crate-audited without claiming
repository-wide completion. The URL outer-allocation limitation remains, and
all unaudited parser/value surfaces remain covered by the existing limitation.

## Consumer and product impact

Consumers receive clearer evidence only. Valid/invalid inputs, wire bytes,
error behavior, features and targets do not change. No Oxid, Midnight Identity,
NeoPRISM, Lace or other downstream repository is mutated or claimed migrated.

## Activation and rollback

Issue #246 directs this routine evidence slice. Activation requires the focused
tests, exact-diff review, normal factory readiness and green hosted CI before
merge to `develop`. Rollback removes the new evidence and restores the prior
ledger wording without weakening `SDK-LIM-007`.

## Evidence

Acceptance requires a complete public-surface inventory, existing URL
exact/over-limit proof, numeric serde rejection for negative/fractional/
overflow JSON, proof that monotonic time has no serde surface, no manifest or
lockfile delta, a pinned Pi invocation receipt and bounded factory metrics.
