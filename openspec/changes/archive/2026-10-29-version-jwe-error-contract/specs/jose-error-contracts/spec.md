# jose-error-contracts delta

## ADDED Requirements

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
