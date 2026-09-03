## Why

Backlog item `IDR-006` requires one chain-neutral invocation boundary after the
validated DID syntax, document and result contracts delivered under `IDR-005`.
NeoPRISM, midnight-identity, Lace ID Portal and Oxid currently duplicate async
resolver traits and option records with incompatible identifiers, outputs and
error channels.

GitHub issue #43 defines the first independently testable `IDR-006` child. It
pins the W3C DID Resolution v1 Candidate Recommendation Draft dated 28 August
2026. HTTP remains #10, while method dispatch, cache/clock policy and the
still-draft registrar state machine remain later children.

## What Changes

- Add immutable, bounded resolution and dereferencing option maps to
  `identus-did`.
- Type the common W3C options while preserving registered or method-defined
  extension members.
- Add object-safe `DidResolver` and `DidUrlDereferencer` ports using explicit
  boxed `Send` futures and the existing validated result envelopes.
- Keep W3C result errors inside those envelopes instead of adding a competing
  transport or infrastructure error channel.
- Prove injection and option forwarding through two independent,
  consumer-shaped mock method implementations.
- Record the query-port boundary and the deliberately deferred stateful ports
  in ADR 0011.

## Capabilities

### Modified Capabilities

- `did-core`: extend the DID result capability with bounded input options and
  runtime-neutral query ports.

## Impact

- **Issue:** #43, child of #5 and #20 (`IDR-006`).
- **API:** additive types and traits in the unpublished `identus-did` package.
- **Dependencies:** no new third-party package and no lockfile change expected.
- **Consumers:** NeoPRISM and Midnight method implementations plus Lace/Oxid
  callers can share one injection seam; no downstream repository is modified.
- **Sequencing:** #10 consumes this port from an HTTP adapter; registry,
  cache/clock, dereferencing algorithms and registrar state remain separate.
- **Rollback:** revert issue #43's pull request. No published API, stored SDK
  state or downstream migration is involved.
