# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation is the protected `develop` tree at
`c622268f7839d0636155576b0fe3984b6954404f`.
`identus-did` owns bounded generic DID syntax, documents, and resolver ports.
`identus-crypto` owns bounded public JWK values and Ed25519, X25519, P-256, and
secp256k1 public-key material. `identus-did-peer` and `identus-prism-did` prove
that independently selectable method crates can reuse those foundations without
making the generic DID crate depend on a method.

No SDK crate currently implements `did:key`, `did:jwk`, or `did:web`. The
existing `identus-did-resolver-http` is a server binding and is directionally
unrelated to `did:web` client retrieval. Private multicodec parsing inside
`identus-did-peer` is not sufficient evidence for a shared public abstraction.

## Normative sources

| Surface | Pinned revision | Role |
| --- | --- | --- |
| W3C CCG did:key | `2cc490c38c5aacf58497a87fc0cf8794668bf716` | normative draft and generation/resolution algorithms |
| did:jwk method | `44e186cf4cc215fc726afeecad1727cc9f9a66e8` | normative method syntax, document projection, and `use` relationships |
| W3C CCG did:web | `ea423c114e6f2537498ee6f94e8d794c64f60c18` | normative URL transformation and document binding |
| Spruce SSI | `886a6cf694dfdb6b019d5f69a04dd16f01cb58a4` | independent implementation oracle |
| DIF did-key.rs | `eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf` | historical did:key oracle |
| Impierce did-manager | `183f3eb061938404060d75817de53dd99adb18b0` | independent DID-method oracle |

Primary source URLs are
https://github.com/w3c-ccg/did-key-spec/tree/2cc490c38c5aacf58497a87fc0cf8794668bf716,
https://github.com/quartzjer/did-jwk/tree/44e186cf4cc215fc726afeecad1727cc9f9a66e8,
and
https://github.com/w3c-ccg/did-method-web/tree/ea423c114e6f2537498ee6f94e8d794c64f60c18.

The detailed source links, profile differences, dependency probes, and evidence
limits are retained in
`docs/research/did-methods/remaining-method-disposition.md`.
All normative and oracle repositories identify Apache-2.0 license evidence at
the pinned revision. No source or fixture is copied, so provenance in this
change is limited to immutable URLs, revisions, retrieval date, and observed
manifest metadata. Any later adaptation must add file-level license,
provenance, transformation, and expected-result records.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| SDK-owned narrow method crates | current SDK foundations | `adopt` | Preserves bounded inputs, redacted SDK errors, target policy, and method independence with no new production dependency. | A maintained crate matches the exact profile and materially lowers lifecycle cost. |
| Spruce narrow DID crates | source `886a6cf` / published 0.x crates | `oracle` | Mature behavior, but broad SSI/JSON-LD/JWK cones and profile choices do not match the selected SDK contracts. | A narrowly feature-gated release proves equivalent bounds, profiles, and targets. |
| DIF `did-key` 0.2.1 | `eb00da60` | `oracle` | Useful historical vectors, but archived and based on old crypto generations with no declared MSRV. | Project resumes with current crypto and target evidence. |
| Impierce did-manager | `183f3eb0` | `oracle` | Broad IOTA, Stronghold, Tokio, and forked-SSI coupling exceeds a method crate's responsibility. | A standalone method package is published and independently qualified. |
| Shared public multicodec crate now | local extraction | `not-adopt` | One private peer implementation and the did:key profile do not yet prove identical public semantics. | Two delivered methods require the same validated behavior and errors. |
| Concrete HTTP client in `identus-did-web` | runtime dependency | `not-adopt` | Would bind portable method semantics to executor, TLS, redirects, DNS, proxy, and SSRF policy. | A separate adapter issue names a runtime and support matrix. |

## Compatibility and dependency evidence

The selected planning contract changes no public Rust, wire, error, ABI,
persistence, target, feature, MSRV, or release behavior. The future method
crates are additive and independently selectable. Generic `identus-did` and
`identus-crypto` remain dependencies; they do not depend on method crates.

The exact assessed published versions are Spruce `did-method-key 0.5.0`,
`did-jwk 0.4.0`, `did-web 0.5.1`, and DIF `did-key 0.2.1`. Their inspected
features and upstream target evidence do not prove the SDK's supported target
matrix. Spruce's published method manifests do not declare an MSRV; DIF
`did-key` also declares no MSRV; Impierce declares Rust 1.75 for its broad
workspace. The SDK remains governed by its existing Rust 1.89.0 MSRV and Rust
1.98.1 primary compiler.

Temporary all-target Cargo metadata probes resolved 295 packages for Spruce
`did-method-key` with default features disabled, 286 for Spruce `did-jwk`, 333
for Spruce `did-web`, and 117 for DIF `did-key`. These are resolved graph sizes,
not compiled closures, but they demonstrate disproportionate structural cost
against the zero-new-production-dependency design. Exact probes are research
evidence only and are not imported into the repository.
This is the direct and resolved dependency cone comparison. Manifest/source
inspection found reachable unsafe and native-code risk in transitive broad
stacks rather than a justified method-local requirement; the selected
zero-new-dependency design adds neither. A child must repeat `cargo deny`,
RustSec, license, source, unsafe, and native inspection as supply-chain
evidence before adopting any candidate.

Public and wire compatibility remain unchanged by planning. Future public
types and errors stay behind an Identus facade; no third-party public type is
re-exported. Rollback is removal before merge or an explicit superseding ADR
after merge because this packet creates no runtime state.

The first `did:key` profile selects base58-btc `z`, Multikey projection,
Ed25519/X25519/P-256/secp256k1 public keys, and direct X25519 agreement. It
defers implicit Ed25519-to-X25519 derivation, base64url identifiers, P-384,
BLS, RSA, private material, and custody. The current draft's syntax prose also
mentions `u`, while its creation and decode algorithms require `z`; the strict
algorithm path is selected explicitly.

The first `did:jwk` profile uses the SDK's existing public-only JWK boundary,
unpadded base64url of UTF-8 JSON, `JsonWebKey2020`, and fragment `#0`.
`use=sig`, `use=enc`, and absent `use` control verification relationships as
specified. Resolution retains the input DID and does not claim one-key/one-DID
uniqueness because JSON serialization is not canonical.

The first `did:web` profile maps an FQDN and optional encoded port/path to HTTPS,
uses `/.well-known/did.json` for a bare domain and `/path/did.json` otherwise,
rejects IP hosts, and requires exact document-ID equality. Retrieval is a
caller-injected bounded effect. Concrete DNS, TLS, redirect, cache, retry,
proxy, executor, and SSRF policy remains outside the method core.

## Security, privacy and maintenance evidence

All future untrusted identifiers, decoded key bytes, JWK JSON, response bytes,
redirects, and document structures require explicit limits under `SDK-SEC-003`.
Private JWK members fail closed. Errors remain redacted and must not echo key
material or fetched bodies. The static methods create no network, filesystem,
clock, entropy, storage, secret-provider, or global-registry effect.

The `did:web` port grants the smallest retrieval authority and returns bounded
bytes plus transport metadata; it does not decide trust. An adapter owns host
resolution, private-network rejection, TLS, redirect, time, retry, proxy, and
cache policy. E2E evidence must use a deterministic in-process fetch adapter
before any real-network adapter can be proposed.

Maintenance is separated by method and specification revision. Property tests,
negative vectors, fuzz targets, target builds, benchmarks, and differential
oracles are owned by each child issue. No third-party type crosses a future
public SDK boundary.

The maintenance, release and security posture is deliberately pre-release:
the child crates remain unpublished until their normal release train qualifies
them. Protocol/draft currency is pinned rather than floating; every child must
compare the then-current draft with the revision above and record any semantic
change before implementation.

## Rejected or deferred candidates

Umbrella SSI facades, SDK-TS DTO/error parity, private-key custody, automatic
key derivation, unsupported curves, JSON-LD processing, real-network clients,
global resolver registries, consumer adoption, bindings, publication, and
release are deferred or rejected from this planning slice.

## Open questions and blockers

None for planning. Each implementation child must re-check the pinned draft,
crate releases, dependency advisories, target matrix, and fixture licenses at
its own preflight. A normative draft change is a profile-review trigger, not an
automatic behavior upgrade.

## Evidence commands

Research commands used `git ls-remote`, `git show`, `rg`, `sed`, and disposable
`cargo metadata --all-features --filter-platform` probes against the immutable
revisions above, plus the current SDK dependency and API inventory. Planning
commands are `scripts/factory doctor`, `scripts/factory research-ready
specify-remaining-portable-did-methods`, `scripts/factory constraints-ready
specify-remaining-portable-did-methods`, `scripts/factory validate
specify-remaining-portable-did-methods`,
`scripts/check-ssi-upstream-backlog.py`, `scripts/factory check`, and
`scripts/factory preflight specify-remaining-portable-did-methods --issue 555
--write`.

Unrun checks are Rust build/test/Clippy/docs, conformance, fuzz, benchmark,
WASM/mobile targets, production E2E, hosted PR CI, and release evidence. They
are not applicable to this behavior-neutral planning packet and belong to
issues #557–#559 before those capabilities can be claimed.
