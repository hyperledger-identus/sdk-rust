# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/157
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective: the dependency is chain-neutral RFC grammar
and stays private inside generic DID and protocol-semantics crates.
`SDK-COMPAT-004` and `SDK-COMPAT-005` remain effective because the candidate
declares Rust 1.68 and must pass the pinned Rust 1.98.1 lanes.
`SDK-SEC-001` remains effective: SDK source adds no unsafe block, while the
transitive `ref-cast` unsafe surface is explicitly attributed and reviewed as
dependency evidence rather than treated as an SDK-source exception.
`SDK-SEC-003` remains effective because all existing byte ceilings execute
before parsing. `SDK-DELIVERY-001` is satisfied by issue #157 and this
specification-first lifecycle.

## Introduced or changed constraints

No repository-wide constraint value changes. The `did-core` capability changes
from a dependency-free generic URI implementation to a private pinned grammar
dependency. Exact representation, public errors, resource bounds and public
ownership remain mandatory. This does not relax the foundation ring or permit
framework types in public APIs.

## Introduced or changed limitations

- IRI parsing, normalization, equivalence, resolution, networking and serde
  features are unsupported.
- `identus-core::Url`, `Did` and `DidUrl` remain on their current parsers.
- `uriparse 0.6.4` remains a DID development oracle and is not a runtime
  dependency after successful delivery.
- The candidate release/tag is unsigned; the exact crates.io checksum and
  source comparison are the provenance anchors.
- Transitive `ref-cast` unsafe code remains dependency-owned and must pass the
  focused security review; repository-wide automated unsafe enforcement is
  still tracked by #169.

## Consumer and product impact

No public type, serialized value or error changes. Standards-valid IPvFuture
is parsed directly in OID4VCI instead of through a substituted temporary.
Downstream consumers retain exact input spelling and all existing protocol
policy.

## Activation and rollback

The dependency and grammar-engine decision activates only when issue #157's
PR passes all local and hosted gates and merges to `develop`. Reverting that PR
restores local/`uriparse` mechanics without data migration. No claim extends to
`main`, publication or downstream adoption.

## Evidence

The research record identifies every consumer, normative source, exact
artifact/revision/license, MSRV, features, direct/resolved cone, target plan,
unsafe/native shape, supply-chain checks, public/wire compatibility, facade,
rollback, maintenance posture and explicit stop conditions.
