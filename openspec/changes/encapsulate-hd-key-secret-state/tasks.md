# Tasks

## 1. Contract and decision

- [x] 1.1 Record issue #269, exact base, threat/copy-boundary analysis,
  compatibility decision, research and constraints.
- [x] 1.2 Add ADR 0114 and a complete `crypto` requirement delta.
- [x] 1.3 Complete pre-implementation semantic, security and API review.
- [x] 1.4 Commit the planning-only contract and create the durable preflight
  receipt before implementation.

## 2. Opaque secret boundary

- [x] 2.1 Make both HD private-key and chain-code fields private without
  changing derivation behavior or public metadata.
- [x] 2.2 Add the fixed, SDK-owned, redacted and drop-zeroizing exposure value
  plus named private-key/chain-code exposure methods.
- [x] 2.3 Migrate repository vectors and internal tests through the explicit
  boundary while retaining byte-identical results.
- [x] 2.4 Add compile-fail field, Display and Serde regressions plus runtime
  redaction and erasure tests.

## 3. Candidate and target evidence

- [x] 3.1 Update the unpublished `0.1.0-rc.1` public API baseline and record the
  intentional pre-release SemVer assessment.
- [ ] 3.2 Pass focused default, minimal, all-feature and KMP crypto gates.
- [ ] 3.3 Pass relevant native, WASM, iOS, Android, Nix, factory and candidate
  gates, reporting every unrun command exactly.

## 4. Review and delivery

- [ ] 4.1 Complete a distinct exact-diff architecture/security/API review and
  resolve every finding.
- [ ] 4.2 Record verification evidence, mark tasks complete, run ready/receipt,
  synchronize canonical specs and archive through the factory facade.
- [ ] 4.3 Push signed/DCO commits and open a ready issue-linked PR to
  `develop`; do not publish, release, touch consumers, or advance #7/#168.
