# Tasks

## 1. Specification and planning

- [x] 1.1 Audit the schema-v1 ledger, A1 registries, factory lifecycle, and
      accepted compatibility/deprecation decisions at the exact base.
- [x] 1.2 Decide schema-v2 identity, lifecycle, cross-record, compatibility,
      empty-state, impact-disposition, and deterministic-rendering rules.
- [x] 1.3 Specify the synthetic DID proof without creating a canonical
      consumer-migration claim.
- [x] 1.4 Commit this planning packet and bind an exact preimplementation
      receipt to issue #422.

## 2. Registry and cross-record validation

- [ ] 2.1 Upgrade the canonical ledger to a closed schema-v2 empty registry.
- [ ] 2.2 Add the bounded offline validator and class/lifecycle matrices.
- [ ] 2.3 Resolve capability-coherent vector, mapping, and quality IDs without
      copying payloads or evaluating promotion freshness.
- [ ] 2.4 Add the noncanonical synthetic DID fixture and focused mutation
      suite for every critical field and graph edge.

## 3. Compatibility disposition and rendering

- [ ] 3.1 Add the closed OpenSpec compatibility-impact declaration and make
      qualifying factory readiness reject omission or contradiction.
- [ ] 3.2 Render deterministic release-note, migration, compatibility-window,
      rollback/removal, limitation, and honest empty-state views.
- [ ] 3.3 Integrate validator/render drift with the factory facade and update
      issue/evidence templates to reference stable ledger IDs.

## 4. Verification and closeout

- [ ] 4.1 Run focused ledger, mutation, rendering, graph, and impact tests.
- [ ] 4.2 Run OpenSpec readiness, repository factory, diff, signature, and
      proportionate Nix gates.
- [ ] 4.3 Perform a distinct architecture/security review and prepare archive,
      metrics, protected-PR, #504, and #492 handoff evidence.

## 5. Explicit stop boundary

- [ ] 5.1 Confirm the canonical ledger remains empty, the synthetic record is
      test-only, no Rust/consumer behavior changed, no deprecation or release
      was activated, and no downstream repository changed.
