# Tasks

## 1. Research and specification

- [x] 1.1 Refine issue #356 and classify response correlation as a material,
  roadmap-directed security transition.
- [x] 1.2 Pin Final/RFC response, state, issuer-identification, grammar,
  callback-boundary, compatibility, reuse and resource decisions.
- [x] 1.3 Pass research/constraint readiness, commit the planning contract,
  and bind its immutable preimplementation receipt.

## 2. Implementation

- [ ] 2.1 Extend Authorization Server Metadata with the bounded RFC 9207
  support flag and exact false default.
- [ ] 2.2 Factor private shared OAuth grammar without changing token-error
  behavior.
- [ ] 2.3 Add bounded strict query parsing, duplicate defense and exact state
  and selected-server issuer correlation.
- [ ] 2.4 Add closed success/error outcomes with zeroizing values and redacted
  diagnostics.
- [ ] 2.5 Add normative, delegated-server, adversarial, boundary, ownership,
  metadata and regression tests.
- [ ] 2.6 Add the ADR and update inventory, blueprint and roadmap evidence with
  a focused code-exchange-request successor.

## 3. Review and delivery

- [ ] 3.1 Complete focused/full gates and exact-diff review.
- [ ] 3.2 Prepare the OpenSpec archive; merge remains gated by the hosted
  `fast` lane.
