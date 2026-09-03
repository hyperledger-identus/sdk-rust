# Change: add a safe chain-neutral DID Registration lifecycle

## Why

The SDK has bounded DID documents, resolution and dereferencing results,
object-safe query ports, caching, and exact method dispatch. It has no reusable
contract for creating, updating, or deactivating a DID when a method requires
signing actions, ledger finality, or multiple calls. PRISM, Midnight, and wallet
consumers therefore expose different synchronous lifecycle seams and repeat
security-sensitive job, custody, retry, and idempotency decisions.

The DIF DID Registration specification remains an unratified draft. It provides
useful operation and state vocabulary but permits raw secret interchange,
omits mature idempotency and cancellation semantics, and contains contradictory
job examples. Copying its JSON directly into a foundational SDK would create an
unsafe and unstable compatibility commitment.

## What changes

- Add immutable create, update, deactivate, continue, and cancel requests.
- Add bounded idempotency, method-scoped job, action-correlation, and opaque
  custody-handle values with redacted diagnostics.
- Model complete-document replacement plus ordered standard and method-specific
  public update data without applying patches in the generic layer.
- Model internal, external, and client-managed secret modes without raw private
  key, seed, password, or decrypted-payload fields.
- Add validated terminal and non-terminal registration states, bounded public
  metadata, action requests/responses, retry hints, and identity checks.
- Add an object-safe async `DidRegistrar` port and optional exact registrar
  dispatch through the existing immutable DID method registry.
- Record PRISM/Midnight-shaped adapter proofs, adversarial state/secret tests,
  coverage, performance, and full target evidence.

## Boundaries

- The new types are an internal Rust domain contract, not the DIF JSON/HTTP
  binding and not a promise of byte compatibility with the draft.
- Adapters own idempotency persistence, job state, signing/custody, VDR calls,
  finality, compensation, and method-specific validation.
- Callers own polling schedules, retry/attempt/deadline policy, confirmation,
  authorization, and automatic cache invalidation.
- DIF `execute`, DID URL resource mutation, redirects, decryption actions, raw
  secret return, HTTP, and downstream adoption remain separate work.
- No downstream repository is modified and no donor source or fixture is
  copied.

## Delivery

Issue #47 and ADR 0015 precede implementation. Delivery requires a signed+DCO
specification commit, distinct semantic/security/API review, focused coverage
above 80%, complete Nix gates, and exact-head hosted CI before merge into
`develop`.
