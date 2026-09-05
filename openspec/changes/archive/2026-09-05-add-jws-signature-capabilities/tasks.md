## 1. Contract and standards

- [x] 1.1 Confirm issue #98, exact `develop` base, current crypto/JWK surfaces,
  consumer isolation and applicable repository rules.
- [x] 1.2 Pin RFC 7515, RFC 7518, RFC 8037, RFC 8725 and RFC 9864; record the
  fully specified `Ed25519` decision and explicit legacy posture.
- [x] 1.3 Specify closed algorithms, key binding, signer/verifier ports,
  registry bounds, explicit verified state, errors and non-scope.
- [x] 1.4 Record ADR 0035 and complete pre-implementation architecture, API,
  standards, security and performance review with no unresolved blocker.

## 2. Primitive and capability implementation

- [x] 2.1 Add fixed-width P-256 sign/verify methods without changing existing
  DER behavior.
- [x] 2.2 Add algorithms, bound verification keys and redaction-safe errors.
- [x] 2.3 Add synchronous signer port and Ed25519/ES256 software adapters.
- [x] 2.4 Add bounded suite registry, built-in strict verifiers and explicit
  verified state.

## 3. Evidence and integration

- [x] 3.1 Add RFC 8037, fully specified Ed25519 and independent ES256 positive
  conformance tests.
- [x] 3.2 Add negative tests for algorithm/key confusion, legacy opt-in,
  malformed keys/signatures, duplicates, capacity and exact input handling.
- [x] 3.3 Add redaction assertions and a release-only verification performance
  diagnostic.
- [x] 3.4 Update canonical architecture/specs, docs and issue receipts; verify
  both observed consumer checkout states remain unchanged.

## 4. Verification and delivery

- [x] 4.1 Pass focused formatting, tests, no-default, strict Clippy and docs.
- [x] 4.2 Pass workspace, factory/conformance, target/MSRV, supply-chain and
  full Nix gates.
- [x] 4.3 Complete a distinct post-implementation architecture/API/standards/
  security/performance review and resolve every finding.
- [x] 4.4 Produce ready/receipt, sync canonical specs, archive the change and
  prepare the signed/DCO issue-linked PR; hosted review, green CI, merge,
  roadmap receipts and cleanup remain GitHub evidence.
