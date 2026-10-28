# SD-JWT VC draft-22 differential qualification

**Decision date:** 2026-10-29  
**Owner:** issue #573  
**Normative authority:** `draft-ietf-oauth-sd-jwt-vc-22`, text SHA-256
`da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124`

## Decision

`identus-sd-jwt-vc` has two independent, immutable behavioral oracles for its
draft-22 profile. They are research inputs, not production or CI dependencies.
Normal CI reads a bounded offline packet and executes every normative result
through the public Rust API. Draft-22, not implementation majority, resolves
all disagreements.

The crate remains unpublished. The packet does not add HTTP, rendering, trust,
status-acceptance, legacy-input, or donor-runtime behavior.

## Frozen engines

| Engine | Immutable source | Isolated evidence | Supply-chain result |
| --- | --- | --- | --- |
| OpenWallet Foundation `@sd-jwt/sd-jwt-vc` 0.20.1 | commit `b983e03253481ab6d33e147cc7e30e4904d0813f`; npm tarball SHA-256 `95722f5a8396a8f1ca480b0506d615155f97fd2cbfe69fa71834658440f23620` | Node 22.23.4, pnpm 9.4.0; 20 node and 17 E2E tests passed | 25 production dependencies; one high advisory in isolated `zod` 4.4.3; browser suite did not qualify because the upstream jsdom harness mixes realms and imports `node:test` |
| K2GL `sd-jwt-vc` 2.1.0 | commit `7d938daf0a1e14974420651e7ce84146ed1875bb`; archive SHA-256 `fe86f82ce0b1345f163086f9d0b09265643982f07b9ec15d08c1596bc46586c2` | PHP 8.3.36, Composer 2.10.3; 128 tests and 296 assertions passed | 44 resolved packages; zero advisories and zero abandoned packages |

Exact license, lock, audit and toolchain receipts are in
[`oracle-receipts.json`](../../../crates/sd-jwt-vc/tests/fixtures/differential-v1/oracle-receipts.json).
The OpenWallet advisory and browser-harness failure are reasons to keep the
engine isolated; neither invalidates its passing Node behavior as an oracle.

## Normalization

The packet admits only `accept`, `reject`, and `not-exposed` observations.
Donor DTOs, error text, compact bytes, signatures, timestamps, network
responses and serialization order are intentionally absent. Every observation
has an immutable-revision source selector and a bounded note code. The Rust
test validates the closed schema, provenance receipts, oracle cardinality and
normative result.

## Agreement and disagreement ledger

| Surface | Draft-22 result | OpenWallet 0.20.1 | K2GL 2.1.0 | Resolution |
| --- | --- | --- | --- | --- |
| canonical `dc+sd-jwt` | accept | accept | accept | agreement |
| legacy `vc+sd-jwt` by default | reject | not exposed as a negative | reject | draft-22 reject; compatibility remains issue #572 only |
| required `vct` | reject when missing | compile-time shape only | reject | draft-22 reject |
| `aka_vcts` match and shape | accept valid; reject malformed | not exposed | accept/reject | draft-22 governs; Rust covers both |
| nested protected claim disclosure | reject | root-frame check only | reject | draft-22 reject; Rust covers nested status |
| required holder confirmation | accept only with valid `cnf` at this policy stage | accept/reject | separate policy surface | Rust keeps policy explicit |
| finite acyclic metadata extension | accept valid; reject cycle/depth | accept/reject | accept/reject | agreement |
| missing mandatory cleartext (`sd=never`) path | reject | mandatory not enforced | **accept** because the verifier explicitly skips `mandatory` | draft-20/22 complete-payload rule wins where absence is observable |
| malformed optional claim `display`/`svg_id` | **ignore for credential processing** | reject in metadata schema | reject in metadata schema | draft-21/22 wins; Rust imports only processing-critical path/mandatory/sd |
| valid status reference shape | accept as evidence, not policy | accept and performs optional network policy | accept/expose | Rust stops before retrieval or acceptance |
| malformed status reference | reject | not exposed by cited test | not exposed by cited test | draft-22 reject |

`not-exposed` is never counted as agreement. Rendering and SVG safety, live
metadata retrieval, certificate path validation, trust, status-list retrieval
and credential acceptance remain outside this crate and outside the claim.

## Draft-19 to draft-22 delta

- Draft-20 made `aka_vcts` part of type matching, tightened metadata identity
  at retrieval edges, defined mandatory-path processing over the complete
  payload, and clarified issuer metadata, X.509 policy, display and SVG rules.
- Draft-21 made malformed optional claim `display` and `svg_id` non-critical to
  credential claim-metadata processing. `path`, `mandatory`, and `sd` remain
  processing-critical.
- Draft-22 mainly corrected status wording, examples, identifiers and media
  registration; it did not add another local core-profile decision.

## Reproduction and maintenance

The immutable packet is
[`packet.json`](../../../crates/sd-jwt-vc/tests/fixtures/differential-v1/packet.json).
Run:

```console
cargo test -p identus-sd-jwt-vc --test differential
```

A new draft, final RFC, oracle release, advisory, normalization ambiguity, or
newly exposed surface creates a new issue and packet version. Historical
packet bytes and receipts are not rewritten. A future maintained exact
draft-22/final-RFC engine may replace an oracle only in a new packet.
