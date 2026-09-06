## 1. Contract and standards

- [x] 1.1 Confirm #104, exact `develop` base, repository rules and downstream
  read-only receipts.
- [x] 1.2 Pin OpenID4VCI 1.0 Final Appendix D.1/F.1 and OpenID Federation 1.0
  Final section 4.
- [x] 1.3 Specify parsing, construction, provider, state, bound, error and
  non-goal contracts.
- [x] 1.4 Record ADR 0040 and complete pre-implementation architecture,
  standards, API, security and portability review with no blocker.

## 2. Protected header and holder construction

- [x] 2.1 Add bounded duplicate-safe protected-header evidence parsing,
  serialization, accessors and redacted Debug.
- [x] 2.2 Add validated holder evidence and source-compatible proof builder
  construction.
- [x] 2.3 Reject invalid compact-token shapes, excess chain depth and ambiguous
  trust-chain key sources before signing or provider calls.

## 3. Issuer verification and trust

- [x] 3.1 Add injected trust-chain key selection with exact `kid`, algorithm
  re-binding and outer proof verification.
- [x] 3.2 Add injected key-attestation validation bound to the verified proof
  key and nonce.
- [x] 3.3 Add explicit trust-evaluated state while keeping existing authorize
  and verify-and-authorize calls source compatible.
- [x] 3.4 Add stable redaction-safe errors and provider-call ceilings.

## 4. Evidence and integration

- [x] 4.1 Add positive construction/parsing/verification cases for each
  extension and their combination.
- [x] 4.2 Add independent malformed, missing-provider, rejected/unavailable,
  algorithm/key/nonce and provider-order negative tests.
- [x] 4.3 Update ADR, blueprint, inventory, canonical specs and API docs.
- [x] 4.4 Recheck every downstream HEAD/status receipt without mutation.

## 5. Verification and delivery

- [x] 5.1 Pass focused formatting, tests, minimal features, strict Clippy and
  warning-denied docs.
- [x] 5.2 Pass workspace, factory, target/MSRV, supply-chain and full Nix gates.
- [x] 5.3 Complete and record a distinct post-implementation review; resolve
  every finding.
- [x] 5.4 Produce ready/receipt, archive safely, sign+DCO commit, open #104-linked
  ready PR, obtain exact-head hosted review, merge only on all-green CI, sync
  develop and clean up.
