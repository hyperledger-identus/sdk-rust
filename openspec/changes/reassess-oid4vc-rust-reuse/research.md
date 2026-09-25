# Rust OID4VC reuse reassessment

Research class: protocol
Research status: ready
Decision date: 2026-09-26
Source retrieval date: 2026-09-26
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-oid4vci` already owns OID4VCI 1.0 Final parsing, state transitions,
request construction, response binding, resource limits, and redacted errors.
`identus-presentations` owns generic request, candidate, disclosure, artifact,
receipt, and lifecycle semantics. `identus-openid4vc` remains a quarantined
placeholder and there is no accepted OID4VP implementation. The decision is
therefore not one framework versus no code: it is which layer, if any, can be
reused without surrendering the SDK's boundaries.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html)
  and [OpenID4VP 1.0 Final](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html)
  are normative. Protocol/draft currency is therefore the two Final standards,
  not an earlier draft.
- [Impierce `openid4vc`](https://github.com/impierce/openid4vc) at revision
  `e9d99d211` is a Final-labelled source oracle; its
  published 0.1 crates remain old placeholders and upstream uses git patches.
- [Spruce `openid4vp`](https://github.com/spruceid/openid4vp) at revision
  `e5f29b85` is a Final-labelled source oracle without a
  release or declared MSRV and with broad SSI, JOSE, HTTP, runtime, RSA, and
  X.509 coupling.
- [Spruce `open-auth2-rs`](https://github.com/spruceid/open-auth2-rs) at
  revision `5d653eac` is modular source evidence but has no
  release, tag, or declared MSRV and retains unconditional HTTP/form/JSON/RNG.
- [Affinidi TDK Rust](https://github.com/affinidi/affinidi-tdk-rs) supplies
  exact versions `affinidi-openid4vci 0.2.1`, `affinidi-openid4vp 0.1.3`, and
  `affinidi-oid4vc-core 0.1.8` come from `66120dd70`; they are Apache-2.0 and
  published, but require Rust 1.95 and the minimal combined probe resolved 107
  external packages. Its OID4VP surface is Presentation Exchange-oriented,
  not a narrow DCQL engine.
- [Equs `equs-oid4vci 0.1.0`](https://github.com/equs-ai/oid4vci-rs) comes from
  revision `0ad382cee`; it is Apache-2.0 OR MIT,
  has no declared MSRV, and the minimal probe resolved 534 packages through
  broad SSI, mdoc, and OAuth-alpha dependencies.
- [Credibil `vdc`](https://github.com/credibil/vdc) at revision `288e2dcb`
  requires Rust 1.90 but relies on unpublished
  private-registry packages and has been inactive since 2025-09-30.
- [`siros-dcql 0.3.0`](https://crates.io/crates/siros-dcql/0.3.0) comes from
  [tag/revision `d4eeff53d`](https://github.com/sirosfoundation/siros-dc-matcher/tree/d4eeff53d24c9bf8e3fe50aecba2a554b2618961);
  it is BSD-2-Clause,
  declares Rust 1.82, denies unsafe, and depends only on serde and serde_json.
  The separately locked probe resolved 12 external registry packages and
  compiled with the SDK's Rust 1.89 MSRV host compiler.
- `oauth2 5.0.0` remains the authorization/PKCE oracle under ADR 0102.

The exact version and feature surface for the SIROS probe is
`siros-dcql = "=0.3.0"` with its only available default surface; the crate
declares no optional features. Crates.io plus the tagged VCS revision establish
license and provenance for the immutable candidate.

## Candidate decisions

| Layer/candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Existing `identus-oid4vci` | `retain-local` | Final-profile bounded behavior already exists; frameworks add coupling without proven replacement parity. | A released candidate passes the SDK conformance matrix, bounds, targets, errors, and demonstrates meaningful code/risk deletion. |
| Impierce, Spruce, Affinidi | `oracle` | Valuable current behavior; release, MSRV, cone, and/or model coupling prevent production adoption. | A narrow published core passes the standard dependency gates behind Identus-owned types. |
| Equs and Credibil | `not-adopt` | Excessive or unavailable dependency graph and immature provenance. | Published cohesive modules with public reproducible dependencies and target evidence. |
| OID4VP orchestration | `retain-local` future facade | Consent, trust, transport, nonce, verification, and lifecycle policy are SDK responsibilities. | Never delegate public policy; private closed mechanics remain independently reusable. |
| `siros-dcql 0.3.0` | `spike` | Narrow, current, pure Rust, tested, but inputs/results/errors are unbounded and validation is intentionally tolerant. | Adopt privately only if the bounded adapter proves semantic fit and target/cone evidence. |

## Compatibility and dependency evidence

Public and wire compatibility remains unchanged because the fixture is a
nested workspace with its own lock, and no candidate type enters an SDK crate
or facade boundary. Rollback deletes research assets without migration.

For the exact SIROS version, the direct dependency cone is `serde` and
`serde_json`; the separately resolved dependency cone contains 12 external
registry packages. The minimal Affinidi and Equs probes resolved 107 and 534
external packages respectively. These counts describe isolated manifests, not
incremental workspace deltas. The SIROS probe compiled on the Rust 1.89 MSRV
host. WASM, iOS, and Android target evidence remains unrun at planning time
because those targets and repository Nix were unavailable in the invoking
shell; no portable support is inferred.

The fixture selects no HTTP, runtime, clock, RNG, native library, or build
script. It cannot change consumer APIs or release artifacts. Its bounded
adapter is the only candidate-facing facade and no candidate-owned string,
vector, JSON value, error, or result may escape it.

## Security, privacy and maintenance evidence

The exact SIROS crate has no HTTP/runtime/native dependency and no authored
unsafe. It provides a credential trait, caller-supplied policy, query
execution, bounded combination enumeration, and extensive spec-derived tests.
That cohesion is materially better than a full framework.

Its public `from_json(&str)` has no byte/depth/member limits; public models use
unbounded `String`, `Vec`, and `serde_json::Value`; `Debug` and `Display` can
include verifier-controlled identifiers. Validation intentionally permits
identifier characters outside the OpenID4VP grammar and defaults missing
`meta`, because its matching philosophy is tolerant. Those are compatible
only behind an Identus-owned pre-parse bound, stricter validation, redacted
errors, and private type mapping.

Supply-chain evidence includes the exact crates.io release, generated lock,
published checksum, license metadata, and source scan; advisory and license
gates will run once the fixture exists. Maintenance evidence is a 0.3.0 release dated
2026-09-21 from an active tagged repository; this is promising, not a security
or longevity guarantee.

## Rejected or deferred candidates

Full Impierce, Spruce, and Affinidi frameworks are deferred as production
dependencies and retained as oracles. Equs and Credibil are rejected for now.
Replacing `identus-oid4vci` is rejected because no candidate has established
bounded Final-profile parity with lower coupling. Building a complete OID4VP
framework before a public facade and consumer states are specified is also
deferred. Each candidate decisions table row contains its objective
reconsideration trigger.

## Open questions and blockers

There is no research-readiness blocker. The executable fixture must establish
whether strict identifier/meta validation can be layered without duplicating
most of the engine and whether result mapping stays small. Production adoption
remains blocked on a separate OID4VP consumer issue, portable target evidence,
and an accepted public facade.

## Evidence commands

Planned exact locked evidence:

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 test --locked
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 tree --locked -p siros-dcql --edges normal,build
cargo +1.89.0 check --locked
rg 'unsafe|extern "C"|#\[link' <siros-dcql-0.3.0>/src
```

Unrun checks must include any unavailable WASM/iOS/Android compilation,
browser/mobile runtime, interoperability, performance, publication, release,
certification, presentation construction, and cryptographic verification.
