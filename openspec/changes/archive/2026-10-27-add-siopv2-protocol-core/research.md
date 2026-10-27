# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation state is split: SDK-Rust already owns bounded DID,
JOSE and OpenID request-retrieval pieces, while Oxid owns the only Identus Rust
SIOPv2 flow together with product lifecycle and adapters. The reusable gap is
protocol validation, response planning and deterministic lifecycle semantics;
it is not cryptography, UI, transport or custody. Consumer evidence comes from
the pinned Oxid source and its tests, but does not define Rust DTOs or errors.

## Normative sources

The authoritative source is Self-Issued OpenID Provider v2 draft 13, published
28 November 2023 at
https://openid.net/specs/openid-connect-self-issued-v2-1_0.html. The OpenID
Foundation AB/Connect specifications page still classifies SIOPv2 as an
Implementer's Draft and links its first Implementer's Draft. It is not a Final
specification. This slice pins draft 13 by exact title/date and makes every
future draft or Final transition a new compatibility decision.

Draft 13 defines a Self-Issued ID Token by `iss == sub`, requires audience,
expiry, issue time and nonce processing, supports DID subject syntax with a
JOSE `kid` selecting the DID verification method, and defines cross-device
`direct_post` to the request's `redirect_uri`. Self-attested claims are not
third-party evidence. OpenID4VP is a separate extension for verifiable
presentations and is outside this crate.

The first SDK profile is deliberately narrower than the whole draft:

- signed request object with exact outer/inner client binding;
- `response_type=id_token`, `id_token_type=subject_signed_id_token`, exact
  `scope=openid`, and `response_mode=direct_post`;
- HTTPS relying-party and redirect identifiers;
- one DID subject syntax and a caller-authorized ES256 or Ed25519/legacy-EdDSA
  verification/signing capability already supported by `identus-jose`;
- no self-attested user claims and no JWK-thumbprint subjects.

The primary source URL above is pinned by title, publication date and draft
number. The donor source revision and published package versions below are
immutable evidence inputs rather than automatic compatibility authorities.

## Candidate decisions

Oxid `origin/develop` revision
`3ceea617557dce795a247fdd9c6180a873f1611e` contains a draft-13 standalone
adapter, application lifecycle and independent verifier. It is
`conformance-only`: request/response invariants, negative cases, replay
consumption and Ed25519/P-256 examples inform tests, but product DTOs, loopback
URLs, in-process storage, consent intent, Dioxus behavior, Midnight DID
adapters and custody do not move into SDK-Rust.

Oxid uses a non-normative `response_uri` field and an `openid4vp://authorize`
fixture. SDK-Rust follows draft 13's `redirect_uri` and protocol-neutral
invocation boundary instead of preserving those implementation choices. A
future Oxid adapter owns any compatibility translation and deprecation.

Apollo and NeoPRISM contain no authoritative SIOPv2 engine. They remain
unmodified. Their cryptographic and DID capabilities are reused through the
existing SDK crates rather than copied into this protocol slice.

Published package metadata and source archives were inspected on 2026-10-27.
Exact version and features, license and provenance, declared MSRV, target
metadata, direct and resolved dependency cone, unsafe and native-code evidence,
and public DTO/error ownership were compared. Supply-chain evidence is the
immutable crates.io archive/checksum plus the repository's pinned advisory and
license gates; no package was added to the product lock during research.

| Candidate | Exact release | Disposition | Reason |
| --- | --- | --- | --- |
| `siopv2` / Impierce | 0.2.0, default feature set | `oracle` | Apache-2.0 and draft-13 coverage, but MSRV 1.95 exceeds SDK MSRV 1.89; the normal cone owns `oid4vc-core`, `jsonwebtoken`, Tokio, Reqwest/middleware/retry, `anyhow`, tracing, time and provider/RP runtime policy. |
| `affinidi-siopv2` | 0.1.4, no declared optional features | `oracle` | Apache-2.0 and narrower source, but MSRV 1.95 exceeds SDK MSRV; it exposes Affinidi core DTOs and adds its own thumbprint, time, tracing and error policy rather than fitting the existing Identus DID/JOSE boundaries. |
| `siop` | 0.1.0 | `not-adopt` | Empty published crate with no implementation or MSRV evidence. |
| `openidconnect`-style clients | current releases | `not-adopt` | Traditional hosted-OP clients do not implement DID-subject self-issued token semantics and introduce transport/provider policy. |
| existing Identus crates | current workspace | `adopt` | `identus-jose` supplies bounded JWS inputs, injected signers and signature suites; `identus-did` supplies DID/DID URL values; `identus-openid4vc` can own the already-proven neutral request retrieval seam. |
| SDK-owned SIOPv2 protocol facade | issue #514 | `retain-local` | Own only version-pinned validation, bounded serialization and lifecycle semantics while delegating crypto and effects through existing ports. |

No third-party SIOPv2 package becomes a production dependency. This is not a
decision to reimplement cryptography: signing and verification remain behind
the reviewed `identus-jose` capability and typed crypto keys. The local code
owns only protocol validation, bounded JSON/form encoding and deterministic
state transitions.

## Compatibility and dependency evidence

Activate `identus-openid4vc` only after two protocol consumers exist. Move the
runtime-neutral request retrieval value contract out of OID4VP, keep OID4VP
re-exports for source compatibility, and avoid a generic OAuth/OIDC umbrella.
Protocol-specific invocation parsing and request claims stay in their protocol
crates.

`identus-siopv2` depends downward on `identus-openid4vc`, `identus-jose`,
`identus-did`, `identus-core`, bounded JSON/URI utilities and zeroization. It
does not depend on OID4VP, Oxid, Midnight, a network runtime, storage or a DID
method.

The facade boundary exposes only SDK-owned types and stable errors. Public and
wire compatibility are additive because the new crate is unpublished and the
OID4VP retrieval names remain re-exported. The state machine is pure and caller-driven. It accepts explicit time and
bounded event IDs, returns typed effects, and retains no storage implementation.
Sensitive request/token material is zeroized on drop and redacted from
`Debug`/errors. Persistence adapters, if later introduced, may retain only a
separately specified checkpoint; this slice makes no persistence claim.

Rollback removes the additive crate and shared delegation before publication;
existing OID4VP behavior and wire output remain unchanged. The maintenance,
release and security posture is experimental until independent conformance,
portable targets and a consumer canary are accepted.

## Security, privacy and maintenance evidence

- Reject duplicate JSON members, unknown profile members, excessive depth,
  node counts, string sizes, compact size and form size before expensive work.
- Verify the request object before trusting redirect, nonce, state, time,
  metadata or subject syntax.
- Require exact client/audience binding and caller-authorized verification key;
  signature validity never implies relying-party trust.
- Require `iss == sub`, exact audience/nonce, bounded lifetime and DID URL
  `kid`; no arbitrary claims enter the first ID Token.
- A POST `wallet_nonce` must match the signed request exactly, and a SHA-256
  request-object fingerprint is inert evidence for a caller-owned replay store;
  the SDK does not pretend in-memory validation is durable replay defense.
- Refusal, cancellation and timeout never invoke a signer.
- Unknown delivery outcome is terminal until the caller starts a separately
  correlated recovery; no automatic resend can duplicate authentication.
- Errors are static and diagnostics reveal no request object, token, nonce,
  state, DID, method, redirect, subject claim or key material.

## Rejected or deferred candidates

Direct adoption of the two implemented SIOPv2 crates is deferred because of
MSRV, dependency-cone and public-policy mismatch. The empty `siop` crate and
traditional hosted-OP clients are rejected. Reimplementing signature or hash
primitives is rejected; the existing JOSE/crypto facade remains authoritative.
JWK-thumbprint subjects, dynamic discovery, federation, authorization-code
flow, attester tokens, arbitrary self-attested claims and production replay
storage are deferred to independent profile issues.

## Quality plan

Property tests cover event-identity boundaries and transition
legality/idempotency. E2E tests cover time, request/response round trips,
retrieval nonce binding and independent request/response algorithms. Fuzzing
targets compact parsing, signed arbitrary request JSON, response construction
and state events under finite limits. The benchmark measures ordinary signed
request verification; response planning remains an explicit follow-up. The conformance test executes the checked-in
clean-room normative vector; Impierce, Affinidi and Oxid remain pinned research
oracles until attributed automated fixtures are added.

The E2E route is invocation/reference → bounded retrieval → signed request
verification → validated SIOP request → explicit approval → JOSE signer →
exact `direct_post` form → independent signature verification and claims
assertion. A refusal/timeout matrix proves the signer is not called.

## Open questions and blockers

No blocker remains for the narrow profile. JWK-thumbprint subjects, dynamic OP
discovery, federation, authorization-code flow, attester tokens, arbitrary
claims, production replay storage and draft-to-Final migration require
separate issues.

The reconsideration trigger is a maintained stable SIOPv2 crate that meets the
SDK MSRV and portable targets, offers a narrow feature/dependency cone, keeps
network/runtime and DTO policy outside the protocol facade, and passes the same
redaction, bounds, conformance and supply-chain gates. A future SIOPv2 Final
specification is a separate reconsideration trigger and migration decision.

## Evidence commands

Executed research commands:

```text
cargo search siop --limit 20
cargo search self-issued --limit 20
cargo search openid4vc --limit 30
cargo info siopv2
cargo info affinidi-siopv2
cargo info siop
cargo info oid4vc-core
```

Published manifests and source archives were inspected directly, including
their feature declarations and normal dependencies. Exact target builds and a
resolved `cargo tree` were not run because neither foreign crate is being
adopted; these are recorded as unrun checks rather than implied evidence. The
repository's workspace, MSRV, portable target, advisory, license, fuzz and Nix
commands become required after implementation.
