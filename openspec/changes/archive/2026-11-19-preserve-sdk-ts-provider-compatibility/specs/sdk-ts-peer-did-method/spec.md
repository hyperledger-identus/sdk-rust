## ADDED Requirements

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
