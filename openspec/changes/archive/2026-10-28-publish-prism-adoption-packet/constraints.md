# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/554
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep generic layers inward and concrete
effects downstream. ADRs 0174 through 0177 govern the delivered PRISM crates.
The documentation-site contract requires reproducible, truthful, self-contained
handbook content. The upstream-program contract forbids delivery claims without
immutable evidence.

## Introduced or changed constraints

- The handbook names all three PRISM crates as source-available experimental
  capabilities, not stable published services.
- The dependency direction and Cardano/SQL/HTTP/runtime stop boundary are
  visible in prose and a tracked static diagram.
- The E2E test uses only bounded public APIs and caller-owned in-memory effects.
- IDR-040 records delivered PRISM and peer profiles while leaving unimplemented
  DID methods explicit; it does not collapse mixed portfolio status to
  `delivered`.
- Downstream NeoPRISM evidence is cited by immutable revision/run/comment and
  never represented as an SDK-hosted E2E environment.

## Introduced or changed limitations

The packet does not provide a Cardano adapter, durable repository, production
runtime, publication promise, live SDK CI network test, or proof of every DID
method. In-memory execution proves portable semantics and effect seams only.

## Consumer and product impact

Consumers gain a navigable architecture and executable adoption reference.
There is no API, behavior, dependency, target, wire, persisted-data, or release
change.

## Activation and rollback

Activation requires planning readiness, exact issue preflight, focused E2E and
documentation evidence, workspace/Nix gates, independent CLI review, signed
DCO commits, green protected CI, and guarded merge. Rollback removes only the
packet and restores the prior roadmap wording.

## Evidence

Issue #554 directs the packet. Issues #521/#522 and their archived OpenSpec
changes prove implementation; NeoPRISM #325 records live downstream parity.
