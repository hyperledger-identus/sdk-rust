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
Preserve every SDK-owned method and observable cryptographic invariant. Four
published tuple fields already expose the upstream `k256`/`p256` key types;
retaining their 0.13 identities would retain the rejected closure. Classify
their move to the corresponding 0.14 types as a source break on the candidate
line, record it as `crypto.secp256k1.rustcrypto-security-stack.v1`, and publish the
versioned migration before the next candidate. Require clean root and fuzz
audits with no ignores plus existing vector and target evidence before
integration.

## Consequences

Implementation calls need mechanical migration to upstream 0.14 traits. The
repository receives an advisory-clean closure and makes the accidental public
RustCrypto boundary explicit rather than pretending it is private. SDK byte
constructors and encoders remain the recommended boundary; later encapsulation
requires its own compatibility decision. Future family upgrades remain
coordinated and evidence-driven rather than lockfile-only.

## Rejected alternatives

- Ignoring either advisory weakens the security gate and is rejected.
- Forcing `crypto-bigint 0.7` under `elliptic-curve 0.13` is not source-compatible.
- Replacing RustCrypto with a new native or unsafe backend adds unnecessary
  supply-chain and portability risk.
- Keeping both curve families would leave the affected closure reachable.
- Describing the work as behavior-neutral contradicts the published API
  snapshot and would hide a real consumer migration.
