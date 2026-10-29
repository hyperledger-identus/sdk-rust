# jose-error-contracts Specification

## Purpose
TBD - created by archiving change decompose-jose-error-contracts. Update Purpose after archive.
## Requirements
### Requirement: JOSE owns a multi-kind private error contract

`identus-jose` SHALL own a crate-private compile-time record containing exactly
stable code, error kind, and one static message. The `jose` capability SHALL
remain centralized. No dependency, feature, allocation, unsafe/native code,
serialization, FFI, retryability, public API, or protocol behavior SHALL be
introduced.

#### Scenario: varying kind is explicit without repeating invariants

- **WHEN** any current JOSE error is converted
- **THEN** its record supplies code/kind/message and shared conversion supplies capability

### Requirement: Catalogue ownership follows JOSE responsibilities

Records SHALL be grouped privately into compact/header,
algorithm/key/registry/signing, proof/key/evidence, and
proof/policy/time/replay catalogues with counts 15, 11, 16, and 9 and every
current variant in exactly one group.

#### Scenario: one responsibility is reviewable alone

- **WHEN** a reviewer inspects proof policy and replay errors
- **THEN** all nine records are visible without reading the other catalogues

### Requirement: Routing and inventory are compile-exhaustive

One private wildcard-free router SHALL map every `JoseError` variant to exactly
one record. The same private list SHALL produce only its test inventory and
SHALL NOT generate a public declaration. No code, kind, or message literal
SHALL occur in the router.

#### Scenario: an unmapped variant fails

- **WHEN** a variant is added without a record/router entry
- **THEN** compilation fails rather than inheriting a default kind

### Requirement: Exact pre-refactor behavior is immutable

The compatibility oracle SHALL be a planning golden captured from
`develop@c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1` with exactly 51 unique
ordered rows. It SHALL pin constant identity/visibility, code, kind, capability,
local/public/full display and source. Stable and planning copies SHALL be
byte-identical, fixed-hash, and receipt-blob bound. Ambiguity, absence,
mutation, path escape, or symlinked components SHALL fail closed.

#### Scenario: the uncharacterized edge is pinned

- **WHEN** `SizeOverflow` is converted and displayed
- **THEN** its exact public code, `InvalidInput` kind, messages, and empty source match the golden

### Requirement: Public and target boundaries remain unchanged

The public JOSE error surface SHALL remain exact, including enum order,
derives, non-exhaustive marker, all public constants/paths, `From`, `Display`,
`Error`, and `pub const fn to_identus_error`. Direct target compilation is
evidence only and SHALL NOT promise runtime/device/package/FFI/binding support.

#### Scenario: compatibility diff is empty

- **WHEN** base and head public/dependency inventories are compared
- **THEN** no public item, method constness, manifest, feature, dependency or lockfile delta exists

### Requirement: Maintainability evidence is truthful

The change SHALL report largest router/function/catalogue, mapping sites,
wildcard defaults, decision count, and relevant total lines. It SHALL retain
51 behavioral rows, reduce mapping sites from two to one and wildcard defaults
from one to zero, and cap catalogues at 16 rows without claiming behavioral
decision compression.

#### Scenario: cohesion improves without false compression

- **WHEN** before/after evidence is reviewed
- **THEN** all 51 decisions remain explicit and total-line movement is disclosed

### Requirement: JWE errors own an independent immutable contract

`JweEncryptionError` SHALL own an independently versioned v1 compatibility
contract rather than extending or rewriting the `JoseError` historical golden.
The v1 contract SHALL contain exactly the eight ordered variants present at
`develop@c66d506690d3cd6f119f9ae6430b285f98115d10` and SHALL pin stable code,
kind, capability, local/public/full display, empty source behavior, and absence
of public code constants.

#### Scenario: existing JWE behavior is exact

- **WHEN** every v1 JWE error is converted, displayed, and inspected in order
- **THEN** every field matches the immutable eight-row golden exactly

#### Scenario: existing JOSE history remains independent

- **WHEN** the JWE contract is added
- **THEN** the existing `JoseError` fixture and its version policy remain byte-identical and unchanged

### Requirement: JWE routing and live inventory are exhaustive

One crate-private compile-time record SHALL contain exactly stable code, kind,
and static message. One wildcard-free route SHALL map every
`JweEncryptionError` variant to exactly one record. The shared `jose`
capability SHALL remain centralized, and no public constant or declaration
SHALL be generated.

#### Scenario: an unmapped variant fails compilation

- **WHEN** a new JWE error variant lacks an explicit route
- **THEN** compilation fails rather than inheriting a default contract

#### Scenario: all codes are stable and unique

- **WHEN** the complete live inventory is checked
- **THEN** every variant and code is unique and the first eight entries equal the v1 ordered prefix

### Requirement: JWE diagnostics remain static and redacted

The SDK SHALL keep every JWE local display, public message, full SDK display,
`Debug`, and source projection independent of key, plaintext, ciphertext,
compact JWE, state, engine diagnostic, or other caller-controlled private data.

#### Scenario: private canaries cannot cross the boundary

- **WHEN** every v1 diagnostic projection is searched for representative private canaries
- **THEN** no canary appears and every source remains empty

### Requirement: JWE contract hardening is behavior neutral

The contract slice SHALL NOT change the public enum, variant order, method
constness, conversions, manifest, features, dependencies, lockfile, algorithms,
runtime results, OID4VP wire behavior, FFI, or supported targets.

#### Scenario: release compatibility is reviewed

- **WHEN** exact base and head API/dependency inventories are compared
- **THEN** only private implementation and test/documentation evidence differ

### Requirement: future JWE additions preserve the v1 prefix

Future additive variants SHALL append after the complete v1 live inventory and
receive feature-owned exact contract tests. The v1 fixture SHALL remain
immutable. Any change to a v1 row SHALL require a new compatibility decision
and versioned evidence.

#### Scenario: an additive error is introduced later

- **WHEN** a new JWE error is added without changing an existing category
- **THEN** the original eight entries remain exact and ordered while the new suffix is tested independently
