# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The current implementation is the canonical merged `identus-sd-jwt-vc`
draft-22 profile. Its property, fuzz, benchmark, portable-target, and offline
OID4VC evidence are source-bound.
The declaration `credentials.sd-jwt-vc.draft22.quality.v1` deliberately leaves
its differential class required/pending under issue #573. Its current test
compares the normative draft and SDK-TS 8.1.4 legacy behavior, but SDK-TS emits
`vc+sd-jwt` and is a migration consumer rather than an independent canonical
profile engine.

No public implementation found on 2026-10-29 advertises draft-22, which was
published only on 2026-10-27. Exact draft labels are therefore not used as a
proxy for evidence. The selected engines are frozen at exact versions and
their observed decisions are compared with the exact draft-22 rules. A passing
oracle is useful agreement; a disagreement is useful only after the normative
resolution is explicit.

## Normative sources

- `draft-ietf-oauth-sd-jwt-vc-22`, dated 2026-10-27, text SHA-256
  `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124`,
  https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-22.txt.
- Stable base RFC 9901, text SHA-256
  `fd4cb79184162fa7ef95f4f63abab8ee6d03c9a04c19b8f83758946464b284eb`.
- Official draft repository revision for the comparison:
  `66a7d2a997eda2a42f62e987bff65962e201fc85`.

The draft-19-to-22 diff is finite and operationally relevant:

| Revision | Draft-22-relevant change | Differential consequence |
| --- | --- | --- |
| 20 | media type requirement is scoped to places using a media type; `aka_vcts` participates in type matching; metadata identity is checked at each retrieval edge; mandatory paths traverse the complete conforming issuer view; issuer metadata rejects dot segments; `x5c` is policy-bound; display direction and SVG constraints are clarified | Older engines may under-check mandatory paths, retrieval identity, or type matching and may not expose the new display/rendering vocabulary. |
| 21 | malformed optional claim `display` and `svg_id` metadata no longer fails credential claim-metadata processing; only `path`, `mandatory`, and `sd` are processing-critical | Draft-19 engines can reject credentials that draft-22 requires the processing layer to tolerate. |
| 22 | registered-status wording, RFC 7800 departure, example signatures, claim-display prose, identifiers, and media registration are corrected | No additional core profile decision is introduced, but official example bytes and source identity change. |

Rendering, HTTP, X.509 path building, and trust remain outside the Rust crate.
Their delta is documented but cannot be claimed as local runtime evidence.

## Candidate decisions

| Candidate | Immutable identity | Coverage | Decision |
| --- | --- | --- | --- |
| OpenWallet Foundation `@sd-jwt/sd-jwt-vc` | npm 0.20.1; tag commit `b983e03253481ab6d33e147cc7e30e4904d0813f`; npm SHA-512 `IYalqTDCgHfh08TESuXxUGa90MTD134kAUgrJxlhtAVEPbIEvxbG4DTT7TshLV4Vr8w3fRaSpfBZQhH0y1sRag==`; package SHA-256 `95722f5a8396a8f1ca480b0506d615155f97fd2cbfe69fa71834658440f23620` | canonical issuance/verification, key binding, status-list behavior, VCT metadata retrieval, extension chains, cycles/depth and claim rules | `oracle` |
| K2GL `sd-jwt-vc` | Composer 2.1.0; tag commit `7d938daf0a1e14974420651e7ce84146ed1875bb`; archive SHA-256 `fe86f82ce0b1345f163086f9d0b09265643982f07b9ec15d08c1596bc46586c2` | protected type, required and non-selective claims, key binding, additional types, status reference, issuer-key inputs, Type Metadata, integrity, extension and claim-path rules | `oracle` |
| SDK-TS 8.1.4 | revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | released Identus migration behavior using legacy `vc+sd-jwt` | retain as `migration-oracle`; never count toward the two independent canonical engines |
| Affinidi, IOTA, Spruce, ReallyMe and other ADR 0193 candidates | exact identities retained by the engine-qualification archive | RFC 9901 or partial profile mechanics | retain prior disposition; do not duplicate the two selected profile packets |

The OpenWallet Foundation engine is Apache-2.0; its checked-out license
SHA-256 is `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`.
K2GL is MIT; its license SHA-256 is
`44afa83375fd31d2326941bf65b82fc3369b169c283cb848a0361b7a63a16f8c`.
Neither license or donor source is copied into SDK production code.

## Compatibility and dependency evidence

The OpenWallet package declares Node 20 or newer and is exercised with its
tagged pnpm lock. K2GL declares PHP 8.1 or newer and is exercised through a
generated exact Composer lock rooted only at version 2.1.0. The implementation
packet records resolved names, versions, archive integrity, tool versions,
dependency counts, upstream test outcomes, and machine-readable audit results.

The exact version and features are frozen: OpenWallet 0.20.1 uses its
SD-JWT-VC, RFC-base, status-list and Type Metadata surfaces; K2GL 2.1.0 uses
its default compact, key-binding, issuer-key, status-reference and Type
Metadata surfaces without optional live HTTP/status services. Neither project
declares a Rust MSRV because neither is a Rust dependency; the SDK MSRV and
portable target matrix remain unchanged.

The implementation receipt records the direct and resolved dependency cone
for each isolated lock, including every archive integrity. Source scans record
unsafe and native code evidence separately: foreign-language memory safety is
not inferred, native extensions are listed, and no such code enters an SDK
artifact. Supply-chain evidence includes package-manager audit output at the
recorded retrieval time.

Public and wire compatibility is behavior-neutral: existing Rust APIs and
compact output do not change. The facade boundary is the closed neutral packet
and stable result categories; no donor type or error crosses it. Rollback
restores the differential obligation to pending while retaining immutable
research provenance.

These locks and installed trees are isolated research inputs. They do not
enter `Cargo.toml`, `Cargo.lock`, Nix runtime closures, release artifacts, or
portable target builds. Advisory success proves only the frozen oracle graph
at the recorded retrieval time. A later advisory does not alter the immutable
packet; it opens visible evidence debt.

## Neutral packet and normalization

The offline packet uses SDK-authored non-secret values and stable result
categories rather than foreign DTOs, errors, signatures, timestamps, or
serialization order. It covers:

- exact `dc+sd-jwt` protected/media identity and legacy rejection;
- required collision-resistant `vct`, `aka_vcts` matching, and malformed
  additional types;
- non-selective `iss`, `nbf`, `exp`, `cnf`, `vct`, `vct#integrity`,
  `aka_vcts`, and `status` placement;
- holder-confirmation presence and key-binding verification stage;
- Type Metadata identity, integrity, extension depth/cycles, mandatory and
  selective-disclosure rules, including draft-20/21 delta cases;
- status-reference shape and correlation without status acceptance policy.

An oracle can report `accept`, `reject`, or `not-exposed`. `not-exposed` is not
agreement. Every `accept`/`reject` disagreement is resolved to `draft22_accept`,
`draft22_reject`, or `out_of_sdk_scope` with an exact draft section and source
selector. The Rust test checks the normative result, never majority voting.

## Security, privacy and maintenance evidence

Packets contain no real credential, issuer, holder, audience, nonce, key,
status handle, or network response. Candidate diagnostics are reduced to
stable categories before check-in. The runtime test reads bounded regular JSON
and does not execute JavaScript, PHP, donor source, HTTP, or package-manager
hooks. Reproduction scripts remain explicit research tooling and are not a
required consumer or production dependency.

Draft evolution, candidate updates, package compromise, or a discovered
normalization ambiguity opens a new issue and packet version. Existing packet
bytes remain immutable. Maintenance and release posture stays conservative:
publication remains blocked until this evidence and the ordinary security and
slow lanes pass.

## Rejected or deferred candidates

- Counting the IETF text or SDK-TS as one of two independent engines: they are
  normative and migration inputs respectively, not independent canonical
  implementations.
- Requiring both older engines to claim draft-22: no such releases exist and
  a version label would still not prove behavior.
- Importing either engine into production or dev dependencies: this expands
  coupling and supply-chain risk without improving offline evidence.
- Majority voting between implementations: two implementations can share the
  same bug; draft-22 remains the authority.
- Waiting indefinitely for new releases: the finite delta can be qualified
  now and refreshed later without weakening the profile.

The reconsideration trigger is a maintained exact draft-22 or final-RFC
implementation that exposes the complete neutral surface with a smaller,
cleaner reproducible probe. Such a release may replace an oracle in a new
packet version but cannot rewrite historical evidence.

## Open questions and blockers

No planning blocker remains. Implementation must not mark the differential
obligation satisfied until both upstream suites, frozen receipts, the neutral
packet, the SDK route, the disagreement ledger, and quality-plan regeneration
all pass. Any missing surface remains explicit `not-exposed` evidence rather
than an inferred success.

## Evidence commands

Planning uses `scripts/factory research-ready`, `constraints-ready`, OpenSpec
validation, a signed/DCO planning commit, and issue #573 preflight. Delivery
uses the frozen upstream test/audit probes, the focused differential test,
vector/catalog and quality-plan checks, workspace/factory/Nix gates, and
Claude CLI review or immediate Codex CLI fallback.

Exact commands are recorded in the packet receipts. Unrun checks at planning
time are both upstream suites/audits, packet generation, focused Rust tests,
workspace and Nix gates, independent review, hosted CI, archive, merge, and
metrics publication; none is inferred from research readiness.
