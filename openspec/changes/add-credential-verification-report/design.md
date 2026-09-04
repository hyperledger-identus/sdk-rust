## Context

IDR-009 calls for structural, issuer/key, proof, temporal, status, and schema
verification evidence without product trust decisions. Oxid currently models
seven stages including trust, stores stages in a heap vector, and accepts an
aggregate outcome that must then be checked for consistency. Midnight status
verification and NeoPRISM status-list storage are adapter evidence producers,
not generic domain dependencies.

The implementation base is
`develop@4e4cf5a8b4fb66e289629d287b4b87c63d5fc2bf`. Exact donor revisions,
paths, digests, licenses, and the conceptual-adaptation classification are in
#73. Every donor/consumer repository is read-only.

## Goals / Non-Goals

**Goals:**

- make a complete verification pipeline state impossible to contradict;
- separate evidence validity from product trust and acceptance;
- preserve specific machine-readable failure/not-checked reasons safely;
- keep construction bounded, allocation-light, and deterministic;
- let Midnight, Cardano, and future adapters contribute stage results without
  becoming SDK dependencies.

**Non-goals:**

- metadata, claims, schema descriptors, holder/status bindings, disclosure,
  status policy, trust policy, verification execution, or resolver ports;
- evidence payload bytes, errors from verifier operations, clocks, serde,
  codecs, storage, FFI, publication, or downstream adoption.

## Decisions

### D1 — Verification is a distinct capability inside the credential crate

Implement the first IDR-009 slice in `identus-credentials`, beside but not
inside the envelope types. A new crate would add packaging and dependency
surface without separating a reusable concern: the report is credential
semantics and has no execution adapters.

Metadata/schema work remains IDR-007b. Mixing it into this change would join
two roadmap capabilities and make the result harder to review and reuse.

### D2 — Six fixed policy-neutral stages

`VerificationStageName` has exactly Structural, IssuerKey, Proof, Temporal,
Status, and Schema in canonical order. Trust is absent because trust answers
whether a relying party accepts otherwise valid evidence; it does not change
whether structure, keys, proofs, time, status, or schema checks succeeded.

The taxonomy is intentionally closed for this first contract. Adding a
normative stage changes aggregate semantics and requires a compatibility
decision rather than silently accepting an extension that old consumers do
not evaluate.

### D3 — Stage state carries bounded reasons

`VerificationStageStatus` is Passed, Failed, or NotChecked. A passed stage
forbids a reason. Failed and not-checked stages require a
`VerificationReasonCode`, allowing callers to distinguish invalid evidence
from unavailable, unsupported, or inapplicable checks without embedding human
or secret detail.

Reason codes accept 1–128 lowercase ASCII bytes. The first byte is
alphanumeric; remaining bytes are lowercase alphanumeric or `.`, `_`, `-`,
`:`, permitting namespaced codes such as `proof.invalid_signature` and
`status:not_supported`. Validation precedes the single successful string
allocation. Errors never retain or echo rejected input.

### D4 — Complete fixed array and derived outcome

`VerificationReport::new` accepts `[VerificationStage; 6]` and requires exact
canonical stage order. This representation makes missing/extra stages
unrepresentable at the type boundary and rejects duplicates/reordering in one
six-item pass. It stores the same array and a derived outcome:

1. any Failed stage means Invalid;
2. otherwise any NotChecked stage means Indeterminate;
3. otherwise the result is Valid.

Callers cannot provide a conflicting aggregate outcome. Canonical storage
makes `stage(name)` direct array indexing and avoids map/set/vector allocation.

### D5 — No trust or execution claim

The report states only verifier-produced evidence. A valid report can be
rejected because the issuer is untrusted, and an invalid report remains
invalid even if the issuer is trusted. Tests keep those booleans outside the
type to demonstrate the separation.

Adapters own cryptography, DID resolution, time, schema validation, status
lookup, and mode-specific evidence. Operational errors stay in those APIs;
they can be projected into a not-checked reason only when their contract says
that is appropriate.

### D6 — Stable construction errors and measured hot path

Extend `CredentialError` with invalid reason code, missing/unexpected reason,
and non-canonical report errors. All bridge through the existing `credential`
capability with static `credential.*` codes and `InvalidInput` kind.

A manual ignored release-mode diagnostic measures complete valid-report
construction. It prints throughput but has no wall-clock assertion, avoiding
flaky correctness while keeping performance visible. The design target is a
constant six-stage scan, no report collection allocation, and direct lookup.

## Risks / Trade-offs

- A fixed taxonomy requires a future compatibility decision for new normative
  stages. That is safer than different consumers silently computing different
  aggregate outcomes.
- Canonical order is stricter than a map input but removes duplicate/order
  ambiguity and heap work. Adapters have only six compile-time-known entries.
- Reason codes intentionally cannot carry prose or dynamic diagnostics. Such
  detail belongs in adapter-local telemetry, not portable credential state.
- `Indeterminate` is not validity or acceptance. Product policy decides whether
  and how to retry or reject incomplete evidence.

## Migration Plan

1. Land this contract and ADR before implementation.
2. Add the focused module, error extensions, tests, and inventory update.
3. Run focused/full gates and the release diagnostic; complete a fresh exact-
   diff API/security/performance review.
4. Sync canonical specs, archive, receipt, and deliver a signed/DCO PR to
   `develop`.

Rollback is a two-commit revert while the crate is unpublished. Consumer
adoption, concrete verifier adapters, and removal from donor repositories are
separate issue-first changes.
