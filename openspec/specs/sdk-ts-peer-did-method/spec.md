# sdk-ts-peer-did-method Specification

## Purpose
TBD - created by archiving change add-sdk-ts-peer-did-method. Update Purpose after archive.
## Requirements
### Requirement: Peer DID profiles are explicit at every public boundary

The generated and TypeScript APIs SHALL expose distinct numalgo-4 and
numalgo-2-compatibility operations and values. They SHALL NOT accept a generic
algorithm selector, silently dispatch between profiles, or accept numalgos 0,
1, 3, or an unknown form.

#### Scenario: an unsupported peer DID is supplied

- **WHEN** a caller supplies a numalgo 0, 1, 3, unknown, or cross-profile DID
- **THEN** the selected explicit operation fails with a stable redacted
  profile/algorithm error before proportional decoding or allocation

### Requirement: Numalgo 4 remains the current deterministic profile

Numalgo-4 creation SHALL accept a bounded root input document without `id`,
produce a verified long form and derived short form, and locally project the
contextualized immutable DID Document. Parsing SHALL verify codec, multihash,
encoded document, secret-member exclusion, and projection constraints.

#### Scenario: a bounded input document is created

- **WHEN** a valid public numalgo-4 input document crosses the TypeScript/WASM
  boundary
- **THEN** Rust returns the deterministic long form, derived short form, and
  immutable locally resolved document with no host effect or secret custody

### Requirement: Numalgo 4 short resolution requires matching caller material

The short-form operation SHALL resolve only with a caller-supplied long form
whose derived short form matches. The facade SHALL NOT fetch, search, cache, or
silently use unrelated material.

#### Scenario: the supplied long form does not match

- **WHEN** short-form resolution receives absent or non-matching long-form
  material
- **THEN** it fails with the stable long-form-required or hash-mismatch code,
  produces no partial document, and performs no I/O or fallback

### Requirement: Numalgo 2 is an explicit public-material compatibility route

Numalgo-2 compatibility creation SHALL accept only bounded caller-owned public
multibase keys with explicit purposes and bounded compact service inputs.
Parsing/resolution SHALL preserve the accepted deterministic identifier,
relationship, key-ID, and service behavior without weakening numalgo 4.

#### Scenario: the pinned donor public material is supplied

- **WHEN** fixed Ed25519/X25519 public keys and DIDCommMessaging service values
  from the immutable compatibility packet are created and resolved
- **THEN** the exact accepted numalgo-2 DID and immutable DID Document fields
  match the source-bound compatibility evidence

### Requirement: Secret and platform responsibilities never cross the facade

The facade SHALL NOT accept private JWK members, private keys, seeds,
mnemonics, entropy, signing providers, custody handles, storage, network,
executor, mediator, relationship, or agent policy. Public inputs SHALL be
validated structurally by TypeScript and canonically by Rust.

#### Scenario: secret-shaped input is attempted

- **WHEN** a structured input contains private material or an unsupported
  public purpose/codec
- **THEN** validation fails before retention and no input bytes appear in
  errors, observations, generated values, or logs

### Requirement: Inputs and projections remain finite and lossless

TypeScript SHALL reject cycles, prototype keys, unsafe integers, unknown
fields, and structural budgets before closed serialization. Rust SHALL enforce
its public byte/work/depth/node/member/key/service/element/document bounds and
method invariants. The public projection SHALL never silently round or drop a
validated value.

#### Scenario: an exact resource boundary is crossed

- **WHEN** an input is accepted at the declared limit and rejected one unit
  beyond it
- **THEN** both routes behave deterministically, the over-limit failure is
  redacted, and no partial/generated owner escapes

### Requirement: Public values are immutable and reuse canonical DID models

The package SHALL expose strict frozen plain peer profile, DID, and DID
Document values. It SHALL reuse the existing DID Document projector and SHALL
NOT expose generated WASM classes/memory, raw ownership, initializer details,
historical component names, or explicit public `any`.

#### Scenario: local resolution succeeds

- **WHEN** Rust returns a validated profile result
- **THEN** TypeScript copies it into the reviewed immutable profile value,
  reuses canonical DID Document projection, and frees every generated owner
  exactly once before returning

### Requirement: Stable failures preserve profile distinctions without data

The adapter SHALL map wrong profile, unsupported algorithm, malformed
encoding, unsupported codec/purpose, invalid input document, hash mismatch,
long-form required, resource limit, projection failure, and binding drift to
closed constant errors. Errors/observations SHALL NOT contain caller material,
generated messages, or stacks.

#### Scenario: generated error identity drifts

- **WHEN** a generated value or error contains an unknown API version, field,
  shape, or code
- **THEN** projection fails closed as a binding-contract error after exact
  ownership cleanup and does not reinterpret the value

### Requirement: Donor journeys and Rust additions are completely ledgered

Every discovered pinned SDK-TS peer source/test SHALL have immutable identity
and exactly one adapted, platform-owned, superseded, or excluded disposition.
Numalgo 4 SHALL be recorded as Rust-owned addition/deviation evidence rather
than donor equivalence.

#### Scenario: a signature or agent test is discovered

- **WHEN** a donor journey mixes peer identifiers with key generation,
  signature verification, storage, mediator updates, or DIDComm execution
- **THEN** only the method-owned deterministic outcome is adapted and the
  remaining behavior is assigned to its independent owner, not silently copied

### Requirement: A pairwise consumer shape does not import DIDComm semantics

One source-bound fixture SHALL create/parse two peer DID values and preserve
them in a frozen `{from,to}` request-shaped value. It SHALL NOT construct,
pack, route, send, store, or process a DIDComm message.

#### Scenario: peer values enter a pairwise request boundary

- **WHEN** two validated peer DID values are assigned to `from` and `to`
- **THEN** their exact serialized identities survive the immutable round trip
  while no messaging engine or protocol state is invoked

### Requirement: One conformance contract executes on applicable targets

One provider-neutral assertion body SHALL execute every active peer vector
unchanged through Node ESM, Node CommonJS, browser ESM, React SSR, and hydrated
React client providers. React SHALL reuse target providers and implement no
method semantics. React Native SHALL remain explicitly inapplicable.

#### Scenario: SSR output hydrates in the browser

- **WHEN** SSR uses the Node provider and the client hydrates through the
  browser provider
- **THEN** both observe identical normalized peer outcomes and a client-only
  signal proves real browser execution without inferring React Native support

### Requirement: Peer adapter quality evidence is source-bound and local-first

The train SHALL provide immutable vectors, strict declarations, deterministic
artifacts, negative/resource/differential/property/fuzz/mutation evidence,
target receipts, pairwise proof, and measurement-only performance results
bound to exact source. Existing core evidence SHALL be referenced, not
relabeled as TypeScript adapter evidence. Comparative measurements SHALL use
matched baseline and candidate artifact definitions and SHALL distinguish
import-inclusive cold and warm initialization from provider construction and
peer operations. Benchmark target labels SHALL name only measured runtimes.

#### Scenario: local evidence completes

- **WHEN** every applicable focused and full local gate passes
- **THEN** TS1 records only the exact proved private target state and retains
  external adoption, native binding, accepted budget, support, publication,
  default-switch, DIDComm, release, and mdoc gaps

#### Scenario: matched measurement evidence completes

- **WHEN** the peer-enabled candidate is measured against its exact pre-peer baseline
- **THEN** one revision-bound receipt records comparable compressed and uncompressed artifact bytes, import-inclusive cold and warm Node initialization, separate provider/operation measurements, samples, aggregation, environment, exact revisions, and limitations without activating a support budget

#### Scenario: functional target proof is recorded

- **WHEN** browser, Node.js, and React web private functional target receipts pass but adoption, rollback, and accepted-budget gates remain open
- **THEN** the parity ledger retains those receipts with partial target states and does not describe them as delivered support or measured browser/React performance

### Requirement: CI isolation and rollback remain fail-closed

Rust core SHALL complete before fail-closed impact planning and isolated
package/browser/node/react jobs. Relevant peer/core/binding/vector/mapping/
package paths SHALL select all applicable targets. Rollback SHALL select the
provider without peer-method capability before initialization; failures SHALL
never switch profile/provider automatically.

#### Scenario: one peer target fails

- **WHEN** Rust core succeeds but a selected target fails, times out, is
  cancelled, skipped, stale, or missing
- **THEN** core and sibling evidence remain independently visible while the
  terminal verdict fails and no target/support claim is made

### Requirement: Runtime and product expansion remains excluded

The package SHALL remain private, unpublished, opt-in, and non-default. It
SHALL NOT claim all peer numalgos, secret custody, DIDComm, networking,
storage, agent runtime, PRISM/Cardano, React Native/native evidence, external
SDK-TS adoption, stable API/support, certification, production use, release,
or mdoc work.

#### Scenario: all repository-local targets pass

- **WHEN** browser, Node.js, and React Web evidence is complete
- **THEN** the capability remains partial until separately authorized
  consumer, native-target, accepted-budget, support, publication, and release
  decisions are accepted

### Requirement: Core provider compatibility and peer capability selection are explicit

`DidProvider` SHALL retain the original DID, DID URL, and DID Document parsing
methods without requiring peer operations. Peer operations SHALL be exposed by
a strongly typed capability extension. Browser and Node SHALL select an
injected core provider or peer-WASM provider before generated artifact
initialization, and rejection after selection SHALL NOT trigger fallback.

#### Scenario: a core-only provider is selected

- **WHEN** a previously valid three-method provider is injected through a
  browser, Node, CommonJS, or React route
- **THEN** it remains assignable, the same provider is returned or injected,
  and the peer-WASM initializer is not invoked

#### Scenario: a selected peer provider rejects an operation

- **WHEN** the peer-WASM route initializes and a later Rust-backed operation
  rejects its input
- **THEN** the stable rejection is terminal and no injected or alternative
  provider silently replaces the selected provider

### Requirement: Peer DID evidence names the exact exercised boundary

Donor, fuzz, and differential evidence SHALL identify exact immutable source
selectors, transformations, executable entry points, target applicability,
and stable outcomes. Rust-core evidence SHALL NOT be relabeled as TypeScript
adapter evidence, and host-only preconditions SHALL NOT be attributed to the
native differential runner.

#### Scenario: donor input differs from the selected profile

- **WHEN** a pinned SDK-TS resolver fixture uses a bare service ID rejected by
  the selected Rust compatibility profile
- **THEN** the donor ledger names the exact selector and omitted input,
  links the adapted positive fixture and intentional negative vector, and does
  not describe the transformed fixture as byte-equivalent donor behavior

#### Scenario: adapter fuzz evidence is claimed

- **WHEN** the TypeScript quality declaration marks fuzz evidence satisfied
- **THEN** its selector enters adapter-owned ingress or projection with bounded
  generated cases while the Rust sanitizer campaign remains credited only to
  the Rust core

#### Scenario: native differential evidence executes

- **WHEN** the Rust runner consumes the shared peer packet
- **THEN** it compares exact identifiers, authentication/key-agreement
  references, service type/URI/routing/accept values, and applicable stable
  rejections while explicitly classifying TypeScript-only preconditions
