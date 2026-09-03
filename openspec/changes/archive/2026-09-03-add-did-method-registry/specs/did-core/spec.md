## ADDED Requirements

### Requirement: Immutable bounded DID method registry

The DID Core capability SHALL provide a `DidMethodBinding` containing one
validated method name, one object-safe resolver and an optional independent
dereferencer. A builder SHALL reject duplicate method ownership and more than
64 bindings before producing a cheaply cloneable immutable registry whose
lookups require no lock or allocation.

#### Scenario: duplicate method ownership fails during setup

- **WHEN** two independent bindings claim the same exact validated DID method
- **THEN** registration SHALL fail deterministically before a registry can be
  built, without exposing the supplied method through the public error bridge

#### Scenario: a built registry is deterministic and concurrently readable

- **WHEN** registered methods are inspected or dispatched from cloned registry
  handles on concurrent threads
- **THEN** names SHALL appear in lexical order and exact lookups SHALL require
  no mutation, lock, runtime or chain-specific type

### Requirement: Resolution dispatch through the standard port

`DidMethodRegistry` SHALL implement `DidResolver` by selecting exactly the
registered binding whose method equals `Did::method()`. It SHALL forward the
original validated DID and options and return the adapter's existing W3C
result envelope without another generic error channel.

#### Scenario: independent DID methods share one resolver seam

- **WHEN** PRISM- and Midnight-shaped resolvers are registered and the registry
  is invoked through `Arc<dyn DidResolver>`
- **THEN** each exact DID method SHALL reach only its owner with unchanged
  options and result envelope

#### Scenario: unknown methods are standards-shaped failures

- **WHEN** no binding owns the exact requested DID method
- **THEN** resolution SHALL return a valid W3C `methodNotSupported` failure
  envelope without invoking a prefix, fallback or arbitrary adapter

### Requirement: Independent dereferencing dispatch

`DidMethodRegistry` SHALL implement `DidUrlDereferencer` independently. An
unknown method SHALL return `methodNotSupported`; a known method whose binding
has no dereferencer SHALL return `featureNotSupported`; and a registered
dereferencer SHALL receive the original DID URL and options unchanged.

#### Scenario: resolution-only methods remain usable

- **WHEN** a method binding supplies a resolver without a dereferencer
- **THEN** resolution SHALL operate normally while dereferencing returns a
  standards-shaped `featureNotSupported` result

#### Scenario: registered dereferencing routes exactly

- **WHEN** a binding supplies an independent dereferencer and its exact method
  is requested
- **THEN** the registry SHALL forward the validated DID URL and options and
  return its existing result envelope unchanged

### Requirement: Stable registry setup errors and observability

Duplicate ownership SHALL bridge to stable code `did.invalid_method_registry`
with `Conflict` kind. Capacity exhaustion SHALL use the same code with
`InvalidInput` kind. Registry count, emptiness, exact membership,
dereferencing-support and lexically sorted method names SHALL be observable,
while registered adapter objects SHALL remain encapsulated.

#### Scenario: registry setup errors are redaction safe

- **WHEN** duplicate ownership or capacity exhaustion is mapped to the shared
  SDK error surface
- **THEN** the error SHALL identify the DID registry capability and kind
  without containing a registered method name or adapter detail

#### Scenario: dispatch cost remains observable

- **WHEN** a representative multi-method registry is dispatched repeatedly in
  release mode
- **THEN** throughput SHALL be recorded as evidence without a machine-specific
  CI pass threshold
