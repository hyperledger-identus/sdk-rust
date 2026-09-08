# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The canonical support policy already defines Rust 1.98.1 compile-only gates for
`wasm32-unknown-unknown`, `aarch64-apple-ios` and
`aarch64-linux-android`. The Apollo parity manifest records those targets, but
its iOS and Android gate labels use stale `*-arm64` spellings and all portable
rows point to an older run. The current checker validates only target inventory,
SHA syntax and a broad tier enum, so a host-only gate or unrelated Actions run
could satisfy it.

## Normative sources

- Issue #213 defines the M2 closing receipt and requires exact packages,
  features, toolchain, target gates, CI links and honest compile-only limits.
- `docs/architecture/sdk-support-policy.toml` is the machine-readable source
  for target names, gates, packages, features, tier and limitations.
- `nix/checks/gates.toml` defines the actual Cargo build operations generated
  into the Nix check graph.
- ADR 0081 places the complete matrix in weekly/manual slow CI while preserving
  one Linux fast merge gate.
- `SDK-LIM-003` explicitly limits the three portable targets to compile checks;
  `SDK-LIM-009` keeps this evidence outside the per-PR fast path.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One exact manual slow run plus target-specific gate binding | `adopt` | Reuses the existing authoritative check graph, gives one closing SHA and does not add CI friction. | The slow workflow stops executing all three named target gates. |
| Reuse the Linux fast receipt for every target | `reject` | A host workspace build is not evidence that the three cross targets compile. | Never while target-specific gates exist. |
| Add three per-PR platform jobs | `not-adopt-now` | Contradicts the temporary fast/slow policy and adds no new behavior evidence. | ADR 0081 is superseded for release preparation. |
| Add runtime/link/package smoke tests | `not-adopt-now` | That would materially upgrade support beyond the existing compile-checked tier. | A named consumer and focused platform/FFI contract authorize the upgrade. |

## Rejected or deferred candidates

Host-only evidence is rejected because it cannot establish cross-target
compilation. Per-PR target jobs and runtime/link/package harnesses are deferred
until ADR 0081 is superseded or a named consumer authorizes a higher support
tier. They are not necessary to truthfully close the existing compile-checked
M2 evidence.

## Compatibility and dependency evidence

No crate, feature, dependency, public API or target promise changes. The
validator reads the existing support-policy TOML and requires the parity
receipt to match it, avoiding a second manually maintained target contract.
The report remains deterministic and the new fields are repository evidence,
not a library interface.

## Security, privacy and maintenance evidence

The receipt contains only public revision, workflow, toolchain and build-shape
metadata. It contains no keys, credentials or user data. Failing closed on gate
and target mismatch reduces the risk of overstating portability. GitHub log
retention remains an evidence durability limitation; the versioned gate shape
and run identity remain durable even when hosted logs expire.

## Open questions and blockers

None. A failed or cancelled closing run is not acceptable evidence and remains
a delivery repair condition rather than a product decision.

## Evidence commands

- `./scripts/check-apollo-parity.py`
- `python3 scripts/tests/apollo-parity.py`
- `./scripts/check-support-policy.py`
- GitHub Actions slow workflow run 34286965807 at exact develop revision
  `244ded689a29a5e6606eeb36aede14149d585071`.
