# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/394
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: the new crate remains chain- and product-neutral.
- `SDK-ARCH-002`: dependencies point inward to core plus reviewed utilities.
- `SDK-COMPAT-001`, `002`, `004`, `005`: stable Rust 1.89 source floor and
  Rust 1.98.1 primary/etalon rules remain unchanged.
- `SDK-SEC-001`, `002`, `003`: unsafe prohibition, secret redaction and bounded
  untrusted-input requirements apply.
- `SDK-REL-001`: the new crate remains unpublished and outside the activated
  three-crate release train.

## Introduced or changed constraints

The roadmap's IDR-024 ownership becomes concrete through an unpublished
`identus-oid4vp` package. Its first accepted public behavior is reference
transport syntax only. It may not infer Request Object authenticity, verifier
trust or protocol completion from parsing.

## Introduced or changed limitations

- Only Request Objects by HTTPS reference are supported initially.
- Inline parameters, by-value Request Objects, `transaction_data`, client
  identifier interpretation, retrieval/JAR, DCQL and responses are unsupported.
- WASM/iOS/Android evidence is compile-only; no runtime target is promised.
- The crate is source-only `0.0.0` and not part of a release candidate.

## Consumer and product impact

Oxid and Lace ID Portal gain a future generic ingress candidate, but no
consumer changes or compatibility claim occur. Oxid's loopback HTTP demo
remains downstream-specific and is not accepted into the generic boundary.

## Activation and rollback

The API activates only after issue-linked reviewed merge to `develop`.
Rollback removes the package, inventory/backlog record, ADR and specification;
no released artifact, stored data, downstream adapter or wire migration exists.

## Evidence

Issue #394 and ADR 0157 are exact authority. Focused negative/resource tests,
dependency guards, portable compiles and exact-diff review prove compliance.
