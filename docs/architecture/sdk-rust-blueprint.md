# Hyperledger Identus SDK for Rust blueprint

**Repository:** `hyperledger-identus/sdk-rust`

**Active integration base:** `develop`, selected from
`yet-another-seed@662f8d7`

**Reserved branch:** protected `main` remains at `2c267d6` and intentionally
outside the current integration/release flow

**Bootstrap decision:** [ADR 0001](../adr/0001-bootstrap-branch-selection.md)

**Initial downstreams:** `midnight-identity`, neoprism, Oxid and the Identus
SDK family; adoption remains downstream-owned

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

## 3. Bootstrap principles

1. **Contract before port.** Every component issue defines its standards,
   owner crate, public contract, non-goals and evidence before code moves.
2. **One component at a time.** One issue, one worktree, one focused draft PR
   and one independently testable result.
3. **No roadmap stubs.** Do not create empty crates merely to reserve the
   future workspace layout. Namespace placeholders are a release-governance
   operation under issue #3, not API commitments.
4. **Plain Cargo is first class.** Nix is the reproducible maintainer/CI path,
   but a consumer can build and test supported crates with stable Cargo.
5. **Stable Rust.** Nightly-only language features are rejected. MSRV changes
   are explicit compatibility decisions and cannot follow a rolling toolchain
   accidentally.
6. **Verification is not trust.** The SDK returns technical evidence. The
   consumer decides whether an issuer, verifier, chain, credential or action is
   trusted and permitted.
7. **Secrets are handles.** Secret bytes never cross a public protocol, log,
   error or FFI surface when an opaque provider/handle can be used.
8. **Parsed is not verified.** Public types preserve construction state where
   confusing untrusted syntax with verified semantics would be unsafe.
9. **Bound every input.** Parsers and decoders have byte, element, nesting,
   decompression, redirect and time limits with privacy-safe errors.
10. **Evidence travels with code.** Vectors record source, source revision,
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

The inherited `identus-agent`, `identus-bindings`, `identus-credentials`,
`identus-messaging`, `identus-openid4vc`, `identus-presentations`,
`identus-trust` and `identus-wallet` packages are empty placeholders. They must
not be published or represented as supported capabilities. Each is removed,
quarantined or replaced only through an accepted component issue and namespace
decision.

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

## 6. Delivery program

### B00 — `develop` selection and governance

Create `develop` at the selected seed revision. Deliver this blueprint,
governance, maintainer inheritance, contribution, security, release,
repository-setting and agentic-SDLC policies. Record the branch evidence and
known debt in ADR 0001 while leaving `main` unchanged.

**Exit:** the signed/DCO governance commit is on `develop`; maintainers approve
the boundary, transition rules and repository controls.

### B01 — selected-baseline stabilization (`#4`)

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
then reconcile it with neoprism's `identus-apollo` and the existing KMP Apollo
behavior. Preserve published vectors and the explicitly profiled PRISM legacy
verification path.

**Exit:** stable/wasm gates pass, sensitive material is zeroized/redacted, and
neoprism plus KMP differential vectors pass. Midnight Jubjub remains outside.

### B05 — DID Core convergence (`#5`)

Reconcile `midnight-did-domain` and neoprism's DID core with W3C DID 1.0:
validated DID/DID URL grammar, embedded-or-reference relationships, lossless
extensions, JWK boundary, resolver/registrar contracts, metadata and typed
errors. Decide parser strategy and wire-error dialect in an ADR.

**Exit:** midnight-identity and neoprism compile consumer-shaped adapters
outside their production branches; W3C and negative/fuzz vectors pass.

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

### B09 — OID4VCI Final (`#7`)

Move the already cross-implementation-tested Oxid/Portal profile into one
versioned wire/state crate. Extension points remain chain-owned. Acceptance
requires the provenance-preserving fixture suite and both consumers' design
review, not production consumer edits.

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
- MSRV plus the pinned NeoPRISM-etalon toolchain;
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
