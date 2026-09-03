## 1. Contract and architecture

- [x] 1.1 Reconcile issue #47 with the current DIF draft, merged DID seams,
      immutable donor revisions, and a precise 70–80% boundary
- [x] 1.2 Record the draft-divergence, job/idempotency/cancellation, custody,
      runtime, cache, and downstream-isolation decisions in OpenSpec and ADR
- [x] 1.3 Complete a pre-implementation semantic and misuse-resistance review
      with no unresolved blocker

## 2. Registration domain contract

- [ ] 2.1 Add bounded redacted identifiers, public data, failure codes,
      secret modes, update operations, actions, jobs, requests, and results
- [ ] 2.2 Enforce request/state/action/identity invariants and native/JSON
      resource and private-material boundaries
- [ ] 2.3 Add the object-safe registrar port and exact optional registry dispatch

## 3. Conformance and misuse resistance

- [ ] 3.1 Add independent PRISM- and Midnight-shaped registrar mocks covering
      immediate and multi-step create/update/deactivate flows
- [ ] 3.2 Cover malformed states, method/job/identity mismatch, ordered updates,
      stale actions, idempotency conflicts, cancellation truth, and redaction
- [ ] 3.3 Cover bounds, constructor/JSON equivalence, object-safe concurrency,
      future-drop semantics, runtime isolation, and release performance

## 4. Delivery evidence

- [ ] 4.1 Run focused coverage and conformance plus workspace, feature, lint,
      docs, target, MSRV, factory, and full Nix gates
- [ ] 4.2 Complete a distinct semantic/security/API review and synchronize the
      canonical DID Core specification with exact verification evidence
- [ ] 4.3 Produce the factory receipt, archive the change, and deliver the
      signed issue-linked PR through the all-green `develop` merge policy
