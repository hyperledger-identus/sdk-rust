# native-did-bindings Specification

## MODIFIED Requirements

### Requirement: Native DID failures are bounded and redacted

The facade SHALL preserve the domain parser's 131,072-byte DID and
139,264-byte DID URL bounds. Invalid, oversized and unexpected-panic failures
SHALL return closed error cases with stable SDK-owned codes and SHALL NOT
return caller text, domain details, panic payloads or implementation
diagnostics.

#### Scenario: invalid input is rejected

- **WHEN** a Swift or Kotlin caller supplies an invalid or oversized DID or DID
  URL containing a canary value
- **THEN** the facade SHALL return the matching closed case and constant code,
  and every returned error representation SHALL omit the canary

#### Scenario: authored wrapper logic unexpectedly unwinds

- **WHEN** an internal wrapper operation panics before producing a value
- **THEN** the authored boundary SHALL catch the unwind and return only the
  constant internal error case/code without exposing the panic payload
