# ADR 0200: raise the SDK MSRV to Rust 1.99.1

- **Status:** Accepted by project-sponsor direction for the next `0.2.x` line
- **Date:** 2026-11-17
- **Issue:** [#596](https://github.com/hyperledger-identus/sdk-rust/issues/596)
- **Decision:** [sponsor direction](https://github.com/hyperledger-identus/sdk-rust/issues/596#issuecomment-6952649191)
- **Supersedes prospectively:** ADR 0133 compiler and `0.1.x` candidate-line
  decisions; historical evidence remains immutable
- **Review no later than:** 2027-05-17

## Context

ADR 0133 selected Rust 1.89.0 as the fixed `0.1.x` MSRV and Rust 1.98.1 as
primary and etalon. It explicitly requires a later pre-1.0 minor line and a
focused compatibility decision before raising that floor. The product sponsor
now directs the SDK to use the Rust 1.99 line as its MSRV.

The latest natural weekly slow run at protected `develop`
`f7e777b8556ba4c82bdc272faf2551e7bfb9d8f8` also failed because the isolated
UniFFI generator lock contains `rustix 1.1.4`, affected by
RUSTSEC-2026-0486. The advisory is independent of compiler identity but belongs
in the same base maintenance repair before further binding work advances.

NeoPRISM `main@755fba1b1625acc795340d8514a7af1f9490d1e4` uses nightly
`2026-09-02` (`rustc 1.100.0-nightly`) and therefore clears the new compiler
floor. Apollo is not a Rust SDK consumer. No released SDK artifact or stable API
commitment exists.

## Decision

1. Rust 1.99.1 is the exact workspace MSRV, primary development compiler,
   candidate-preparation compiler, and compatibility etalon.
2. The next unpublished candidate line is `0.2.0-rc.1` / `0.2.x`. Historical
   `0.1.x` ADRs, notes, and receipts remain truthful immutable history.
3. Primary, etalon, and MSRV gate identities remain separately visible but use
   the same exact stable compiler. Nightly remains a sanitizer-only exception.
4. Every current supported feature profile, host, and portable compile target
   remains required. Raising the floor does not permit deleting a red gate.
5. Pin exact stable-overlay revision
   `ec07af44be2e0ce4e1a17e00a97d44232607c1ec`, which contains Rust 1.99.1,
   while retaining the existing nixpkgs and sanitizer-overlay provenance.
6. Update the isolated UniFFI generator lock to `rustix >=1.1.6`. No advisory
   ignore or waiver is accepted.
7. Do not publish, release, promote `main`, activate stable API, or mutate a
   downstream repository through this decision.

## Consequences

- Rust 1.89 through 1.98 source consumers must upgrade before adopting the
  `0.2.x` line.
- The ordinary compiler matrix becomes simpler because the MSRV, primary, and
  etalon share one exact version while keeping distinct evidence semantics.
- NeoPRISM is compiler-compatible, but its SDK revision refresh remains a
  separate downstream-owned change.
- Historical 0.1 candidate evidence is not relabeled as Rust 1.99 evidence.
- The stable overlay and lockfile become explicit maintenance inputs.

## Migration

Source consumers install Rust 1.99.1 or newer before selecting the `0.2.x`
line. Consumers pinned to an immutable 0.1 candidate may keep that source and
its original compiler contract; no compatibility support is promised for
mixing 0.2 sources with an older compiler. No public API or serialized data
migration is introduced by this decision.

## Verification and rollback

Cargo, Nix providers, machine support policy, constraint index, candidate
descriptors/checkers, current documentation, mutation fixtures, full feature
profiles, host and portable targets, and the UniFFI supply-chain audit must
agree. NeoPRISM compiler identity is inspected read-only at its immutable
revision.

Rollback atomically restores the prior compiler/policy files, reverts to the
0.1/Rust 1.89 compatibility contract, and leaves release preparation blocked
until the `rustix` advisory is fixed separately. No published artifact or
downstream cleanup is required.
