## 1. Contract and review

- [x] 1.1 Confirm issue #100, current `develop` base, standing authority and
  downstream isolation.
- [x] 1.2 Specify arbitrary input, bounded limit tuples, public invariants,
  corpus provenance, campaign resources, non-scope and rollback.
- [x] 1.3 Record ADR 0039 and complete pre-implementation architecture, API,
  security, compatibility and performance review with no blocker.

## 2. Fuzz target and evidence

- [ ] 2.1 Add the standalone JWS target with default/derived-limit parsing,
  canonicality, static-error, exact-input and round-trip assertions.
- [ ] 2.2 Add independently authored RFC/Oxid/Lace positive seeds, negative
  boundary families, dictionary and provenance documentation.
- [ ] 2.3 Add the validated replay/smoke/soak wrapper and path-scoped hosted
  sanitizer workflow with supply-chain checks and failure artifacts.

## 3. Verification and delivery

- [ ] 3.1 Pass corpus replay, deterministic smoke, fuzz formatting, strict
  Clippy, cargo-deny, RustSec and existing DID/crypto replay regressions.
- [ ] 3.2 Pass JOSE/workspace tests, strict lints/docs, factory and full Nix
  host/MSRV/target/supply-chain gates.
- [ ] 3.3 Complete distinct post-implementation review and resolve findings.
- [ ] 3.4 Record effort/performance receipt, sync canonical specs, archive the
  change and deliver the signed/DCO issue-linked all-green PR.
