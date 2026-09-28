# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The current implementation at pinned source revision
`3512f31a660d536cdee7a585ae95e2c0a6129790`
`identus-oid4vp` parses a bounded Final static `openid4vp:` invocation and owns
redacted `client_id`, HTTPS `request_uri`, and GET/POST intent. It performs no
retrieval or Request Object validation. Its parser accepts omitted/default GET
and explicit POST, but incorrectly rejects the Final explicit `get` spelling.

`identus-jose` already owns bounded compact JWS parsing, strict protected
header parsing with duplicate/unknown rejection, algorithm-bound public JWKs,
caller-owned signature-suite allowlists, and Ed25519/ES256 verification. Reuse
is an intended inward dependency edge; a second JAR implementation would
duplicate cryptographic and algorithm-confusion controls.

## Normative sources

- OpenID4VP 1.0 Final with current errata, retrieved from
  `https://openid.net/specs/openid-4-verifiable-presentations-1_0.html`.
  Sections 5 and 5.10 define Request Objects, case-sensitive `get`/`post`, POST
  media types and fields, response binding, `wallet_nonce`, exact `client_id`
  equality, and termination on HTTP errors.
- RFC 9101 sections 5 and 6 define Request Object retrieval, signed/encrypted
  objects, signature validation with an algorithm- and client-associated key,
  and use of only parameters from the Request Object.
- RFC 8725 sections 3.1 and 3.2 supply algorithm verification guidance already
  embodied in `identus-jose` key/registry binding.
- ADRs 0156 and 0157 fix Identus ownership, dependency direction, and the
  least-authority ingress transition.

The current OID4VP publication requires protected
`typ=oauth-authz-req+jwt`; a Wallet must reject absence or mismatch. POST uses
HTTPS, `application/x-www-form-urlencoded`, and Accept
`application/oauth-authz-req+jwt`; its optional wallet nonce must be repeated
exactly in the signed object. The specification permits signed, optionally
encrypted responses, but implementing encryption is a separate key-management
and algorithm surface.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Runtime-neutral Identus retrieval envelope | `retain-local` | Keeps HTTP authority with consumers while centralizing exact wire, bounds, and state transitions. | A focused transport crate provides smaller bounded parity without runtime coupling. |
| `identus-jose` | `adopt` | Existing reviewed compact JWS, header, algorithm/key, and signature registry owner. | Its API cannot preserve the OID4VP state or redaction contract. |
| New JOSE/JWT crate | `not-adopt` | Adds a dependency cone and competing crypto/error policy without missing capability. | A later algorithm unsupported by the internal registry requires a reviewed suite, not parallel parsing. |
| OID4VC framework | `oracle` | ADR 0156 found public-type, limit, and coupling mismatch. | A narrow release passes the SDK adoption gates and deletes more code than its adapter adds. |
| HTTP client/runtime | `not-adopt` | Runtime, TLS, redirect, DNS, timeout, and platform choices belong to the consumer adapter. | Never as a mandatory protocol-core dependency. |
| JWE support | `not-adopt` | Requires encryption key publication/selection, content algorithms, nesting, and zeroization decisions. | A named consumer and separate threat/specification slice. |
| Client-prefix verifier | `not-adopt` | DID, X.509, federation, and attestation have distinct trust inputs. | Separate prefix-specific issues after the generic signed envelope exists. |

## Compatibility and dependency evidence

The only new production edge is `identus-oid4vp -> identus-jose`, already
present in the accepted target graph. `identus-jose` resolves through existing
workspace families (`identus-core`, `identus-crypto`, `identus-did`, `base64`,
`serde`, and `serde_json`); no external package is added or exposed directly
beyond sibling Identus SDK types. The crate remains version `0.0.0`,
unpublished, and outside release trains. Exact version and features remain the
workspace-pinned `identus-jose` path package with its existing default feature
surface; no feature is enabled by this consumer. The direct and resolved
dependency cone is the existing locked JOSE cone and will be measured by Cargo
metadata and repository dependency guards before delivery.

Public compatibility is additive except for accepting explicit `get`, which
widens previously rejected valid input. The new state types are Identus-owned.
`JwsVerificationKey` and `SignatureSuiteRegistry` are explicit sibling-SDK
capabilities, not third-party types. Rust 1.89.0 remains the source floor and
Rust 1.98.1 the primary/etalon toolchain. The facade boundary exposes only
Identus SDK state, limit, error, registry, and algorithm-bound key types.

## Security, privacy and maintenance evidence

Independent limits apply to metadata, nonce, encoded request body, response
content type/body, compact JWS segments, decoded payload, JSON depth/nodes,
object members, and strings. Response binding rejects non-2xx status, wrong
media type, empty/oversized bodies, JWE shape, malformed compact JWS, missing or
wrong `typ`, and malformed/ambiguous JSON. Relevant duplicate claims fail.

The retrieval request owns the outer client id and sent nonce so the verified
transition can enforce exact signed correlation. A caller selects an
algorithm-bound key and explicit suite registry; the SDK verifies the JWS but
does not claim that the caller associated the key with a DID, certificate,
federation entity, attestation, or product trust policy. Documentation names
that obligation at each verification entry point.

Verifier-controlled strings and payloads use zeroizing ownership where
retained. `Debug`, `Display`, public errors, component metadata, metrics, and
tests expose only static categories, lengths, methods, and safe enum evidence.
The exact request object is available only through an explicitly sensitive
accessor after signature and correlation checks.

License and provenance are closed: no donor source or fixtures are copied;
authored changes remain Apache-2.0. Clean-room vectors are generated with
the SDK's existing Ed25519/ES256 facilities. Existing Cargo-deny, source,
dependency, unsafe, portable compile, and factory gates remain authoritative
supply-chain evidence.

## Open questions and blockers

None. The issue deliberately separates cryptographic validity from client-key
authorization and product trust. Audience, time, authorization semantics, and
DCQL must not be inferred from a `VerifiedRequestObject` and remain later
validation transitions.

Protocol or draft currency is fixed to OpenID4VP 1.0 Final plus the retrieved
current errata publication and RFC 9101. Behavioral errata require a focused
compatibility review rather than silent drift. Rollback removes the unpublished
additive modules, dependency edge, ADR, and delta specs; no consumer or stored
data migration exists.

## Evidence commands

Planning evidence is `scripts/factory doctor`, `scripts/factory validate
bind-oid4vp-request-uri-and-jar`, `scripts/factory check`, `scripts/factory
research-ready bind-oid4vp-request-uri-and-jar`, and `scripts/factory
constraints-ready bind-oid4vp-request-uri-and-jar`. Implementation evidence
will include focused tests, strict Clippy/rustdoc, workspace/factory checks,
dependency/inventory/error contracts, stable/MSRV, and applicable compile-only
WASM/iOS/Android checks. Hosted CI, runtime network interop, JWE, prefix trust,
certification, and downstream adoption are explicitly unrun/later evidence.

## Rejected or deferred candidates

No HTTP runtime, new JOSE/JWT dependency, full OID4VC framework, JWE engine, or
client-prefix trust implementation is adopted. They either duplicate an
existing cohesive SDK owner or introduce runtime, dependency-cone, key
management, native target, or trust coupling outside this transition. JWE and
prefix validation remain named later slices rather than hidden caller claims.
