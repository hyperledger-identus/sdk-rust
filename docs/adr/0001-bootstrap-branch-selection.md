# ADR 0001: select `yet-another-seed` as the `develop` baseline

- **Status:** Accepted for the `develop` bootstrap
- **Date:** 2026-09-02
- **Decision authority:** explicit project-sponsor direction; repository
  governance remains subject to Identus maintainer ratification
- **Supersedes:** no prior repository ADR
- **Related work:** sdk-rust issues #3 through #10 and the cross-repository
  crystallization epic

## Context

The repository has a protected, nearly empty `main` branch and three candidate
implementation branches. The program needs an active line from which agents
can stabilize and extract generic SSI components without disrupting Oxid or
chain-specific repositories.

The inspected revisions were:

| Candidate | Revision | Useful evidence | Material concern |
| --- | --- | --- | --- |
| `main` | `2c267d6` | Protected common ancestor and clean history | No Rust implementation; intentionally reserved for now |
| `yet-another-seed` | `662f8d7` | Strongest executable chain-neutral Rust foundation; validated newtypes; crypto and derivation vectors; DID, entropy and conformance crates; Nix/Crane checks | Empty future crates, incomplete governance, `0.0.0` versions, MSRV/toolchain inconsistency, compiler-sensitive UI snapshot and not-all-signed legacy commits |
| `sdk-rust-seed` | `0515540` | Agentic SDLC, conformance catalog, cross-SDK parity concepts and signed/DCO history | Broad speculative surface and sibling-checkout assumptions |
| `jubjub-poc` | `326599d` | WASM, UniFFI, Android and packaging experiments | Chain-coupled Midnight dependency and unsafe secret/FFI boundary for a generic SDK |

At selection time `yet-another-seed` passed Rust formatting and clippy. Host
Rust 1.95 exposed a compiler-wording-only `trybuild` snapshot mismatch. Its
pinned `nix flake check` failed before project checks while building a pinned
Statix dependency whose own snapshot test failed. These failures are accepted
as visible stabilization debt, not waived release gates.

## Decision

1. Create the active `develop` branch directly from
   `yet-another-seed@662f8d7d2b9b9a151365c6bb889cd614bca625f7`. A direct branch
   preserves the selected history without an artificial merge commit.
2. Keep `main` at `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`, intentionally
   minimal and outside the integration/release flow until a later ADR activates
   it. No bootstrap implementation is merged to `main` now.
3. Add the governance and architecture packet as the first signed, DCO-bearing
   commit on `develop`. All subsequent repository-facing commits follow those
   controls. The accepted seed's legacy signature state remains disclosed.
4. Treat the implemented `core`, `derive`, `crypto`, `did`,
   `adapters-entropy` and `conformance` crates as foundations to audit and
   stabilize. Their public names and APIs are still pre-release decisions.
5. Treat the empty `agent`, `bindings`, `credentials`, `messaging`,
   `openid4vc`, `presentations`, `trust` and `wallet` crates as placeholders,
   not roadmap commitments or publishable components. Remove, quarantine or
   fill each only through an accepted component decision.
6. Use `sdk-rust-seed` as a process/evidence donor and `jubjub-poc` only as a
   delivery-pattern reference. Later ports remain component-by-component and
   record source SHA, path, license, transformation and differential evidence.
7. Keep Oxid, midnight-identity, neoprism, Portal and other consumers unchanged
   during upstream SDK work. Adoption is a separate downstream change against
   an immutable SDK revision or release.

## Why this branch

`yet-another-seed` provides the greatest amount of validated, chain-neutral
Rust that already matches the intended SDK boundary. Starting `develop` from
it preserves real tests and design learning while avoiding a costly replay of
the same foundation. The other candidates are valuable references but make
weaker starting points: one is broader and more speculative, while the other
is deliberately Midnight-specific.

The decision accepts code history, not every architectural choice embedded in
that history. Current governance, accepted ADRs and component contracts take
precedence over archived seed proposals.

## Consequences

- `develop` begins with meaningful code and known debt rather than an empty
  workspace.
- Baseline code may be renamed, split, removed or hardened before publication.
- Archived OpenSpec references to merging into `main` are historical and no
  longer operational instructions.
- CI and pull-request rules must target `develop` while `main` remains reserved.
- Existing seed commits are not rewritten merely to retrofit signatures; new
  commits, releases and promotion decisions must meet current policy.

## Stabilization gate

Before the baseline can produce a crate release candidate:

- decide the approved crate namespace and ownership under issue #3;
- classify and remove or explicitly accept every placeholder crate;
- align the pinned NeoPRISM-etalon Rust toolchain, declared MSRV and Nix inputs;
- replace or pin compiler-sensitive UI diagnostics so MSRV and the etalon
  toolchain gates are deterministic;
- make `nix flake check` reproducible on Linux and macOS;
- pass plain-Cargo build, test, fmt, clippy and docs gates;
- keep architecture guards rejecting chain/product dependencies;
- confirm Apache-2.0 source and fixture provenance;
- enable protected `develop` review, DCO, signature and required-check rules;
- preserve this selection and known-debt receipt in the first release notes.
