# Design

## Program model

The migration registry is capability-centered rather than repository-centered.
One row represents one externally meaningful behavior and links its legacy
owners, Rust target, platform adapters, evidence, disposition, compatibility,
deprecation, and legacy-bug policy. Repository inventories populate the
registry; capability issues consume rows in bounded implementation slices.

The default target is:

```text
application / product policy
            |
idiomatic TS / Swift / Kotlin / React Native facade
            |
isolated WASM / UniFFI / TurboModule adapter
            |
Identus-owned Rust DTOs, errors and lifecycle states
            |
reusable sdk-rust capability + private engines / ports
```

Platform storage, key custody, networking, lifecycle, permissions, and UI stay
outside generic crates. Service orchestration remains in service programs.

## Test authority

Tests receive one of five authorities:

1. `normative`: official standards suites or published canonical vectors;
2. `identus-contract`: reviewed cross-language public/wire/error fixtures;
3. `consumer-regression`: evidence of released behavior required by consumers;
4. `implementation-regression`: valuable local behavior, not automatically a
   migration constraint;
5. `exploratory`: fuzz, property, benchmark, or experiment evidence.

Contradictions are resolved in that order, except documented Identus profile
decisions may intentionally narrow a permissive standard. Promoting a lower
tier requires review and a stable fixture owner.

## Change and deprecation lifecycle

Every ledger item is `additive`, `fix`, `behavioral`, `deprecated`, `breaking`,
or `security`. Deprecation progresses through `proposed`, `announced`,
`available-replacement`, `default-off`, and `removed`. A phase can advance only
when its exit evidence is linked. Release notes and migration guides are views
over the same ledger rather than separately remembered prose.

## Legacy-bug handling

The default is `fix-and-document`. Alternatives are `simulate-temporarily`,
`preserve-profile`, and `reject-as-unsafe`. Simulation lives at the outer
compatibility facade, not in generic core invariants, and must be bounded,
versioned, testable, observable where feasible, owned, and scheduled for
removal. Unsafe behavior is never simulated.

## Repository waves

Each SDK follows: immutable inventory, disposition review, binding/API canary,
shared conformance, capability waves, default switch, and retirement review.
Canaries use non-secret bounded DID values because native and browser
experimental evidence already exists. React Native remains an independent
qualification track under issue #223.

## Verification

The bootstrap is verified by strict OpenSpec/factory checks and documentation
review. Later migrations require source and target tests, shared vectors,
negative/boundary evidence, ABI/package/runtime gates, consumer rehearsal,
ledger completeness, generated release notes, and rollback proof.
