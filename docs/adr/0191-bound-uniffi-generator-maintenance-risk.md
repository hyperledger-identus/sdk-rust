# ADR 0191: Bound the UniFFI generator maintenance risk

- Status: Accepted
- Date: 2026-10-28
- Issue: [#564](https://github.com/hyperledger-identus/sdk-rust/issues/564)
- Expires: 2027-01-31

## Context

RUSTSEC-2026-0374 classifies `basic-toml 0.1.10` as unmaintained. SDK-Rust
reaches it only through the separately locked unpublished binding generator:
`identus-uniffi-bindgen -> uniffi 0.32.0 -> uniffi_bindgen 0.32.0 -> askama
0.16.1 -> askama_macros 0.16.1 -> askama_derive 0.16.1 -> basic-toml 0.1.10`.
It does not enter runtime crates or generated packages. UniFFI 0.32.2 retains
the same Askama family, so no compatible published upgrade removes the path.

## Decision

Accept RUSTSEC-2026-0374 only for that exact build-time path through 2027-01-31.
Enforce the decision with a repository script that checks the clock and exact
locked inverse tree before running `cargo deny` for bans, licenses and sources,
then `cargo audit` as the single tool-lock advisory authority scoped to the
single accepted ID. Keep root and fuzz audits ignore-free and preserve the full
Swift, Kotlin/JVM, Apple and Android E2E lane. Remove the
acceptance as soon as upstream publishes a compatible clean graph. A newly
reported vulnerability invalidates this decision immediately.

## Consequences

The weekly lane becomes actionable again without representing an unmaintained
crate as remediated. Exact graph matching and expiry turn upstream drift into a
failure, while one advisory engine prevents contradictory duplicate policy
results and tests make the exception visible and reproducible. If upstream
has not removed the dependency by the deadline, a new issue and ADR decision
must choose migration, a reviewed fork or a short explicit renewal; renewal is
never automatic.

## Rejected alternatives

- Removing `cargo audit` or the host E2E lane hides supply-chain drift.
- An unbounded ignore has no reliable exit and can mask dependency changes.
- UniFFI 0.32.2 does not remove the dependency.
- Forking Askama or UniFFI immediately creates a larger maintenance burden than
  the current build-only maintenance finding.
