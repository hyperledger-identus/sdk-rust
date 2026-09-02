## 1. Contract and architecture

- [x] 1.1 Create child issue #28 under IDR-004 parent #9 with immutable source,
      license, scope, threat, vector, verification, rollback and adoption data
- [x] 1.2 Add the OpenSpec proposal, crypto delta, design and ADR 0005 before
      implementation; record semantic review with no uncleared blocker

## 2. Validated public-key JWK

- [x] 2.1 Implement typed key/curve profiles, public-only private fields,
      constructors, accessors, extensions and `JwkError`
- [x] 2.2 Implement validating serde and canonical 32-byte coordinate gates
- [x] 2.3 Migrate all curve `EncodeJwk` implementations and crate exports
- [x] 2.4 Add feature/dependency wiring without breaking wasm or minimal builds

## 3. Conformance and misuse resistance

- [x] 3.1 Add RFC 8037, encoder parity, round-trip and extension-preservation tests
- [x] 3.2 Add constructor and serde rejection tests for every recorded threat
- [x] 3.3 Verify JWK and serde errors do not expose caller-supplied values
- [ ] 3.4 Run focused, workspace, minimal-feature, wasm, conformance, lint,
      formatting, docs, OpenSpec and factory gates

## 4. Delivery evidence

- [ ] 4.1 Sync the canonical crypto spec, archive the OpenSpec change and record
      exact verification plus a distinct local misuse-resistance/security review
- [ ] 4.2 Open an issue-linked PR to `develop`, obtain exact-head CI/review,
      merge only when every required gate is green, update #9 and clean up
