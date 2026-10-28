# SD-JWT engine research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation has no SD-JWT proof engine. `identus-credentials`
owns a bounded format-neutral envelope, `identus-presentations` owns generic
selection/lifecycle states, `identus-jose` owns bounded JWS and injected
signature capabilities, and OID4VCI/OID4VP route the canonical `dc+sd-jwt`
format without interpreting its proof. This is a real implementation gap, not
permission to import another SDK's DTOs, errors, policy, or framework.

SDK-TS 8.1.4 at immutable revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e` is consumer-regression evidence.
It depends on `@sd-jwt/sd-jwt-vc@^0.7.1`, exposes `vc+sd-jwt`, tolerates
millisecond JWT timestamps, binds verification directly to DID resolution, and
uses TypeScript-specific tasks/models. Those choices are migration inputs, not
normative Rust behavior. The locally available SDK-TS checkout was inspected
read-only at `b5d0ec99515b9433c9a082ab707d75a6041379f2`; it was already dirty and
was not modified.

## Normative sources

| Capability | Immutable source | SHA-256 of retrieved text | Disposition |
| --- | --- | --- | --- |
| SD-JWT base | [RFC 9901](https://www.rfc-editor.org/rfc/rfc9901.txt), November 2025 | `fd4cb79184162fa7ef95f4f63abab8ee6d03c9a04c19b8f83758946464b284eb` | stable normative base |
| SD-JWT VC | [`draft-ietf-oauth-sd-jwt-vc-22`](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt), 2026-10-27 | `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124` | exact independently versioned profile; active I-D in IESG evaluation |
| Prior planning pin | [`draft-ietf-oauth-sd-jwt-vc-19`](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-19.txt), 2026-08-31 | `394c4640c581c301adaf24d1d356238e2c290b5c9e8b130141ca0e28732144fb` | superseded research evidence; do not implement |

Draft-22 retains `application/dc+sd-jwt` and protected `typ=dc+sd-jwt`, changes
type matching and metadata guidance after draft-19, and remains capable of
further IESG change. RFC 9901 compact serialization is mandatory for the first
base slice; JWS JSON Serialization is a later independently bounded capability.

## Candidate decisions

| Candidate | Exact version/revision/checksum | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Focused Identus base engine | repository-local over `identus-jose`, `identus-crypto`, and injected ports | `retain-local` | No release clears every current gate; existing SDK seams already own bounded JOSE, crypto, staged evidence, redaction, and target policy. | Replace private mechanics only when an immutable candidate proves lower total risk and exact parity without public migration. |
| Affinidi `affinidi-sd-jwt` | 0.1.8, `95fde42bd338`, `b44d072f…ccb1cb` | `oracle` | Closest cohesive RFC 9901 base, pluggable signer/verifier, active tests; declared Rust 1.95 violates the 0.1.x MSRV, inputs/collections remain broadly unbounded, and Debug/Display/errors can expose complete credentials, digests, audience, or nonce. | A release supports Rust 1.89, adds caller-owned limits/redaction, and passes the complete SDK differential/target matrix. |
| IOTA `sd-jwt-payload` | 0.5.1, `c47e52c31545`, `3d0271ae…b195cd` | `oracle` | Useful payload/disclosure oracle with Rust 1.89 host compile, but it is only payload mechanics and its minimal cone still brings 46 registry packages plus `anyhow`, async traits, RNG, multibase, JSON pointer, and open models. It does not replace bounded envelope, JOSE, key binding, or policy work. | A narrower feature surface demonstrates meaningful correctness/maintenance deletion and full rejection/bounds parity. |
| Spruce `ssi-sd-jwt` | 0.6.0, `16cd58715aa2`, `0eadfb39…ba6ed` | `not-adopt` | Host/MSRV compile passes, but the minimal probe resolves 218 registry packages, imports the SSI claims/JWK/JWS/JWT stack, and contains authored unsafe constructors. Protocol currency and SDK rejection/resource parity are not explicit. | A narrow safe crate with RFC 9901 conformance, bounded inputs, small cone, and exact target evidence is released. |
| ReallyMe `reallyme-sd-jwt` | 0.4.1, `60f42c536d6e`, `0d2f5508…c46c63d` | `oracle` | Strongest visible bounds, staged APIs, zeroization, and RFC 9901 surface; declared Rust 1.99 fails the SDK's Rust 1.98.1 primary compiler and brings 55 registry packages through a private ReallyMe JOSE/crypto/proto stack. | A compatible release line, independent maintenance evidence, and candidate-free integration prove lower risk than the local focused engine. |
| Blockhouse `bh-sd-jwt` | 0.7.1, `7314e3e002ca`, `78ef8f71…48e75a` | `not-adopt` | SD-JWT VC draft-era coverage is coupled to AGPL-3.0, HTTP, OpenSSL, AWS-LC, ring, X.509, status, and 261 registry packages; it is not a chain-neutral focused engine. | A permissively licensed, transport-free, native-free core is released and passes current profile gates. |
| Kushal Das `sd_jwt` | 0.1.0, `889bba1c763d` | `not-adopt` | Claims RFC 9901 but couples all roles to `josekit`/OpenSSL, exposes full credential Display/Debug, and lacks explicit bounds, MSRV, portable-target, and maintenance evidence. | A maintained narrow release removes native JOSE and passes bounds/redaction/conformance gates. |
| `sdjwt` | 0.8.1, `c671b9eaaef0` | `not-adopt` | Last release is from 2024, has no declared MSRV, and couples RSA, WASM bindings, YAML, and its own JWT stack without RFC 9901/current-profile evidence. | A maintained RFC 9901 release with a focused feature cone and current vectors appears. |
| Equs `equs-sd-jwt-rs` | 0.1.1, `a3e4e621db10` | `not-adopt` | Explicitly remains the draft-07 OWF fork with `jsonwebtoken`, async/runtime-oriented surfaces, and legacy semantics. | It implements RFC 9901/current profile and satisfies ADR 0076's existing reopen trigger. |
| AeroCred `aerocred-sdjwt` | 0.1.0, `349bc49e2d40` | `oracle` | Small Apache-2.0 Rust 1.85 source with redacted key Debug, but hard-wires ES256/`p256`, implements only a narrow VC journey, and has no complete RFC 9901/profile, bounds, maintenance, or target evidence. | A generic current-profile release proves complete negative/vector parity without hard-wired product policy. |

## Compatibility and dependency evidence

All probes were outside the SDK worktree, used exact versions and generated
separate locks, and changed no root manifest, lock, API, wire value, or consumer.
The direct and resolved dependency cone was measured for each exact isolated
manifest. Exact minimal probe results were:

| Candidate | Direct production dependencies | Resolved registry packages | Build scripts | Native `links` packages | Rust 1.98.1 host | Rust 1.89 host |
| --- | ---: | ---: | ---: | --- | --- | --- |
| Affinidi 0.1.8 | 7 | 36 | 9 | none | pass | fail: declares 1.95 |
| IOTA payload 0.5.1, no defaults | 11 | 46 | 11 | none | pass | pass |
| IOTA payload 0.5.1, defaults | 12 including `iota-crypto` | 56 | 13 | none | not needed after minimal result | not needed after minimal result |
| Spruce 0.6.0 | 12 | 218 | 28 | `wasm-bindgen` metadata link | pass | pass |
| ReallyMe 0.4.1, no defaults | 7 | 55 | 13 | `defmt` metadata link | fail: declares 1.99 | not run; cannot improve primary failure |
| Blockhouse 0.7.1 | 20 | 261 | 44 | OpenSSL, AWS-LC, ring, wasm-bindgen | not run; rejected by license/coupling | not run |

Portable target compilation was attempted but the local rustup toolchain lacked
`wasm32-unknown-unknown`, `aarch64-apple-ios`, and
`aarch64-linux-android`; target installation did not complete. Those checks are
unrun, every candidate target remains unsupported for adoption, and no runtime
claim is inferred. The future local SDK implementation instead enters the
repository's existing compile matrix and remains covered by `SDK-LIM-003` until
runtime evidence exists.

Public and wire compatibility is unchanged. Future dependency replacement is
cheap because Identus owns the facade and candidate types never cross it.
Rollback of this research is documentation-only. The eventual base engine uses
a new standards name, not Apollo, Pollux, or another historical component name.

## Security, privacy and maintenance evidence

The candidate scan covered compact/JWS-JSON scope, disclosure parsing and
selection, duplicate/unused disclosure handling, recursive claim graphs,
reserved names, decoys, key binding (`sd_hash`, audience, nonce, issued-at),
JOSE ownership, verifier policy, limits, errors, Debug/Display, secret-bearing
inputs, unsafe/native code, release activity, and source integrity.

Affinidi is actively maintained and closest to a reusable base, but its public
model derives full Debug and implements full Display; error strings interpolate
remote or correlating values. Its depth-128 guard is useful but does not bound
complete bytes, disclosures, members, nodes, retained output, or signing work.
Spruce directly uses unsafe view constructors and has a broad SSI cone.
ReallyMe demonstrates that a bounded staged design is practical but cannot
compile on the primary SDK compiler. Blockhouse is disqualified independently
by AGPL and native/network coupling. Direct source scans found no unsafe/native
tokens in Affinidi, IOTA payload, ReallyMe, Blockhouse, Equs, or AeroCred; this
does not prove their transitive closures safe. Spruce contains 19 direct
`unsafe` source matches.

Supply-chain evidence uses crates.io checksums, `.cargo_vcs_info.json`, exact
locks, Cargo metadata, source scans, and `cargo-audit 0.22.2` against RustSec
revision `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`. Affinidi, IOTA payload,
and ReallyMe scans passed with warnings denied. Spruce failed on
RUSTSEC-2023-0126 and RUSTSEC-2026-0255 unsoundness plus six unmaintained
warnings; Blockhouse failed on two unmaintained warnings. The initially
available `cargo-audit 0.20.1` could not parse current CVSS 4.0, so its output
was discarded. Yank state is separately unavailable under `SDK-LIM-010`; no
clean advisory result is represented as authoritative yank evidence.

Exact upstream self-tests passed for Affinidi (106 tests with its required
private `_test-utils` feature), IOTA payload (35 unit/integration tests plus one
doctest), and Spruce (17 tests). Affinidi's plain published-source `cargo test`
fails because its integration tests import a helper gated behind
`_test-utils`; the explicit feature passes. These results are useful behavior
evidence, not SDK conformance or adoption authority.

## Vector, quality, and end-to-end plan

- RFC 9901 examples and Appendix material are normative-derived vectors with
  IETF provenance and Revised BSD handling for extracted code components.
- Draft-22 Appendix A is a profile-specific packet, never mixed into the base
  packet or silently replaced by a later draft.
- SDK-authored negative vectors cover malformed compact segments/base64/JSON,
  duplicate digests, unused disclosures, collisions, reserved names, recursive
  cycles/depth/work, unsupported hashes, disclosure ambiguity, decoys, key-
  binding type/hash/audience/nonce/time mismatch, and every byte/count limit.
- SDK-TS 8.1.4 fixtures are donor compatibility evidence for the optional
  legacy adapter; `vc+sd-jwt` and millisecond time tolerance are not canonical.
- At least two independent Rust/JavaScript engines act as differential oracles;
  disagreements are resolved against pinned standards, not majority behavior.
- Property tests cover exact parse/serialize preservation, disclosure closure,
  order independence where normative, and staged-state invariants. Fuzzing
  covers compact ingress, disclosures, claim graphs, and selection. Benchmarks
  measure boundary-sized and adversarial graphs under explicit work budgets.
- The offline E2E packet issues through injected signer/salt/time ports, stores
  a bounded credential envelope, selects claims through the presentation seam,
  constructs exact key binding, verifies through injected issuer/holder keys,
  and returns a redacted receipt. It performs no HTTP, trust, custody, or UI.

The production issues will add stable quality declarations; this research does
not invent passing evidence before executable code exists.

## Rejected or deferred candidates

Every candidate has an explicit decision and objective reconsideration trigger
in the table. Full framework adoption, profile/base conflation, native JOSE,
candidate public types, draft-07 compatibility, legacy canonical output, and an
MSRV exception inside 0.1.x are rejected. Affinidi and ReallyMe remain useful
oracles; neither is conditionally adopted.

## Open questions and blockers

There is no research-readiness blocker. Issue #489 and the A3 roadmap authorize
separate RFC 9901 and current-profile capabilities behind Identus-owned types.
The implementation issues own executable limits/API details and the draft-22
profile migration trigger; those are bounded delivery work, not an unresolved
product choice in this qualification.

## Evidence commands

The exact isolated manifests, resolved locks, integrity hashes, commands, and
recorded results are checked in under [`evidence/`](evidence/README.md). The
following additional source and standards commands were executed on 2026-10-28:

```text
scripts/factory doctor
gh issue view 489 -R hyperledger-identus/sdk-rust --json number,title,state,body,labels,milestone,assignees,url
cargo search sd-jwt --limit 25
curl -fsSL https://www.rfc-editor.org/rfc/rfc9901.txt | shasum -a 256
curl -fsSL https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-19.txt | shasum -a 256
curl -fsSL https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt | shasum -a 256
diff -u /tmp/draft-ietf-oauth-sd-jwt-vc-19.txt /tmp/draft-ietf-oauth-sd-jwt-vc-22.txt
git -C /Users/ysh/iohk/sdk-ts show 4bf86ebf:packages/lib/sdk/src/pollux/models/SDJWTVerifiableCredential.ts
git -C /Users/ysh/iohk/sdk-ts show 4bf86ebf:packages/lib/sdk/src/pollux/utils/jwt/SDJWT.ts
git -C /Users/ysh/iohk/sdk-ts show 4bf86ebf:packages/lib/sdk/package.json
```

Additional SDK-TS tests and OEA plugin paths are listed in the capability
inventory. Exact package versions, checksums, VCS revisions, and locks are
preserved in this record. No generated package source or dirty donor checkout
is copied into the SDK.

Unrun or incomplete checks are portable-target compile because target component
installation did not complete, browser/mobile runtime and packaging, deployed
interoperability, fuzz/property/benchmark/E2E code, publication, release,
certification, downstream adoption, and authoritative registry yank state.
No success is inferred for any unrun evidence.
