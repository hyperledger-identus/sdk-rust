# Tasks

## 1. Specification and decision

- [x] 1.1 Pin normative, donor, consumer, and wire-model revisions.
- [x] 1.2 Measure bounded ASCII, Unicode, context, and projection-amplification cases.
- [x] 1.3 Record ADR 0175 and the OpenSpec resource-policy delta.
- [x] 1.4 Bind an exact issue #524 preimplementation receipt after #493 merges.

## 2. Implementation

- [x] 2.1 Prototype literal DID/DID URL limits and independent method-name policy.
- [x] 2.2 Prototype observable oversized cache-key bypass under both failure modes.
- [x] 2.3 Prototype native, WASM, UniFFI, fuzz, vector, adapter, and architecture updates.
- [ ] 2.4 Apply the validated prototype to a fresh issue branch from merged `develop`.

## 3. Verification

- [x] 3.1 Pass disposable focused Rust, adapter, vector, target, fuzz, and factory gates.
- [ ] 3.2 Pass official exact-head local/Nix gates and independent Claude review.
- [ ] 3.3 Open the issue-linked PR, pass protected CI, publish metrics, and merge.
- [ ] 3.4 Archive the completed OpenSpec change honestly.

## 4. Stop boundary

- [x] 4.1 Confirm no PRISM method implementation, generic JSON-ingress widening, HTTP policy, or crypto codec change is included.
