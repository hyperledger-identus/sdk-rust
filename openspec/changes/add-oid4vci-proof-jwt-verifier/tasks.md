## 1. Contract and standards

- [x] 1.1 Confirm #99, exact `develop` base, JOSE/DID/clock readiness,
  repository rules and downstream isolation.
- [x] 1.2 Pin OpenID4VCI 1.0 Final Appendix F.1/F.4 and Section 13.8 plus the
  inherited RFC 7515/7519/8725 signature requirements.
- [x] 1.3 Specify staged states, key-reference providers, caller policy,
  replay semantics, errors, bounds, dependency cone and non-goals.
- [x] 1.4 Record ADR 0037 and complete pre-implementation architecture, API,
  standards, security and performance review with no unresolved blocker.

## 2. Profile and key verification

- [x] 2.1 Add bounded duplicate-safe proof-claim parsing and the explicit
  parsed state.
- [x] 2.2 Add inline-JWK, authenticated DID-URL and injected `x5c` key
  selection plus exact suite-registry signature verification.
- [x] 2.3 Add explicit cryptographically verified state and static redacted
  key-resolution/provider failures.

## 3. Issuer authorization

- [x] 3.1 Add validated client, audience, nonce, freshness and skew policy.
- [x] 3.2 Add injected wall-clock evaluation and checked NumericDate
  arithmetic.
- [x] 3.3 Add the atomic replay-guard port/input and authorized proof state,
  invoking replay only after every prior gate passes.

## 4. Evidence and integration

- [x] 4.1 Add Final-profile inline-JWK, DID-backed and `x5c` provider positive
  cases plus exact staged-state assertions.
- [x] 4.2 Add independent negative tests for type/algorithm/key reference,
  claims/client/audience/nonce/time, DID relationship/material, signature,
  certificate provider, replay, provider isolation and diagnostics.
- [x] 4.3 Add independently reconstructed Oxid behavior and Lace deferred-gap
  evidence plus a release-only verifier throughput diagnostic.
- [x] 4.4 Update the dependency guard, blueprint/inventory/specs/docs and verify
  downstream HEAD/status receipts remain unchanged.

## 5. Verification and delivery

- [x] 5.1 Pass focused formatting, tests, minimal features, strict Clippy and
  warning-denied docs.
- [x] 5.2 Pass workspace, factory/conformance, target/MSRV, supply-chain and
  full Nix gates.
- [x] 5.3 Complete a distinct post-implementation architecture/API/standards/
  security/performance review and resolve every finding.
- [ ] 5.4 Produce ready/receipt, sync canonical specs, archive the change and
  prepare the signed/DCO issue-linked PR; hosted review, green CI, merge,
  roadmap receipts and cleanup remain GitHub evidence.
