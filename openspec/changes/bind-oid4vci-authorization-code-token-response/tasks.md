# Tasks

## 1. Research and specification

- [x] 1.1 Create issue #360 before implementation.
- [x] 1.2 Pin Final/RFC behavior, status/header policy, reuse/dependency and
  compatibility decisions from primary sources.
- [x] 1.3 Pass research and constraint readiness; commit this planning-only
  contract and bind its immutable preimplementation receipt.

## 2. Implementation

- [x] 2.1 Add positive header/body limits and the one-shot request-bound
  success/error transition with early request-secret erasure.
- [x] 2.2 Reuse/generalize strict private HTTP field parsing and append stable
  static diagnostics without changing prior behavior or rows.
- [x] 2.3 Add exact status/header/body precedence, boundary, ownership,
  lineage/evidence and redaction tests.
- [x] 2.4 Add ADR 0145 and update inventory/blueprint/roadmap evidence with a
  focused successor.

## 3. Review and delivery

- [ ] 3.1 Complete focused/full gates and an exact-diff local review.
- [ ] 3.2 Prepare the OpenSpec archive, publish bounded metrics and merge only
  after the hosted `fast` lane passes.
