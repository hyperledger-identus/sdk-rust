# ADR 0103: spike Aries Askar as an isolated storage adapter

- **Status:** Accepted for research only
- **Date:** 2026-09-09
- **Decision owners:** issue #162 and parent #151
- **Constraints:** `SDK-ARCH-001`, `SDK-ARCH-002`, `SDK-SEC-001` through
  `SDK-SEC-003`, `SDK-COMPAT-001` through `SDK-COMPAT-005`,
  `SDK-DELIVERY-001`, `SDK-LIM-001`, `SDK-LIM-003`, `SDK-LIM-005` through
  `SDK-LIM-007`, `SDK-LIM-009`

## Context

Aries Askar is an actively maintained encrypted SSI record/KMS store. It could
avoid reimplementing encryption-at-rest and database mechanics, but its
published top-level package combines storage with a broad cryptographic package
and exposes backend/session semantics that must not become the generic SDK
contract. Its entries also have no public revision token, while the Identus
storage ports require compare-and-swap revisions.

Published 0.4.6 is the only immutable crates.io candidate. Upstream tags 0.5.0,
but no corresponding crate is available. A source branch is not accepted as a
production or research dependency merely to obtain newer code.

## Decision

1. Spike exact `aries-askar 0.4.6` in a nested research workspace with default
   features disabled and only `sqlite` enabled.
2. Exercise one SDK-owned `SecretStore` exact-record adapter against encrypted
   in-memory SQLite and the existing conformance suite.
3. Own scope/key encoding, value envelope, revision semantics and error lowering
   in the adapter. Validate conditional writes/deletes inside transactions.
4. Keep Askar, SQLite, its runtime and all candidate types out of generic crates,
   root locks, fast CI and public APIs.
5. Treat upstream 0.5.0 as maintenance/reference evidence only until published.
6. Decide production fit only after executable, dependency, security and target
   evidence. Any production adapter requires a new issue and ADR.

## Consequences

The spike can demonstrate whether Askar is technically adaptable without
granting it architectural authority. It also measures the real cost of the
unconditionally coupled crypto package and native SQLite backend. A passing
fixture is necessary but insufficient for production adoption; list ports,
migration, persistent databases, crash recovery, performance, key custody and
platform runtime remain separate decisions.

The research is independently reversible and does not alter consumers or
release artifacts.
