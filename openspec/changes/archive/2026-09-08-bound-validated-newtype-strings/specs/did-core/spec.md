## ADDED Requirements

### Requirement: Bounded standalone DID method value

The DID Core capability SHALL provide an immutable owned `DidMethod` value for
the W3C method-name grammar. It SHALL accept non-empty lowercase ASCII letters
and digits up to `MAX_DID_METHOD_BYTES`. That public byte ceiling SHALL equal
`MAX_DID_BYTES` minus the fixed `did:` prefix, method separator, and minimum
one-byte method-specific identifier, so every method name representable in a
maximum accepted bare DID remains representable standalone. Validation SHALL
check the byte ceiling before traversing method characters and SHALL NOT retain
or render rejected caller input in validator-produced errors.

#### Scenario: Exact standalone method ceiling is accepted

- **WHEN** a method name contains exactly `MAX_DID_METHOD_BYTES` valid ASCII bytes
- **THEN** borrowed, owned and serde construction SHALL accept it
- **AND** it SHALL remain the method of a syntactically valid `MAX_DID_BYTES` bare DID with a one-byte method-specific identifier

#### Scenario: One-over method is rejected before grammar traversal

- **WHEN** a method name exceeds `MAX_DID_METHOD_BYTES`, including one whose later byte also violates the grammar
- **THEN** every validated construction path SHALL reject it as over-limit
- **AND** validator-produced `Debug`, `Display`, serde and stable error output SHALL NOT contain caller-controlled rejection text

#### Scenario: Existing method consumers remain compatible

- **WHEN** an accepted method is used by the immutable method registry or a DID Registration request
- **THEN** existing exact-method dispatch and request validation behavior SHALL remain unchanged
