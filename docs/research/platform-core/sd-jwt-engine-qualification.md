# SD-JWT engine qualification

**Decision date:** 2026-10-28

**Owner:** issue [#489](https://github.com/hyperledger-identus/sdk-rust/issues/489)

**Implementation:** [#568](https://github.com/hyperledger-identus/sdk-rust/issues/568), then [#569](https://github.com/hyperledger-identus/sdk-rust/issues/569)

**Decision:** [ADR 0193](../../adr/0193-build-a-focused-rfc9901-engine.md)

## Outcome

SDK-Rust will build a focused `identus-sd-jwt` engine over its existing JOSE,
cryptography, credential, and presentation seams. No assessed Rust release is
adoptable for the 0.1.x line without weakening the Rust floor, dependency,
resource, diagnostic, target, or architectural constraints. Candidate crates
remain immutable differential oracles and their types do not enter public SDK
contracts.

The implementation is deliberately split:

1. RFC 9901 base mechanics and key binding in #568;
2. an independently versioned SD-JWT VC profile pinned to draft-22 in #569;
3. an optional `vc+sd-jwt` migration adapter only if a named consumer proves
   the need. Canonical issuance uses `dc+sd-jwt`.

## Standards pins

| Capability | Immutable source | Retrieved | SHA-256 | Status |
| --- | --- | --- | --- | --- |
| SD-JWT base | [RFC 9901 text](https://www.rfc-editor.org/rfc/rfc9901.txt) | 2026-10-28 | `fd4cb79184162fa7ef95f4f63abab8ee6d03c9a04c19b8f83758946464b284eb` | stable normative base |
| SD-JWT VC | [draft-ietf-oauth-sd-jwt-vc-22](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt) | 2026-10-28 | `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124` | exact evolving profile |
| Superseded planning input | [draft-ietf-oauth-sd-jwt-vc-19](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-19.txt) | 2026-10-28 | `394c4640c581c301adaf24d1d356238e2c290b5c9e8b130141ca0e28732144fb` | research evidence only; do not implement |

Draft-22 was published on 2026-10-27 while this research was active. This is
why the base RFC and VC profile are separate capabilities and why a profile
upgrade requires a visible issue, vector packet, and compatibility decision.

## Candidate matrix

| Candidate | Exact assessment | Disposition | Principal reason |
| --- | --- | --- | --- |
| Affinidi `affinidi-sd-jwt` | 0.1.8 / `95fde42bd338` | oracle | closest complete RFC 9901 engine, but Rust 1.95 exceeds the 0.1.x floor and complete bounds/redaction are missing |
| IOTA `sd-jwt-payload` | 0.5.1 / `c47e52c31545` | oracle | useful payload mechanics, but not an envelope, JOSE, key-binding, or policy engine |
| Spruce `ssi-sd-jwt` | 0.6.0 / `16cd58715aa2` | reject | 218-package SSI graph, authored unsafe, and exact-lock RustSec failures |
| ReallyMe `reallyme-sd-jwt` | 0.4.1 / `60f42c536d6e` | oracle | strong bounded design, but Rust 1.99 and private JOSE/crypto/proto stack |
| Blockhouse `bh-sd-jwt` | 0.7.1 / `7314e3e002ca` | reject | AGPL-3.0 plus HTTP, X.509, native crypto, and broad status coupling |
| Kushal `sd_jwt` | 0.1.0 / `889bba1c763d` | reject | OpenSSL-backed JOSE, full-value diagnostics, and no bounded target contract |
| `sdjwt` | 0.8.1 / `c671b9eaaef0` | reject | stale, broad private JWT/RSA/YAML/WASM stack, and no RFC 9901 evidence |
| Equs `equs-sd-jwt-rs` | 0.1.1 / `a3e4e621db10` | reject | explicit draft-07 fork |
| AeroCred `aerocred-sdjwt` | 0.1.0 / `349bc49e2d40` | oracle | small useful example, but narrow ES256-only journey and incomplete conformance/bounds evidence |

The exact probe manifests and locks, dependency counts, checksums, MSRV
probes, source scans, advisory results, upstream test results, commands, and
objective reconsideration triggers are preserved in the archived OpenSpec
research record for `qualify-sd-jwt-engine`.

## Architecture boundary

```text
injected salt/time/signature/digest capabilities
                       │
                       ▼
 bounded RFC 9901 parse → disclosure graph → selection → key binding → receipt
            │                  │              │               │
            └──────── Identus-owned staged values and errors ─┘
                                      │
                       draft-22 VC profile (separate capability)
                                      │
                  OID4VCI / OID4VP format capability ports
```

The engine owns deterministic format processing. It does not own key custody,
issuer trust, verifier authorization, replay policy, storage, HTTP, consent,
or UI. Inputs and retained outputs are bounded; errors and `Debug` output are
static and correlation-safe; unverified values cannot be used as verified
values through the public API.

## Executable evidence contract

Production delivery is incomplete until #568 and #569 provide:

- RFC 9901 examples and draft-22 Appendix A packets with exact provenance;
- SDK-authored negative cases for malformed compact forms, disclosure
  ambiguity/collision, unused disclosures, recursive graphs, reserved names,
  unsupported hashes, key-binding mismatch, and every resource boundary;
- SDK-TS 8.1.4 compatibility fixtures only for the optional migration layer;
- property tests for parse/serialize and staged-state invariants;
- fuzz targets for compact ingress, disclosures, graphs, and selection;
- boundary/adversarial benchmarks with explicit work budgets;
- at least two immutable independent differential oracles; and
- an offline E2E that issues with injected effects, stores a bounded generic
  envelope, selects disclosures, builds key binding, verifies issuer/holder
  evidence, and emits a redacted receipt through OID4VC capability ports.

| Evidence class | #568 RFC 9901 base | #569 draft-22 profile | Completion rule |
| --- | --- | --- | --- |
| official/provenance | RFC 9901 examples and applicable Appendix material | exact draft-22 Appendix A packet | immutable source, retrieval date, content hash, and license/provenance record |
| clean-room negative | compact, disclosure, graph, hash, key-binding, and every limit boundary | claim, type/metadata, status-input, and base/profile boundary | every documented rejection and exact boundary has a stable vector/result |
| property | parse/serialize, disclosure closure, selection, staged states | claim/type/profile invariants | stable quality declaration and reproducible passing route |
| fuzz | compact ingress, disclosures, graphs, selection, key binding | claims, metadata, profile boundary | time-bounded sanitizer route with corpus/seed/crash retention contract |
| benchmark | normal, exact-limit, adversarial graph/work cases | type/metadata/profile validation | checked baseline and explicit regression budget; no unmeasured performance claim |
| differential | at least two immutable independent RFC 9901 engines | at least two current-profile counterparts where available | disagreement ledger resolves behavior against the pinned standard/profile |
| cross-language | migration inputs only | canonical `dc+sd-jwt` plus optional legacy mapping | SDK-TS behavior is labeled consumer evidence and never silently changes Rust semantics |
| offline E2E | issue, store envelope, select, key-bind, verify, redact | route the same journey through OID4VCI/OID4VP profile ports | deterministic receipt contains no secret, undisclosed claim, compact credential, audience, or nonce |

Issues #568 and #569 create the stable quality declarations when executable
routes exist. This research record does not predeclare passing evidence.

Portable compile and runtime evidence remains required. The research host did
not have the WASM, iOS, or Android rustup components and target installation
did not complete, so no candidate receives a portable-support claim.

## SDK-TS compatibility

SDK-TS 8.1.4 at `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
is a consumer-regression source, not normative authority. Its TypeScript DTOs,
errors, task runtime, DID coupling, `vc+sd-jwt` identifier, and millisecond-time
tolerance remain adapter concerns. Rust domain types and errors stay idiomatic
and may be mapped by a thin SDK-TS compatibility layer.
