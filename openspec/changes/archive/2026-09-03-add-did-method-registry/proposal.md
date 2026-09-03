## Why

Issue #44 is the next bounded child of #5 / #20 (`IDR-006`). The SDK now owns
validated DID values, W3C result envelopes, query options and object-safe query
ports, but a consumer still has to select a method implementation itself.
NeoPRISM and midnight-identity expose one method implementation at a time;
Lace and Oxid inject one resolver at a time. This prevents a wallet or service
from composing PRISM, Midnight and later generic DID methods behind one
chain-neutral SDK boundary.

## What Changes

- Add an immutable, bounded DID method registry to `identus-did`.
- Add method bindings that require a resolver and may independently provide a
  dereferencer.
- Reject duplicate method ownership and capacity exhaustion during setup.
- Implement the existing `DidResolver` and `DidUrlDereferencer` ports by exact
  validated method-name lookup.
- Map unknown methods and absent dereferencing support to current W3C result
  errors without a second invocation error channel.
- Prove deterministic concurrent lookup with independent PRISM- and
  Midnight-shaped adapters.

## Capabilities

### Modified Capabilities

- `did-core`: add deterministic method composition and dispatch over the
  existing DID query-port contract.

## Impact

- **Issue:** #44; depends on merged #43.
- **API:** additive setup and dispatch types in unpublished `identus-did`.
- **Dependencies:** standard library only; no lockfile change expected.
- **Consumers:** NeoPRISM and Midnight method adapters can be composed once;
  Lace and Oxid can inject the registry through the same query ports.
- **Deferred:** cache/clock #45, dereferencing algorithm #46, registrar #47,
  and HTTP/network policy #10.
- **Rollback:** revert issue #44's pull request; no persistence or downstream
  repository changes are involved.
