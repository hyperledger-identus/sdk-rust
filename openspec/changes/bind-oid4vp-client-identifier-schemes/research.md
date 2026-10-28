# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation verifies one caller-selected key, correlates the
full signed client identifier, validates audience/time/replay, and then exposes
typed routing. It does not retain the exact verification key in the evidence
lineage or authorize that key according to the client identifier prefix.
`client_metadata` is interpreted only for encrypted-response recipient
selection, and `verifier_info` is not modeled. Oxid and Lace ID Portal are the
named consumers; both need a portable core while retaining product trust and
network policy outside the SDK.

## Normative sources

- OpenID for Verifiable Presentations 1.0 Final sections 5.9, 5.11, 11, 12,
  and 14.8: https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
- W3C DID Core and DID Resolution define DID syntax and abstract resolution;
  `identus-did` already owns those generic types and ports.
- RFC 7515 defines `kid` and `x5c`; RFC 5280 defines certificate/SAN semantics.

Final is normative. SDK-TS, SDK-Swift, SDK-KMP, Oxid, and Lace ID Portal are
interoperability evidence only and do not dictate Rust DTOs or errors.

## Final scheme dispositions

| Prefix | Signed-JAR disposition | SDK responsibility | Adapter responsibility |
| --- | --- | --- | --- |
| no prefix | supported | exact full-id and verified-key evidence | pre-registration and metadata trust |
| `redirect_uri` | unsupported here | classify and fail closed | use a separately specified unsigned flow |
| `openid_federation` | supported by port | bounded trust-chain and metadata-source rules | Federation validation, policies, key and metadata |
| `decentralized_identifier` | supported by port | DID syntax, required `kid`, exact-key evidence | DID resolution, relationship and key authorization |
| `verifier_attestation` | supported by port | bounded nested `jwt` evidence and exact subject input | attestation signature, issuer trust, `cnf`, redirects |
| `x509_san_dns` | supported by port | bounded DNS identifier, `x5c`, exact-key evidence | chain, SAN, DNS and redirect policy |
| `x509_hash` | supported by port | canonical SHA-256 hash syntax, `x5c`, exact-key evidence | certificate hash, chain and key authorization |
| `origin` | unsupported here | classify and fail closed | reserved for the future DC API profile |
| unknown prefix | unsupported by default | classify first colon and fail closed | extension registration requires a future ADR |

The full prefixed identifier is retained everywhere, including audience-facing
state. A successful port decision is authorization evidence for the supplied
exact key and bounded request only; it is not consent or product authorization.

## Candidate decisions

| Candidate | Revision/version | Decision | Reason |
| --- | --- | --- | --- |
| protocol-local typed state and port | repository base `f0edaf820b3bc12b45faae02b60c55db3d3040ff` | `adopt` | Encodes Final state lineage without exporting foreign DTOs. |
| `identus-jose` protected-header/key types | workspace package | `adopt` | Reuses the exact key and bounded `kid`/`x5c`; add only protocol-neutral bounded `jwt` evidence. |
| `identus-did` syntax/resolution ports | workspace package | `adopt` | Avoids duplicate DID grammar; OID4VP does not own resolution. |
| OID4VCI trust-port pattern | workspace source reference | `oracle` | Proven async fail-closed port design, but protocol types stay separate. |
| `x509-parser`, `webpki`, Federation crates | current registry releases | `not-adopt` | The core needs policy ports, not a bundled trust engine or native root policy. |
| generic OpenID client crates | current releases | `not-adopt` | Do not implement Final scheme evidence with the required bounded state transition. |

## Compatibility and dependency evidence

The production dependency cone does not grow. `identus-oid4vp` may depend on
the existing workspace `identus-crypto` and `identus-did` packages only if typed
key/DID syntax is required; no external crate is added. Rust remains 1.89 MSRV
and 1.98.1 primary. The facade remains executor-, network-, filesystem-, root-
store-, and target-neutral for native, WASM, iOS, and Android library builds.

The transition is a deliberate pre-release source tightening: typed routing
moves from security evidence to verifier-authorized evidence. Wire syntax is
Final-compatible. Existing direct-post encryption metadata keeps its current
closed P-256/ECDH-ES/A128GCM claim and is not recharacterized as verifier trust.

Direct and resolved dependency-cone inspection uses `cargo tree -p
identus-oid4vp --edges normal --depth 2`; the intended cone adds no external
package. License and provenance evidence: repository code is Apache-2.0,
protocol-derived cases are clean-room
implementations based on openly published standards, and no copied donor code
or incompatible license enters the change. `cargo deny` and Nix remain the
supply-chain and advisory gates. No unsafe or native code is added; all adopted
workspace dependencies are already covered by the repository inventory.

Public compatibility is the declared pre-release source break recorded in the
change ledger. Wire behavior follows OpenID4VP 1.0 Final. Before publication,
rollback removes the authorization state and successors as one unit and never
restores an unsafe trust bypass.

Protocol or draft currency is explicit: OpenID4VP 1.0 Final published
2025-07-09 is normative; superseded drafts and donor SDK behavior are not.

## Security, privacy and maintenance evidence

All hostile JSON remains inside existing payload byte/depth/node/member/string
bounds, with new collection limits for metadata, verifier-info entries, and
credential references. Ports receive borrowed redacted evidence and return a
static accept/reject/unavailable outcome. No verifier-controlled value appears
in `Debug`, `Display`, or errors. The SDK creates authorization evidence only
after the port accepts the exact key that already verified the correlated JWS.

Ownership stays with the OID4VP, JOSE, and DID maintainers. The port accepts
borrowed bounded evidence, does not retain or log it, and cannot construct
trusted SDK states. Static errors separate invalid input, rejection, and
provider unavailability without surfacing dependency diagnostics.

## Rejected or deferred candidates

Concrete PKI, DNS, Federation, attestation, HTTP, and persistence adapters are
deferred to consumers or later named profiles. Foreign SDK DTO/error facades,
automatic unknown-prefix fallback, unsigned redirect-URI requests, and DC API
origin processing are rejected from this slice. General-purpose OpenID crates
remain references because they do not provide this bounded Final transition.

## Open questions and blockers

No planning blocker remains. Metadata display semantics and product trust are
intentionally unspecified. Future concrete adapters need their own ADR,
dependency assessment, vectors, and target evidence.

## Reconsideration triggers

Reconsider a concrete X.509, Federation, or attestation engine only when two
adopters require the same profile and a maintained crate satisfies MSRV,
portable targets, bounded input, license, advisory, and SDK-owned facade rules.
Reconsider unknown-prefix fallback only when a named profile requires it; the
default remains fail closed.

## Evidence commands

Exact research commands include `rg` and `sed` repository inspection, `cargo
tree -p identus-oid4vp --edges normal --depth 2`, and inspection of the
2025-07-09 Final specification. Planning evidence uses `scripts/factory
research-ready`, `constraints-ready`,
`check`, and issue-bound `preflight`. Implementation evidence must include
focused/all-feature/no-default tests, clean-room E2E vectors, property tests,
fuzz, benchmark, strict Clippy/rustdoc, portable builds, full factory/Nix gates,
and exact-head Claude-or-Codex CLI review. All implementation, portable, fuzz,
benchmark, Nix, hosted-CI, and review commands are unrun at planning time and
become receipts only after successful execution.
