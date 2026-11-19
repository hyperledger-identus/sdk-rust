# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/603
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001` remains effective: portable Rust owns domain semantics and
  language/target adapters depend inward.
- `SDK-SEC-003` remains effective: untrusted serialized and structured input is
  bounded and validated before semantic use.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005` remain effective for
  exact Rust 1.99.1, mapping, target, and consumer evidence.
- `SDK-LIM-002` remains effective: the private facade is not a supported
  binding commitment.
- `SDK-LIM-003` remains effective: browser WASM and native targets are not
  runtime-certified.
- `SDK-LIM-009` remains effective: target failures preserve completed core
  evidence while the terminal `fast` verdict fails.
- ADR 0174, the canonical `peer-did-method` requirements, and TS1 SC-01 through
  SC-09 remain controlling.

## Introduced or changed constraints

The train adds one inward workspace dependency from `identus-wasm-did` to
`identus-did-peer`, explicit generated/profile exports, strict immutable
TypeScript inputs/results, cross-target evidence, and fail-closed impact
routing. It introduces no new third-party engine, secret boundary, transport,
storage, executor, or native dependency.

## Introduced or changed limitations

Numalgo 4 SHALL be the current route and numalgo 2 SHALL be named explicitly as
compatibility. The facade SHALL NOT infer a profile, accept an algorithm number
from the caller, or dispatch unsupported numalgos 0, 1, 3, or unknown forms.

Creation SHALL accept caller-owned public material only. Private JWK members,
private keys, entropy, seed/mnemonic values, signing, custody, storage, and
secret resolution SHALL NOT cross the boundary. TypeScript SHALL reject cycles,
prototype keys, unsafe numbers, unknown fields, and structural budgets before
closed serialization; Rust SHALL revalidate canonical byte/work/profile/
codec/purpose/private-member/hash/document constraints.

Numalgo-4 long-form creation/parsing and local resolution SHALL preserve the
derived short form. A short form SHALL resolve only with matching caller-
supplied long-form material. Numalgo-2 compatibility creation/parsing SHALL
preserve deterministic identifier, relationship, key-ID, and compact service
behavior accepted by the pinned profile. No operation performs I/O.

Generated values/errors SHALL be copied and freed exactly once. Public values
SHALL be frozen plain values and SHALL NOT expose generated types, WASM memory,
explicit `any`, caller material in diagnostics, or silently rounded integers.
Stable closed error codes SHALL preserve profile/resource distinctions without
embedding input data.

One provider-neutral assertion body SHALL run the active peer vector packet on
Node ESM/CommonJS, browser ESM, React SSR, and hydrated React client routes.
React SHALL reuse browser/node providers. React Native SHALL emit an explicit
inapplicable receipt until a separately qualified native binding exists.

Unknown binding impact SHALL select all target jobs. Selected target failure,
timeout, cancellation, skip, stale receipt, or missing result SHALL keep the
terminal verdict red without erasing independent Rust-core/sibling evidence.

The package remains private, opt-in, non-default, and unpublished. Local
measurements are directional and are not accepted support budgets. No local
result may claim external SDK-TS adoption, DIDComm migration, all peer numalgos,
React Native evidence, publication, stable API/support, certification,
production readiness, release, PRISM/Cardano ownership, or mdoc.

## Consumer and product impact

The consumer-visible addition is an explicit private peer DID method provider
for current numalgo 4 and named numalgo 2 compatibility. Existing DID syntax,
document, resolution, dereferencing, Rust callers, external SDK-TS, product
routes, and DIDComm behavior remain unchanged. The pairwise fixture proves only
frozen `from`/`to` values and local resolved documents, not messaging.

## Scope limitations

This train covers public-material creation, parsing, local resolution,
projection, target packaging, conformance, observations, measurements, and
rollback. It excludes PRISM/VDR/indexing, Cardano, numalgos 0/1/3, secrets,
crypto/signatures, network, persistence, DIDComm/agent behavior, native
bindings, consumer adoption, support/publication/release, and mdoc.

## Activation and rollback

Implementation begins only after research/constraint readiness, a signed+DCO
planning-only commit, and an exact issue #603 preimplementation receipt. Local
completion requires focused Rust/WASM/package/target/property/fuzz/
differential/resource evidence, full factory/Nix gates, distinct exact-diff
architecture/security review, and guarded archive evidence.

Rollback removes or disables the unreleased peer-method provider, mapping,
vectors, and assertions as one unit while preserving every existing DID facade
and Rust API. Provider selection occurs before initialization; failure never
silently changes profile or provider.

## Evidence

Readiness is proved by `research-ready`, `constraints-ready`, strict OpenSpec
validation, the exact issue #603 receipt, and the commands in `research.md`.
Target evidence remains local and partial until external consumer rehearsal,
accepted budgets, native-target, support, publication, and release decisions
are separately authorized.
