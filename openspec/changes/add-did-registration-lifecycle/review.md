# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #47 (child of #5 / `IDR-006`)
- **Develop base:** `558677ef0359f9af241232ed13422a005d5cc5b0`
- **Reviewed contract:** OpenSpec `add-did-registration-lifecycle` and ADR 0015
- **Result:** no unresolved blocker

## Findings

1. **Draft stability:** DIF DID Registration is unratified and its job prose
   conflicts with examples. Pin the commit, prefer the coherent invariant, and
   avoid a direct wire promise.
2. **Secret safety:** the draft's raw `secret` maps and `returnSecrets` cannot
   enter the generic SDK. Opaque custody handles and public action data preserve
   implementability without moving private bytes.
3. **Destructive default:** internal generation with neither storage nor a
   returned handle can permanently lose control. Reject that combination.
4. **Method routing:** continuations need an explicit validated method outside
   the opaque job token so immutable registry dispatch never parses or guesses.
5. **Action replay:** action response ids must be bound into the job and checked
   before dispatch; otherwise stale or cross-job signatures can be injected.
6. **Idempotency:** the port can require immutable equality-comparable requests
   and keys but cannot provide durable replay storage. Enforcement is an
   adapter obligation and conflicts must be explicit.
7. **Cancellation:** cancelling observation is not cancelling ledger work. An
   explicit best-effort request preserves truthful partial/final outcomes.
8. **Patch semantics:** complete replace is generic; add/remove and
   method-specific payloads can be bounded and ordered but are interpreted only
   by the method adapter. The core must not resolve/diff/reorder them.
9. **Public open data:** method extensions are necessary, but all such maps need
   shared resource limits and recursive private-material rejection. Debug must
   redact the entire content rather than rely on field-name filtering.
10. **Cache boundary:** update/deactivate results expose the DID, but automatic
    invalidation remains outer so partial visibility and consistency policy are
    not hidden.
11. **Consumer proof:** PRISM and Midnight differ materially in operation
    encoding, custody, and finality. In-memory mocks can prove the generic seam;
    production downstream edits are neither required nor authorized.
12. **Scope:** redirect callback integrity, decryption plaintext, HTTP, DIF
    execute/resource operations, persistence, fees, finality policy, wallet
    consent, and compensation require independent contracts.

# Post-implementation semantic, security and API review

- **Reviewed head:** `5d178dba7047c145a4e0f0dda6bd1a86b5a40c31`
- **Review completed:** 2026-09-03T07:36:05Z
- **Result:** passed with no unresolved finding

The exact diff from `558677ef0359f9af241232ed13422a005d5cc5b0`
was re-read after focused conformance, both workspace feature modes, strict
Clippy, rustdoc, exact-head coverage, release performance and all compatible
Nix checks. The public surface is additive to the unpublished `identus-did`
crate: closed requests, validated jobs/results, one object-safe registrar and
one optional method-binding capability. Existing resolution, dereferencing and
cache behavior is unchanged unless an application explicitly selects it.

The review verified that create/update/deactivate identity is exact, updates
retain order, continuations cannot inject a stale action, terminal jobs and
jobless non-terminal states cannot be constructed, and wait hints stay within
24 hours. Every mutation carries an idempotency key, but durable replay and
conflict storage remain an explicit adapter obligation. Dropping a future only
drops observation; cancellation remains a request and cannot claim rollback.

Secret modes expose policy or opaque custody references, never key bytes.
Registration maps, complete documents and document metadata are recursively
checked for reserved/private-material-shaped fields. Their diagnostics reveal
only variants, counts and presence. Raw JSON, strings, maps, arrays, depth,
nodes, identifiers, handles and returned-handle sets are bounded before port
dispatch. Requests/results deliberately have no DIF JSON representation.

PRISM-shaped immediate and Midnight-shaped action/wait/finality mocks exercise
the same `Arc<dyn DidRegistrar>` seam without method types entering core. Exact
registry lookup has no fallback or prefix behavior; known missing and unknown
methods return different valid terminal codes. There is no network,
filesystem, clock, entropy, storage, signing, cache mutation, chain or runtime
dependency in the generic implementation.

The Nix MSRV gate found one new-syntax `if let` chain accepted by the local
compiler but not Rust 1.85. It was replaced with equivalent nested conditions,
re-reviewed and rerun through the complete matrix. No downstream repository was
edited, switched, staged, copied from or built, and SDK `main` remains intact.
