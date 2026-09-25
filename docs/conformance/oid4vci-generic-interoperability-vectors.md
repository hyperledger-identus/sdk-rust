# Generic OpenID4VCI interoperability-vector assessment

**SDK base:** `149a35e62fc54f91dc99a496feb884d5b2f92cce`

**Suite:** `crates/oid4vci/tests/fixtures/interop-v1/manifest.json`

**Normative baseline:** OpenID for Verifiable Credential Issuance 1.0 Final,
published 2025-09-16 and retrieved 2026-09-25.

## Decision

The SDK uses independently authored Apache-2.0 fixtures and imports no Oxid or
Lace ID Portal bytes. This resolves the generic wallet-core evidence gap
without treating an ambiguous license, a Midnight credential format or an
issuer-side responsibility as an SDK contract.

The suite proves bounded structural expressibility through public APIs. It is
not a byte-parity claim, live HTTP result, current human app-team approval,
credential-format verification, official certification or production
interoperability result.

## Immutable consumer evidence

| Source | Immutable evidence | License disposition | SDK disposition |
| --- | --- | --- | --- |
| MediaNoxLabs/oxid | `e9ecfe5df27c0790dac40776e1d0a37e5f99f907`; `docs/adr/0101-gate-laceid-portal-interoperability-on-final-openid4vci.md`; `crates/adapters/openid4vci/src/laceid_portal_contract_tests.rs` | Apache-2.0 at the pinned revision | Reference-only strict Final holder and legacy-rejection oracle |
| input-output-hk/lace-id-portal | profile `25499870f84d77173c46e4af3021311decfb840b`; baseline `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`; profile source `76e8edf394a4cb37ca822037272d543c68f25f71` | No explicit repository license at the pinned revisions | Reference-only Final-shaped issuer/profile oracle; no bytes copied |

The Portal profile manifest's own `Apache-2.0` field is not treated as an
independent license grant. Oxid's Apache-2.0 wrapper also does not cure the
origin question for Portal-derived fixture bytes.

## Behavior mapping

| Consumer behavior | Generic SDK evidence | Result |
| --- | --- | --- |
| Final by-value Credential Offer with Pre-Authorized Code and Transaction Code | `CredentialOfferRequest`, semantic/grant validation and the positive offer fixture | Expressible and accepted |
| Separate Credential Issuer and Authorization Server metadata | Public metadata parsers and exact issuer/server matching | Expressible and accepted |
| Deterministic Pre-Authorized Code Token Request and bounded request-bound Token Response | Public request builder/binder plus exact synthetic form/response fixtures | Expressible and accepted |
| JWT-proof Credential Request and immediate `credentials` response | Public JOSE builder, Credential Request builder and immediate response binder | Expressible and accepted structurally |
| Extra offer query member | Independently authored negative fixture | Deliberately rejected as unsupported transport |
| `tx_code: null` | Independently authored negative fixture | Deliberately rejected during grant validation |
| Singular custom Credential Response | Independently authored negative fixture | Deliberately rejected by the immediate response boundary |
| Singular issuer-side proof request and JSON Token Request | No wallet-side parser is owned by this crate | Outside the M4 wallet-core boundary |
| Origin reflection, replay/single-use, proof verification and live transport | Consumer policy/issuer/transport concerns | Downstream; not evidenced by the suite |
| `midnight_cbor_phase1` credential semantics | Chain-profile behavior | Remains downstream and absent from executable fixtures |

## Executable packet

The closed manifest covers nine fixture files: six positive inputs/expected
wire values and three negative inputs. For every vector it records a stable ID,
path, SHA-256, normative section, repository authorship, Apache-2.0 license,
transformation, expected result and public API. The test rejects unknown
manifest fields, incomplete provenance, duplicate IDs/paths, unsafe paths,
symlinks, extra files and digest drift before executing the flow.

The coherent positive journey validates offer transport and grants, issuer and
server metadata, Transaction Code ownership, the exact Token Request form,
request-bound Token Response lineage, JWT Credential Request construction and
an immediate Credential Response. All codes, tokens, nonces, credentials,
hosts and signatures are synthetic.

## Read-only consumer receipt

- Oxid was inspected at
  `183664aeca500c25d6d27a22fa402b4d40c649d3` on `develop` with four
  pre-existing modified/untracked status entries.
- Lace ID Portal was inspected at
  `d284b85bfcbb4a7e5d2200837703419c145c60f5` on `develop`, behind its remote,
  with one pre-existing untracked `.pi/` directory.
- No consumer file, branch, index, worktree or remote was changed.

## M4 disposition

The cross-consumer matrix row can be `implemented` from clean-room executable
evidence. This closes the last required missing row and permits IDR-023,
issue #7 and M4 to close as a bounded wallet-core milestone. The five partial and
four unsupported rows remain visible; publication, certification and
downstream adoption remain separate.
