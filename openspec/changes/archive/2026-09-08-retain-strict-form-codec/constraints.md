# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/158
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective because the local codec is protocol-generic
mechanics inside the OID4VCI crate and introduces no product or chain policy.
`SDK-SEC-001` and `SDK-SEC-002` remain effective: no unsafe/dependency is added
and secret-bearing outputs retain zeroizing ownership. `SDK-DELIVERY-001` is
satisfied by issue #158 and this spec-first lifecycle.

## Introduced or changed constraints

No repository-wide constraint value changes. Dependency research gains an
explicit fail-closed parser-equivalence rule. A candidate's happy-path format
support cannot authorize adoption when malformed, invalid-text or resource
behavior weakens an existing capability specification.

## Introduced or changed limitations

The SDK continues to own its small form codec and maintenance burden. It does
not accept browser-style lossy decoding or expose a generic form API. Candidate
source can be used as an oracle without becoming a runtime dependency.

## Consumer and product impact

There is no API, body-byte, parsing, error, storage or migration impact.
Credential-offer inputs remain strict, and pre-authorized token request bodies
remain deterministic, bounded and zeroizing.

## Activation and rollback

The corrected decision activates only after issue #158's PR merges to
`develop`. Reverting it restores the prior research wording without changing
runtime artifacts. A future adoption issue must explicitly update the ADR and
specification before Cargo changes.

## Evidence

The research record pins normative sources, current implementation, exact
artifact and VCS provenance, features/cone, permissive parsing, reachable
unsafe, strict SDK behavior, compatibility, rollback and finite
reconsideration triggers.
