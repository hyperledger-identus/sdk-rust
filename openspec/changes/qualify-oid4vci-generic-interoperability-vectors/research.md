# Generic OID4VCI interoperability-vector research

Research class: protocol
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The SDK base is
`develop@149a35e62fc54f91dc99a496feb884d5b2f92cce`. The unpublished
`identus-oid4vci` crate now has bounded public wallet-side APIs for offer,
metadata, both grant paths, token, nonce, credential and deferred-credential
state. The machine matrix reports fourteen implemented, five partial, four
unsupported and one missing row. The missing row is cross-consumer vector
evidence owned by issue #376.

The current implementation is therefore functionally broad but lacks one
cohesive provenance-bearing cross-consumer fixture packet.

The existing crate tests are extensive but independently shaped. They do not
provide one provenance-bearing fixture packet that demonstrates a generic
consumer journey, deliberate legacy rejection and drift protection.

## Normative sources

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  especially sections 4, 6, 7, 8 and 12; retrieved 2026-09-25 from
  `https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html`.
- SDK base and canonical capability contracts at
  `149a35e62fc54f91dc99a496feb884d5b2f92cce`.
- MediaNoxLabs/oxid
  `e9ecfe5df27c0790dac40776e1d0a37e5f99f907`, Apache-2.0, including
  `docs/adr/0101-gate-laceid-portal-interoperability-on-final-openid4vci.md`
  and `crates/adapters/openid4vci/src/laceid_portal_contract_tests.rs`.
- input-output-hk/lace-id-portal profile revision
  `25499870f84d77173c46e4af3021311decfb840b`, baseline
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` and profile source
  `76e8edf394a4cb37ca822037272d543c68f25f71`.

The Oxid checkout was inspected read-only at
`183664aeca500c25d6d27a22fa402b4d40c649d3` with its pre-existing modified and
untracked entries unchanged. The Lace ID Portal checkout was inspected
read-only at `d284b85bfcbb4a7e5d2200837703419c145c60f5`, behind its remote with one
pre-existing untracked `.pi/` directory. Neither checkout is an implementation
target.

## Source findings

Oxid ADR-0101 explicitly records negative interoperability with Portal
baseline `804de0a`: extra offer query data, `tx_code: null`, combined metadata,
singular proof and singular custom Credential Response shapes are rejected.
The same ADR retains a strict OpenID4VCI Final holder profile and states that
there is no positive live Portal interoperability claim.

The Portal profile revision contains a fourteen-file Final-shaped fixture
manifest and immutable Oxid snapshots. Its protocol shapes are useful design
evidence, but the repository root and workspace package metadata contain no
license grant. The profile also selects `midnight_cbor_phase1`, and some files
exercise issuer-side proof validation rather than an SDK wallet API. Its own
manifest's `Apache-2.0` assertion is not independent legal evidence.

These facts support reference and behavioral classification, not byte copying
or a positive production interoperability claim.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Copy Portal profile fixtures | `25499870f84d77173c46e4af3021311decfb840b` | `not-adopt` | Exact source lacks an explicit license; profile and some semantics are Midnight/issuer specific. | Explicit license plus a separate generic fixture history. |
| Copy Oxid's derived Portal fixtures | `e9ecfe5df27c0790dac40776e1d0a37e5f99f907` | `not-adopt` | Oxid is licensed, but its files derive from the ambiguous Portal source and represent a negative product gate. Re-copying would not cure origin provenance. | Independent proof that every retained byte is solely Oxid-authored and generic. |
| Keep consumer evidence reference-only | exact SHAs above | `oracle` | Immutable SHAs, paths, ADRs and hashes explain the interoperability contract without entering SDK artifacts. | A licensed immutable consumer release adopts an SDK candidate and publishes conformance output. |
| Author synthetic SDK vectors from the Final contract | OpenID4VCI 1.0 Final | `adopt` | Clean-room values can exercise the generic wallet APIs, use the SDK's Apache-2.0 license and avoid donor-byte ambiguity. | An official reusable OpenID conformance suite becomes available under a compatible license. |
| Include only wallet-side inputs | SDK base `149a35e` | `adopt` | `identus-oid4vci` is a wallet-side core. Issuer request parsing and proof verification cannot be represented as local conformance without expanding scope. | A separately accepted issuer-side SDK component owns those APIs. |
| Retain legacy concepts as independently authored negative vectors | Final plus consumer oracles | `adopt` | Extra query data, null Transaction Code and singular Credential Response are standards-level incompatibility classes and can be recreated without copying bytes. | Final errata or a versioned compatibility profile changes the accepted grammar. |
| Claim live Oxid/Portal E2E or app-team approval | not available | `not-adopt` | No network, consumer build or current human review occurs; immutable design/code evidence is narrower and truthful. | A downstream adoption issue publishes exact-release live evidence and accountable review. |

## Planned vector contract

The suite will retain a small versioned directory under
`crates/oid4vci/tests/fixtures`. Positive files cover a by-value
Pre-Authorized Code offer with Transaction Code, separate Credential Issuer
and Authorization Server metadata, successful Token Response and immediate
Credential Response. Negative files cover an extra offer query member, a null
Transaction Code and a singular Credential Response. All identifiers, tokens,
codes, nonces and credentials are visibly synthetic.

One manifest records for each file: stable ID, relative path, SHA-256,
repository authorship, Apache-2.0 license, normative section, transformation,
expected result and public API entry. It separately records the immutable
consumer references and why no source bytes were imported. The integration
test validates the closed manifest before executing the inputs.

## Compatibility and dependency evidence

This is test/conformance evidence only. Public and wire compatibility are
unchanged: it changes no public Rust API, wire
serializer, error taxonomy, feature, runtime dependency, manifest, lockfile,
MSRV or target promise. Existing `serde_json` and `sha2` dependencies are
sufficient for test parsing and drift hashes. Plain Cargo consumers do not
load the fixture directory.

The direct and resolved dependency cone is unchanged. No new package,
build script, proc macro, unsafe or native code enters the graph. Supply-chain
evidence remains the existing locked workspace and source policy. The facade
boundary is the public `identus-oid4vci`/`identus-jose` API; tests do not use
private modules or expose a new facade.

The relevant public APIs already compile on Linux, WASM, iOS and Android. The
slice will run focused and workspace tests, strict Clippy/docs/fmt, factory
checks and eligible portable compile checks. Nix remains unavailable on the
host PATH but is reachable through the absolute system profile; the complete
bootstrap check and hosted fast line remain authoritative candidate evidence.

Rollback deletes the additive fixture/test/report assets and restores the
matrix/backlog status; no public compatibility, stored data or consumer
migration is involved. Protocol/draft currency is fixed to OpenID4VCI 1.0
Final; later errata or revision changes require an explicit reconsideration.

## Security, privacy and maintenance evidence

All payloads are synthetic and public. No production credentials, keys,
tokens, nonces, DIDs, hosts, PII, captures or deployment values are retained.
Secret-shaped values use explicit `SYNTHETIC_` markers and are still accessed
only through existing sensitive SDK methods inside tests. The suite introduces
no parser, unsafe code, native dependency or execution effect.

Repository-authored bytes are licensed under the SDK's Apache-2.0 license.
Consumer sources remain reference-only. The manifest checker rejects unknown
fields, missing or escaping paths, non-regular files, wrong digests, incomplete
provenance and any claim that consumer bytes were copied.

Maintenance is limited to versioned fixtures, a manifest and tests. Release
and security posture are unchanged: the suite is not shipped runtime code,
does not authorize publication and adds no vulnerable execution surface. The
objective reconsideration trigger is a compatible official vector suite, a
new Final/errata profile, a licensed generic consumer fixture release, or a
public API change that invalidates the manifest's entry-point mapping.

## Rejected or deferred candidates

Copied Portal/Oxid bytes, issuer-side parsers, live HTTP, credential-format
semantics, Midnight fixtures, consumer submodules, runtime fixture loaders,
official certification and downstream adoption are rejected or deferred for
the reasons above. No dependency candidate is conditionally introduced.

## Open questions and blockers

There is no blocker to the clean-room suite. Portal licensing remains
unresolved, but it is no longer a delivery blocker because no Portal bytes are
copied. Current human app-team approval and live product interoperability
remain downstream evidence and must not be inferred from M4.

## Evidence commands

```text
cargo test --locked -p identus-oid4vci --test generic_interoperability_vectors
cargo test --locked -p identus-oid4vci --all-features
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps
cargo fmt --all -- --check
scripts/factory check
scripts/factory backlog-live
./bootstrap.sh --check
```

During planning, `scripts/factory doctor`, OpenSpec validation, source-tree
inspection and consumer read-only Git commands ran. The implementation commands
above are unrun until the preimplementation receipt exists; their exact results
will be recorded before delivery.
