# OID4VCI Final conformance-closeout research

Research class: protocol
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The current implementation is the unpublished `identus-oid4vci` version
`0.0.0` crate at `develop@7f23129a245072e34ea5ddf1e601e14a4b493ed5`.
It has no default feature and directly depends only on the workspace-pinned
`identus-core`, `identus-jose`, `serde`, `serde_json`, `sha2`, `url`,
`zeroize` and `thiserror` surfaces. Thirty-seven issue-first deliveries expose
bounded offer, metadata, grant, authorization, token, nonce, credential and
deferred-credential types and state transitions. Thirty-six integration-test
files and thirty-four canonical OID4VCI capability specs exist, but their facts
are not summarized in one closed coverage inventory.

Issue #7 still says the Oxid/Lace ID Portal vector set transfers into this
crate and both app teams sign off. That wording predates the current bounded
architecture and cannot be treated as delivered merely because equivalent
repository-authored tests exist.

## Normative sources

- [OpenID for Verifiable Credential Issuance 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  published 2025-09-16, sections 4 through 12 and appendices where referenced,
  retrieved 2026-09-25. The immutable Final publication, not a draft, is the
  protocol baseline; errata migration remains a separate decision.
- SDK base revision
  `7f23129a245072e34ea5ddf1e601e14a4b493ed5`, including canonical OpenSpec
  contracts and tests under `crates/oid4vci`.
- Read-only consumer evidence: MediaNoxLabs/oxid
  `e9ecfe5df27c0790dac40776e1d0a37e5f99f907` and
  input-output-hk/lace-id-portal profile revision
  `25499870f84d77173c46e4af3021311decfb840b`, whose provenance pins Portal
  baseline `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` and profile source
  `76e8edf394a4cb37ca822037272d543c68f25f71`.

The Oxid checkout was inspected read-only at
`183664aeca500c25d6d27a22fa402b4d40c649d3` with four pre-existing status
entries. The Lace ID Portal checkout was inspected read-only at
`d284b85bfcbb4a7e5d2200837703419c145c60f5` with one pre-existing status
entry. Neither consumer is an implementation target.

## Candidate decisions

| Candidate | Decision | Evidence and reason |
| --- | --- | --- |
| Narrative-only closeout report | `not-adopt` | It drifts and cannot prove that paths, gap owners or status vocabulary remain valid. |
| Closed CSV matrix plus offline validator | `adopt` | It is reviewable, diffable, tool-independent and can fail when repository evidence or ownership disappears. |
| Generate all public documentation from code introspection | `not-adopt` | Rust syntax alone cannot establish standards scope, unsupported behavior, normative source or consumer evidence. |
| Copy the complete Lace vector directory now | `not-adopt` | The exact Portal revisions contain no explicit repository license, and several vectors are the Midnight-specific `midnight_cbor_phase1` profile rather than chain-neutral protocol evidence. |
| Record immutable Oxid/Portal vectors as reference-only evidence | `oracle` | Their hashes and source revisions are useful interoperability evidence without copying ambiguous or product-specific artifacts. |
| Recreate generic protocol vectors in SDK ownership where a gap exists | `conditional-adopt` | Only focused follow-up issues with repository-authored inputs and no donor copying may do this. |
| Add missing protocol behavior during reconciliation | `not-adopt` | A discovered functional gap requires its own issue and OpenSpec contract. |

## Compatibility and dependency evidence

The change adds evidence, an offline checker and checker tests only. It changes
no public Rust API, wire representation, error taxonomy, Cargo feature, MSRV,
target claim, manifest or lockfile. The direct and resolved dependency cone is
therefore unchanged. The matrix is a maintainer/conformance artifact behind the
repository facade boundary, not a runtime facade or compatibility promise.
Rollback deletes the additive matrix, report, validator and factory hook; no
consumer migration or stored-data recovery is required.

The checker uses Python 3 standard-library CSV/path handling already present in
the factory. No new crate, package, supply-chain input, native code, build
script or unsafe code is introduced. Plain Cargo consumers do not execute it.

## Security, privacy and maintenance evidence

The matrix contains repository-relative paths, public standard URLs, issue
numbers and status text only. It stores no credentials, tokens, proofs, private
keys, PII or consumer payloads. Paths are bounded, normalized and prohibited
from escaping the repository. Closed status and provenance vocabularies prevent
an unreviewed label from becoming a conformance claim.

Maintenance is limited to changing a row when a capability or limitation
changes; the factory validates the result. Release and security posture do not
change: this is coverage evidence, not official certification, penetration
testing, proof of issuer trust or authorization to publish.

The Oxid repository is Apache-2.0 at the pinned evidence revision. The Portal
fixture provenance is detailed and hash-pinned, but the Portal repository has
no explicit license at the exact source revisions. License and provenance are
therefore sufficient for an immutable reference, not for copying. A source
license clarification plus chain-neutrality review is the reconsideration trigger
for fixture import.

## Rejected or deferred candidates

Official certification claims, percentage scores, runtime reflection, network
fetching in the checker, consumer submodules, copied Midnight-profile fixtures,
format-specific parsers, encrypted responses, notifications, PAR, DPoP, client
attestation and HTTP execution are rejected or deferred. Unsupported and
partial rows remain visible limitations with focused owner issues rather than
being counted as success.

## Open questions and blockers

No blocker exists for creating the matrix and closeout recommendation. Missing
Portal license evidence blocks fixture copying, not this evidence-only change.
Whether M4 can close is an output of the reviewed matrix; any missing required
wallet-core behavior will receive a separate focused issue rather than being
implemented here.

## Evidence commands

```text
scripts/check-oid4vci-conformance.py
python3 -m unittest scripts/tests/oid4vci-conformance.py
cargo test -p identus-oid4vci --all-features
cargo test -p identus-oid4vci --no-default-features
scripts/factory check
scripts/factory backlog-live
```

The exact commands above are planned. Full workspace and portable target
commands run before delivery. Nix is available through its absolute host
profile but is unrun during planning; hosted `fast` supplies the required exact
candidate gate.
