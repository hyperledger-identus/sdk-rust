# Hyperledger Identus SDK for Rust blueprint

**Repository:** `hyperledger-identus/sdk-rust`

**Active integration base:** `develop`, selected from
`yet-another-seed@662f8d7`

**Reserved branch:** protected `main` remains at `2c267d6` and intentionally
outside the current integration/release flow

**Bootstrap decision:** [ADR 0001](../adr/0001-bootstrap-branch-selection.md)

**Initial downstreams:** `midnight-identity`, NeoPRISM, Lace ID Portal, Oxid
and the Identus SDK family; adoption remains downstream-owned

**Canonical component program:**
[SDK SSI upstream dependency backlog](../roadmap/ssi-upstream-dependency-backlog.csv),
with immutable donor classifications in the
[source matrix](ssi-upstream-source-matrix.md) and coordination in
[issue #20](https://github.com/hyperledger-identus/sdk-rust/issues/20). The
machine-readable [support policy](sdk-support-policy.toml) and its
[compatibility explanation](sdk-support-policy.md) govern target claims. The
[bootstrap inventory](sdk-bootstrap-inventory.md) distinguishes implemented,
verification-only and quarantined-placeholder packages and records the
repository-local `IDR-001` evidence.
The [Rust reuse research](../research/rust-library-reuse/report-source.md),
[ADR 0061](../adr/0061-adopt-narrow-crates-behind-identus-facades.md) and
[ADR 0064](../adr/0064-separate-primary-rust-from-evidence-driven-msrv.md)
govern third-party crate selection and independent primary-stable, MSRV and
etalon policy. [ADR 0063](../adr/0063-make-material-constraints-explicit.md) and the
[constraint index](../governance/sdk-constraints.toml) distinguish effective
cross-cutting promises, future targets, prohibitions and known limitations.

## 1. Mission

Build the chain-agnostic Rust foundation for decentralized identity and
verifiable credential products in the Identus ecosystem. The repository owns
standards-derived domain models, protocol wire/state engines, cryptographic
utilities, ports and conformance assets that can be used by multiple chains and
multiple products.

It does **not** own a wallet product, chain operations, custody implementation,
user-consent policy, cloud deployment, UI or AI-agent authorization. Those are
consumer responsibilities.

## 2. Repository boundary

The SDK is Layer 1 of the three-layer component model:

```text
applications: Oxid, Lace ID Portal, wallets and agents
                         │
                         ▼
chain families: midnight-identity, neoprism/did:prism, future chains
                         │
                         ▼
Identus SDK-Rust: generic SSI models, protocols, ports and evidence
```

Dependencies point downward only. Therefore SDK crates must not depend on:

- `midnight-*`, `compact-runtime`, Cardano/PRISM ledger clients or chain
  transaction types;
- Oxid, Lace ID Portal or another product repository;
- UI, wallet database, cloud-agent or AI-agent runtime frameworks;
- a consumer's trust, disclosure, consent or custody policy.

Chain- and product-specific behavior enters through typed ports or adapter
crates outside this repository.

The `identus-conformance` suite enforces this boundary across direct normal,
development, build and target-scoped Cargo declarations, renamed packages,
donor sources, local paths and the resolved lockfile closure. The guard is
offline and does not build a donor or consumer repository.

## 3. Bootstrap principles

1. **Contract before port.** Every component issue defines its standards,
   owner crate, public contract, non-goals and evidence before code moves.
2. **Research before implementation.** Every qualifying OpenSpec change
   passes `factory research-ready`; foundational/protocol/security work records
   build-versus-adopt evidence and negative decisions.
3. **Constraints before activation.** Every qualifying change passes
   `factory constraints-ready`; targets, limitations and effective promises
   remain distinct, and material outcomes cite their exact decision authority.
4. **One component at a time.** One issue, one worktree, one focused ready PR
   after local review, and one independently testable result.
5. **No roadmap stubs.** Do not create empty crates merely to reserve the
   future workspace layout. Namespace placeholders are a release-governance
   operation under issue #3, not API commitments.
6. **Plain Cargo is first class.** Nix is the reproducible maintainer/CI path,
   but a consumer can build and test supported crates with stable Cargo.
7. **Stable Rust.** Nightly-only language features are rejected. MSRV changes
   are explicit compatibility decisions and cannot follow a rolling toolchain
   accidentally.
8. **Verification is not trust.** The SDK returns technical evidence. The
   consumer decides whether an issuer, verifier, chain, credential or action is
   trusted and permitted.
9. **Secrets are handles.** Secret bytes never cross a public protocol, log,
   error or FFI surface when an opaque provider/handle can be used.
10. **Parsed is not verified.** Public types preserve construction state where
   confusing untrusted syntax with verified semantics would be unsafe.
11. **Bound every new or changed input.** New and materially changed parsers
   and decoders have byte, element, nesting, decompression, redirect and time
   limits with privacy-safe errors. Inherited surfaces remain unsupported for
   unbounded hostile input until their limits are audited and enforced.
12. **Evidence travels with code.** Vectors record source, source revision,
    license, retrieval date, transformation and expected result.

## 4. Baseline and target crate portfolio

The selected baseline already contains real and placeholder crates. The
decision to preserve its history does not make every crate name or API an
accepted product commitment.

| Implemented baseline crate | Current value | Bootstrap treatment |
| --- | --- | --- |
| `identus-core` | validated value/error foundations | audit construction, serialization and error contracts |
| `identus-derive` | validated-newtype derivation support | harden macro diagnostics and compiler-version tests |
| `identus-crypto` | chain-neutral algorithms, HD derivation and published vectors | reconcile namespace/API under Apollo issue #9; retain vector evidence |
| `identus-did` | initial generic DID types | converge against DID Core issue #5; do not infer a chain method |
| `identus-adapters-entropy` | system entropy adapter | retain as the first ports implementation and review runtime/target support |
| `identus-conformance` | dependency-layer and workspace guards | expand with provenance and architecture evidence |

The inherited `identus-agent`, `identus-bindings`, `identus-messaging`,
`identus-openid4vc` and `identus-trust` packages remain empty placeholders.
They must not be published or represented as supported capabilities. Each is
removed, quarantined or replaced only through an accepted component issue and
namespace decision. Focused issues have activated experimental credential,
presentation and wallet-storage surfaces in their inherited packages; this is
not a namespace-stability, publication or wallet-product commitment. Issue
Issue #111 activates the separate experimental `identus-oid4vci` package for the
Credential Offer transport boundary without changing the quarantined
`identus-openid4vc` umbrella marker.

The intended component portfolio below is a planning target. A crate enters
the supported portfolio only through an accepted slice; inherited names do not
reserve the final namespace automatically.

| Crate | Responsibility | Explicit exclusions |
| --- | --- | --- |
| `identus-ports` | time, entropy, HTTP, secure-key/signing, storage transaction and cancellation contracts | production storage, platform keychain, browser or cloud runtime |
| `identus-apollo` | validated encodings, public-key/JWK conversion, curve utilities and HD derivation required by SSI formats | custody, JOSE protocol, Midnight Jubjub suite |
| `identus-did-core` | DID/DID URL, document, verification relationships, services, resolution/dereference/registration contracts and metadata | DID method ledger behavior, HTTP server/client |
| `identus-did-resolver-http` | optional Axum binding for the W3C DID Resolution HTTP interface over any resolver | chain resolver implementation |
| `identus-vc-core` | format-neutral credential/presentation envelopes, disclosure descriptors, staged verification evidence and status vocabulary | format codecs, trust policy, inventory/storage IDs |
| `identus-jose` | bounded JWS compact codec and narrowly profiled builder/verifier APIs | general-purpose JOSE, JWE, custody |
| `identus-oid4vci` | OID4VCI Final wire types, validation and resumable holder/issuer state contracts | wallet UI, browser launching, issuer policy |
| `identus-oid4vp` | OID4VP Final, DCQL and response-state contracts | credential selection and consent UI |
| `identus-siopv2` | SIOPv2 profile types and validation | relying-party trust policy |
| `identus-conformance` | fixture schemas, provenance checks, architecture/dependency guards and reusable harnesses | product end-to-end tests |
| `identus-ffi-*` | deliberately small UniFFI/WASM-facing value and handle APIs | exposing generic Rust traits or raw secrets |

An umbrella `identus-sdk` crate is deferred until at least two independent
consumers need a stable curated facade. Generic names such as `identus-core` or
`identus-crypto` may be reserved defensively without becoming real catch-all
crates.

## 5. Target dependency direction

```text
                     identus-ffi-*
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
       identus-oid4vci  identus-oid4vp  identus-siopv2
             │             │             │
             └───────┬─────┴──────┬──────┘
                     ▼            ▼
                identus-jose  identus-vc-core
                     │            │
                     └──────┬─────┘
                            ▼
                    identus-did-core
                       │          │
                       ▼          ▼
                identus-apollo  identus-ports

identus-did-resolver-http -> identus-did-core + optional HTTP framework
identus-conformance       -> dev/test edges only
```

The exact graph can become narrower. A new upward or sideways dependency needs
an ADR and dependency-cone evidence.

The support matrix deliberately separates host-tested Rust behavior from
compile-only browser/mobile evidence and from planned targets. Rust `1.85.0`
is tested as the stable consumer floor independently of primary stable Rust
`1.98.1` and the pinned NeoPRISM-etalon nightly. There is no supported FFI during bootstrap, and
binary size/build time remain measurement-only until a candidate release
defines reproducible artifacts and budgets.

ADR 0064 replaces the rolling policy with evidence-driven selection and names
Rust 1.89 as the next candidate. Rust 1.85 remains effective until a focused
activation decision updates Cargo, Nix and machine policy together.

## 6. Delivery program

The `B00`–`B12` sequence below explains program intent. The canonical CSV is
the machine-checkable delivery ledger for the thirty SDK-owned `IDR-*` rows.
Its `commitment` records the portfolio decision, while `delivery_status`
records actual progress; a `Foundation` or `Committed` row is not evidence that
the component exists. Rows still linked to #20 are queued program work and
receive a focused issue before implementation.

### B00 — `develop` selection and governance

Create `develop` at the selected seed revision. Deliver this blueprint,
governance, maintainer inheritance, contribution, security, release,
repository-setting and agentic-SDLC policies. Record the branch evidence and
known debt in ADR 0001 while leaving `main` unchanged.

**Exit:** the signed/DCO governance commit is on `develop`; maintainers approve
the boundary, transition rules and repository controls.

### B01 — selected-baseline stabilization (`#4`, `#22`)

Align the declared MSRV with the NeoPRISM-etalon pinned Rust toolchain and Nix
inputs; fix deterministic host and Nix gates; inventory public APIs; classify
all placeholder crates; verify license/provenance; and make CI run on
`develop`. Preserve working seed code and published vectors while removing
accidental roadmap commitments.

Required gates: fmt, clippy with warnings denied, test/doctest, MSRV, stable,
Linux/macOS, eligible WASM, `cargo deny`, advisory scan, docs, license/source
policy, architecture checks and generated/fixture drift.

**Exit:** fresh-clone plain-Cargo and Nix instructions pass; no chain/product
dependency is present; every workspace member is implemented or explicitly
quarantined; baseline limitations are documented.

`IDR-002` and `IDR-003` are delivered by issue #22: the chain-neutral boundary
and compatibility matrix are machine-enforced. Issue #25 delivers the
repository-local governance/API inventory, publication denial and placeholder
quarantine for `IDR-001`. The row remains in progress until accountable
maintainers complete the public repository and protected-control activation in
issue #26; namespace and trusted-publishing ownership remain separate under
issue #3.

### B02 — namespace and release ownership (`#3`)

Agree crate names with Identus maintainers, establish organization-controlled
crates.io ownership and trusted publishing, and reserve only approved names.
Do not publish inherited empty crates merely to hold a name.

**Exit:** ownership and recovery are controlled by the project, not an
individual account.

### B03 — ports contract

Define the minimum object-safe or async-compatible ports needed by the first
two components: clock, entropy, key handles, sign/verify, resolver I/O and
cancellation. Ship deterministic in-memory test implementations only.

**Exit:** two unrelated consumers can implement the ports without framework or
runtime coupling.

### B04 — Apollo convergence (`#9`)

Harden the selected baseline's validated newtype, crypto and derivation work,
then reconcile it with NeoPRISM's Rust `lib/apollo` and the existing Kotlin
Multiplatform Apollo behavior. Apollo supplies compatibility behavior and
vectors; it is not a Rust source port. Preserve published vectors and the
explicitly profiled PRISM legacy verification path without deciding the final
crate brand or Apollo repository lifecycle in the component implementation.

**Exit:** stable/wasm gates pass, sensitive material is zeroized/redacted, and
neoprism plus KMP differential vectors pass. Midnight Jubjub remains outside.

### B05 — DID Core convergence (`#5`)

Reconcile `midnight-did-domain` and neoprism's DID core with W3C DID 1.0:
validated DID/DID URL grammar, embedded-or-reference relationships, lossless
extensions, JWK boundary, resolver/registrar contracts, metadata and typed
errors. Decide parser strategy and wire-error dialect in an ADR.

**Exit:** midnight-identity and neoprism compile consumer-shaped adapters
outside their production branches; W3C and negative/fuzz vectors pass.

Issue #101 removes the unused `identus-did -> identus-crypto` default-feature
edge. DID documents retain bounded structural public-JWK handling, while an
operation-owning consumer such as JOSE selects and binds its exact algorithms.
The featureless DID crate therefore adds no curve, derivation, hashing or COSE
code to a DID-only dependency cone.

### B06 — DID Resolution HTTP (`#10`)

Port the chain-neutral Axum binding after B05. Keep content negotiation and
error/status behavior byte-compatible with its source fixtures.

### B07 — VC core convergence (`#6`)

Reconcile the Oxid holder envelope and staged verification report with
midnight-vc-domain's status vocabulary. Prove an open format registration
point with two unrelated test formats. Do not import Oxid IDs or policy.

### B08 — bounded JOSE (`#8`)

Deliver strict, narrowly profiled JWS compact handling and proof-JWT
builder/verifier APIs over DID resolution and signing ports. Fuzz all decoder
boundaries and reject algorithm confusion, ambiguous keys and unbounded claims.

Issue #95 implements the first wire-only foundation: bounded canonical Compact
Serialization, a closed initial protected header, exact signing-input
preservation and explicit unverified state. Issue #98 adds runtime-neutral
external signing, exact algorithm-bound public keys, a bounded caller-owned
allowlist/dispatcher, fully specified `Ed25519` and `ES256` suites, opt-in
legacy `EdDSA`, and explicit cryptographically verified state. DID key
authorization and OpenID4VCI proof claims are split under #99: the first
delivery adds exclusive bounded `kid`/public-`jwk`/`x5c` headers plus staged
holder construction. Its second delivery adds three-stage issuer verification,
exact DID `authentication` authorization, an injected X.509 leaf-key provider,
explicit client/audience/nonce/freshness policy and an atomic caller-owned
replay gate. Issue #100 supplies bounded coverage-guided fuzzing across the
Compact boundary. Issue #104 adds bounded opaque proof-attestation and
trust-chain headers, explicit holder construction, an injected OpenID
Federation key provider, an injected key-attestation validator and a distinct
trust-evaluated state without selecting platform or product trust policy.

### B09 — OID4VCI Final (`#7`)

Move the already cross-implementation-tested Oxid/Portal profile into one
versioned wire/state crate. Extension points remain chain-owned. Acceptance
requires the provenance-preserving fixture suite and both consumers' design
review, not production consumer edits.

The first bounded delivery, issue #111, accepts only the Final Credential Offer
invocation transport: strict by-value JSON or least-authority HTTPS reference,
with explicit resource limits and redaction. Offer-member semantics, fetching,
trust and issuance state remain later slices under #7.

The second bounded delivery, issue #113, consumes that transport into a
validated core Credential Offer: exact HTTPS issuer syntax, a non-empty unique
configuration-ID list, object-shaped opaque grants, lossless extensions, and
independent semantic bounds. Grant internals, metadata matching, fetching,
trust, authorization selection, and issuance state remain later slices.

The third bounded delivery, issue #115, consumes the core offer into a
grant-validated state. It exposes the Final Authorization Code and
Pre-Authorized Code alternatives without choosing a flow; validates bounded
opaque state/code values, RFC 8414 Authorization Server syntax, and Transaction
Code object/mode/length/description rules; and keeps unknown object-shaped grant
extensions lossless. Metadata matching, fetching, trust, grant selection,
authorization/token messages, replay controls, and issuance state remain later
slices.

The fourth bounded delivery, issue #117, parses a bounded unsigned Credential
Issuer Metadata core and consumes it with the grant-validated offer to prove
exact issuer, offered-configuration, and multi-server grant-hint agreement.
It retains exact unknown metadata while exposing only safe issuer,
Authorization Server, Credential Endpoint, configuration ID, and opaque format
summaries. HTTP retrieval, signed metadata/trust, OAuth Authorization Server
metadata, format-profile interpretation, grant selection, endpoint messages,
replay controls, and issuance state remain later slices.

The fifth bounded delivery, issue #119, parses a bounded, lossless
`AuthorizationServerMetadataCore` with exact RFC 8414 issuer binding, safe
optional authorization/token endpoints, explicit or defaulted grant types, and
the OID4VCI anonymous Pre-Authorized Code flag. Its partial type deliberately
does not claim complete RFC 8414 conformance. Discovery, signed metadata,
trust, server/grant selection, endpoint messages, replay controls, and issuance
state remain later slices.

The sixth bounded delivery, issue #121, consumes the matched Credential Offer
and Credential Issuer Metadata state with one caller-selected Authorization
Server Metadata core. It proves that the offer contains the Pre-Authorized Code
grant, the server is effectively advertised and matches any grant hint, the
grant is explicitly supported, and a Token Endpoint is present. Server
discovery/ranking, trust, client authentication, Transaction Code input, Token
Request construction, endpoint execution, responses, retries, replay controls,
and issuance state remain later slices.

The seventh bounded delivery, issue #123, consumes that server-bound state
with optional caller-owned Transaction Code input. It requires input exactly
when the Final offer contains a `tx_code` object, applies an independent byte
bound, immediately adopts zeroizing ownership, and exposes only input presence
publicly. Advertised mode/length remain UI guidance rather than a local
validity verdict. Token Request encoding/execution, client
identity/authentication, responses, trust, retries/replay controls, and
issuance state remain later slices.

The eighth bounded delivery, issue #125, consumes that prepared input state
into one headless `PreAuthorizedTokenRequest`. It carries the validated Token
Endpoint, static POST/media-type guidance, and an exact bounded UTF-8 form body
containing the mandatory Final fields in deterministic order. The secret body
is zeroized and available only through an explicitly sensitive accessor;
diagnostics remain data-free. HTTP execution, client identity/authentication,
optional authorization selectors, responses, trust, retries/replay controls,
and issuance state remain later slices.

The ninth bounded delivery, issue #127, parses a bounded successful
`TokenResponseCore`. It validates the mandatory OAuth token fields and optional
expiry, refresh token, and scope; ignores bounded unknown members semantically;
owns response secrets in zeroizing storage; and reports only whether
`authorization_details` is present. HTTP response validation, correlation,
Authorization Details and Credential Dataset semantics, token trust/time,
refresh, nonce, Credential Requests/Responses, and replay controls remain later
slices.

The tenth bounded delivery, issue #129, parses a bounded OAuth
`TokenErrorResponseCore`. It preserves exact extension-compatible error codes,
classifies the six standard token-endpoint errors, validates optional untrusted
developer description and URI-reference metadata, discards bounded unknown
members, zeroizes retained strings, and keeps diagnostics data-free. HTTP
status/header validation, request correlation, retry/remediation policy, URI
navigation, trust, nonce, Credential Requests/Responses, and replay controls
remain later slices.

The eleventh bounded delivery, issue #131, parses a bounded OID4VCI Final
`CredentialNonceResponseCore`. It retains the required opaque non-empty
`c_nonce` exactly in zeroizing storage without inventing base64url, ASCII,
entropy, fixed-length, or expiry semantics; bounded unknown members are checked
and discarded, and diagnostics remain data-free. Metadata endpoint exposure,
Nonce Request transport, HTTP status/media/cache/DPoP validation, Issuer nonce
generation, freshness/replay state, proof construction, and Credential
Requests/Responses remain later slices.

The twelfth bounded delivery, issue #133, exposes the optional OID4VCI Final
`nonce_endpoint` from bounded Credential Issuer Metadata as an exact,
redaction-safe HTTPS URL. It reuses the existing endpoint byte budget without
breaking the public limits constructor, preserves port/path/query components,
and distinguishes unsafe and oversized values with static diagnostics. Nonce
Request transport, HTTP status/media/cache/DPoP validation, endpoint trust and
network safety, nonce lifecycle, proof construction, and Credential
Requests/Responses remain later slices.

The thirteenth bounded delivery, issue #135, constructs an owned,
transport-neutral Final `CredentialNonceRequest` only from issuer metadata
that advertises a validated Nonce Endpoint. It exposes exactly POST, an empty
byte body, and no access-token requirement while keeping the endpoint in
independent zeroizing storage. HTTP execution and headers, network policy,
response validation and correlation, trust, nonce lifecycle, proof
construction, and Credential Requests/Responses remain later slices.

The fourteenth bounded delivery, issue #137, validates caller-supplied Final
Credential Nonce HTTP response metadata through a validated request. It
requires a 2xx status, an RFC-shaped case-insensitive `application/json` media
type, an RFC-shaped Cache-Control list containing bare `no-store`, independent
field/body bounds, and the existing strict response-body parser. HTTP
execution, network/trust policy, generic header collection, DPoP, actual
response provenance, nonce lifecycle, proof construction, and Credential
Requests/Responses remain later slices.

The fifteenth bounded delivery, issue #139, constructs the unencrypted Final
Credential Request for the configuration-ID/JWT-proof path. It selects one ID
from the matched offer, requires a successful Bearer Token Response without
opaque Authorization Details, accepts non-empty holder-produced
`Oid4vciProofJwt` values through the architecture-approved JOSE dependency,
validates RFC 6750 Bearer syntax, and owns bounded deterministic JSON and
Authorization values in zeroizing storage. HTTP execution, DPoP, token/proof
trust and freshness, Authorization Details/Credential identifiers, encryption,
format or chain extensions, and Credential Response processing remain later
slices.

The sixteenth bounded delivery, issue #141, parses the unencrypted Final
immediate Credential Response JSON body. It requires a non-empty ordered
`credentials` array, accepts only string or object credential values, retains
their exact JSON in zeroizing ownership, exposes decoded string values, accepts
an optional opaque `notification_id`, and bounds response structure, member
counts, credential counts and retained allocations independently. Unknown
unique extensions remain interoperable while duplicate names fail, and the
deferred `transaction_id` branch is reported as explicitly unsupported. HTTP
metadata/provenance, request correlation, error/deferred/encrypted responses,
format decoding and verification, issuer/schema/status trust, notifications,
storage, chain extensions and product policy remain later layers.

The seventeenth bounded delivery, issue #143, binds that immediate response to
the originating configuration-ID/JWT-proof request. It requires exact HTTP
status 200, a bounded RFC-shaped case-insensitive `application/json` effective
media type, the existing bounded body parser, and the necessary
`credential_count <= proof_count` upper bound. Status 202 remains explicitly
unsupported and is classified before remote fields are inspected. The state
does not claim HTTP execution or provenance, distinct proof keys,
credential-to-key binding, format verification, trust, notification execution,
storage or replay safety; error/deferred/encrypted responses remain later
slices.

The eighteenth bounded delivery, issue #145, parses the Final Credential
Endpoint payload-error body. It requires one bounded duplicate-safe JSON
object with an exact non-empty error code and optional explicitly untrusted
description, recognizes the seven Final values while preserving valid
extensions, discards bounded unknown and legacy fields, zeroizes retained
strings, and keeps diagnostics data-free. HTTP status/media/authentication,
RFC 6750 Authorization errors, request correlation, issuer truth,
retry/remediation/UI policy, deferred or encrypted responses, credential
verification, storage and product policy remain later layers.

The nineteenth bounded delivery, issue #147, validates that body through the
Final Credential Request payload-error HTTP envelope. It requires exact status
400 and one bounded RFC-shaped case-insensitive `application/json` effective
media value before body parsing, rejects the exact generic `invalid_request`
code displaced by the Final payload-error vocabulary, and preserves all other
valid extension codes. It deliberately ignores the non-normative example-only
Cache-Control header and returns only the existing redaction-safe body core.
HTTP execution/provenance, RFC 6750 Authorization Errors and challenges,
request correlation, retry/remediation/UI policy, deferred or encrypted
responses, credential verification, storage, and product policy remain later
layers.

The twentieth bounded delivery, issue #149, parses the Final deferred
Credential Response body. It requires one decoded non-empty `transaction_id`
and a mathematically positive JSON-number `interval`, retains the identifier
and exact interval lexeme in zeroizing ownership without numeric conversion,
rejects immediate-branch fields, and traverses unique extensions under
independent byte, depth, node, member and retained-value limits. It establishes
only bounded body syntax: HTTP 202/media binding, request correlation,
transaction validity, Deferred Credential Requests, polling, scheduling,
retry/UI policy, encrypted responses, verification, storage and consumer
adoption remain later issue-first layers.

### B10 — OID4VP Final, DCQL and SIOPv2

Crystallize issues before implementation. Pin final/errata versions and any
pre-final dependencies required by the selected high-assurance profile.

### B11 — formats and profiles

Add SD-JWT, SD-JWT VC, ISO mdoc, VCDM and status mechanisms one issue at a time.
Format crates own raw bytes and proof semantics; the VC core owns lifecycle and
evidence contracts only.

### B12 — bindings and 1.0 stabilization

Expose only consumer-required FFI surfaces. Require public API diff tooling,
migration guides, LTS/support policy, independent security review, two
independent production consumers and reproducible signed releases before 1.0.

## 7. Slice contract

Every implementation issue and PR records:

- owner crate and consumer outcome;
- exact base SHA and source SHAs/paths;
- normative standard version, errata and conformance source;
- public types, serialized forms, error taxonomy and compatibility promise;
- explicit non-scope and dependency cone;
- threat/misuse cases and resource bounds;
- fixture provenance and license;
- tests and target matrix;
- security, docs and conformance review requirements;
- candidate version, rollback and downstream adoption issue;
- confirmation that consumer repositories were read-only.
- constraint-impact class, affected index IDs and introduced limitations;
- exact decision authority and activation path for a material outcome.

The source matrix classification (`extract`, `adapt`, `conformance-only`, or
`remain-downstream`) is recorded before a port. A downstream deletion or
dependency repoint follows only after an immutable compatible SDK candidate;
it is never part of the upstream implementation issue.

## 8. Assurance model

### Per-PR fast lane

- formatting, lint and targeted unit/integration/doctest;
- affected feature/minimal-feature checks;
- architecture and dependency guards;
- fixture/provenance/generated-file drift;
- docs/link checks and public API diff;
- secret, license, source and advisory checks.

### Candidate lane

- full workspace and target matrix;
- MSRV, pinned primary stable and the independent NeoPRISM-etalon toolchain;
- WASM/mobile compile for eligible crates;
- fuzz/property/negative tests for parsers and crypto boundaries;
- cross-implementation conformance;
- SBOM, checksums and signed provenance;
- independent security review for crypto, parser, protocol and FFI changes.

Coverage is a ratchet, not an aggregate vanity score. Critical parser,
verification, secret and state-transition paths need explicit branch coverage
and negative tests regardless of repository percentage.

## 9. Version and compatibility policy

- `0.0.x`: reservation/bootstrap only; no stability promise.
- `0.x`: experimental component releases with SemVer and migration notes.
- `1.0`: only after public API review, independent security assessment,
  conformance publication and at least two independent consumers.

Crates version independently once their release cadence diverges. A coordinated
release manifest records tested combinations. Breaking wire behavior requires
a new capability/profile identifier when SemVer alone cannot protect peers.

## 10. Normative source registry

Each component pins an immutable publication/draft URL and content hash. The
initial registry includes:

- [W3C DID 1.0](https://www.w3.org/TR/did/);
- [W3C VC Data Model 2.0](https://www.w3.org/TR/vc-data-model-2.0/);
- [W3C Bitstring Status List 1.0](https://www.w3.org/TR/vc-bitstring-status-list/);
- [RFC 9901 SD-JWT](https://www.rfc-editor.org/rfc/rfc9901.html);
- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html);
- [OpenID4VP 1.0 Final](https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html);
- [OpenID4VC HAIP 1.0 Final](https://openid.net/specs/openid4vc-high-assurance-interoperability-profile-1_0-final.html).

Current errata are reviewed separately. A profile's pinned pre-final dependency
is not silently replaced by a later RFC; migration is an explicit capability
decision with interoperability evidence.

## 11. Exit from bootstrap

The repository leaves bootstrap when B00–B03 are accepted, the publishing
identity and security team are operational, mandatory branch rules match the
documented policy, and one real component can produce a reproducible signed
candidate without changing any consumer repository.
