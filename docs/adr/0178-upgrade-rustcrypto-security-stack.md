# ADR 0178: Upgrade the RustCrypto security stack as one family

- Status: Accepted
- Date: 2026-10-27
- Issue: [#525](https://github.com/hyperledger-identus/sdk-rust/issues/525)

## Context

RUSTSEC-2026-0387 rejects `crypto-bigint 0.5.5`, reached through the current
RustCrypto curve family. RUSTSEC-2026-0386 also rejects Tokio 1.53.1 under the
repository's denied-warning policy. Suppression would restore green output
without correcting the affected code.

## Decision

Upgrade `k256` and `p256` together to 0.14 and consume the resulting maintained
`elliptic-curve`/`crypto-bigint` family only behind `identus-crypto`. Upgrade
Tokio to 1.54.1 or a later compatible 1.x patch without feature expansion.
Preserve every SDK-owned API and observable cryptographic invariant. Require
clean root and fuzz audits with no ignores plus existing vector and target
evidence before integration.

## Consequences

Private implementation calls may need mechanical migration to upstream 0.14
traits. The repository receives an advisory-clean closure without making
RustCrypto or Tokio types part of its public architecture. Future family
upgrades remain coordinated and evidence-driven rather than lockfile-only.

## Rejected alternatives

- Ignoring either advisory weakens the security gate and is rejected.
- Forcing `crypto-bigint 0.7` under `elliptic-curve 0.13` is not source-compatible.
- Replacing RustCrypto with a new native or unsafe backend adds unnecessary
  supply-chain and portability risk.
- Keeping both curve families would leave the affected closure reachable.
