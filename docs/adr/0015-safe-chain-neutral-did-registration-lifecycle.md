# ADR 0015: isolate a safe chain-neutral DID Registration lifecycle

- **Status:** Accepted for implementation
- **Date:** 2026-09-03
- **Decision authority:** standing product mandate and `IDR-006e` issue #47
- **Normative evidence:** DIF DID Registration draft
  `ec1bf38f7860b361eb6692c02f742b9dfc48291b`
- **Related work:** issues #5, #43, #44, #46, #47, #50 and #51

## Context

PRISM and Midnight both create, update, and deactivate DIDs, but their operation
encodings, signing/custody, ledger submission, and finality models differ. The
existing Midnight donor trait is synchronous in lifecycle shape and omits
long-running state, action correlation, idempotency, and cancellation. Oxid and
Lace demonstrate why those omissions become wallet safety problems.

The DIF DID Registration draft offers common vocabulary but is not ratified.
It permits arbitrary raw secret input/output and contains a non-terminal action
example without the job id required by its own prose. It also leaves retry,
idempotency, and cancellation behavior unspecified. Directly mirroring this
wire shape in an identity-wallet foundation would freeze both draft volatility
and unsafe secret handling.

## Decision

1. Implement a replaceable internal Rust lifecycle profile in `identus-did`,
   not a DIF JSON/HTTP binding.
2. Model immutable create, update, deactivate, continue, and best-effort cancel
   requests. Require a bounded idempotency key on every request.
3. Make every non-terminal state carry a method-scoped opaque job and every
   terminal state carry none. Bind action ids into jobs and require exact
   continuation-response correlation.
4. Represent internal, external, and client-managed custody with opaque handles
   and bounded public action data. Expose no raw private keys, seeds, passwords,
   decrypted payloads, or generic secret bags.
5. Reject internal generation that neither stores material nor returns an
   opaque handle.
6. Preserve ordered set/add/remove/method-specific document operations, but do
   not interpret, diff, resolve, reorder, or claim atomicity for patches.
7. Define one object-safe asynchronous `DidRegistrar` and add it as an optional
   independent capability on `DidMethodBinding`. Registry routing uses the exact
   validated method carried by requests and jobs.
8. Treat identical idempotent replay as the same logical operation and
   different input under the same key as conflict. Durable enforcement remains
   in the method adapter.
9. Treat dropped futures as cancelled observation only. Explicit cancellation
   never fabricates rollback after irreversible work.
10. Keep I/O, persistence, clocks, retries, backoff, signing, key generation,
    VDR execution, finality, wallet confirmation, cache invalidation, and trust
    policy outside the generic crate.

## Consequences

- NeoPRISM, midnight-identity, Oxid, and Lace can converge on one lifecycle
  vocabulary without importing Cardano, Midnight, wallet, or runtime types.
- The SDK can represent immediate and multi-step operations, truthful partial
  outcomes, and safe retries while remaining stateless.
- A future DIF binding needs explicit translation and may not pass raw draft
  secret objects through this API.
- Adapters have a stronger contract: durable idempotency, job/action
  correlation, and honest cancellation/finality reporting are conformance
  obligations.
- The public surface is additive and pre-release. Existing query/cache behavior
  is unchanged unless an application explicitly binds a registrar.

## Deferred work

- DIF HTTP/JSON, execute and DID URL resource mutation;
- redirect/callback and decryption-action profiles;
- custody/signing providers, storage transactions and recovery/compensation;
- automatic resolver-cache coordination and downstream adoption.

## Rollback

Revert the issue #47 pull request. The capability owns no state or external
effects and does not modify existing method bindings unless the optional
registrar is selected.
