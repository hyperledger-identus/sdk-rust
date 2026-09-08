## Why

DID Registration opaque identifiers already have explicit 256-byte and
1,024-byte ceilings, but their borrowed parser clones the complete input before
enforcing those limits. Oversized untrusted input can therefore force an
attacker-sized rejection allocation despite the intended resource boundary.

## What Changes

- Validate borrowed registration identifiers before allocating their owned
  success value.
- Preserve existing grammar, byte ceilings, owned constructors, public APIs,
  wire behavior, and redacted diagnostics across all six generated types.
- Add exact/one-over, validation-precedence, allocation-order, constructor-path,
  and redaction evidence.
- Narrow the evidence behind `SDK-LIM-007` without claiming the repository-wide
  resource audit complete.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `did-core`: bounded DID Registration opaque identifier borrowed parsing
  validates before allocating the retained value.

## Impact

The change affects the private `opaque_identifier!` macro in `identus-did`, its
tests, the DID Core specification, ADR and resource-bound evidence. Accepted
input and every public signature remain unchanged. No dependency, feature,
target, wire, downstream, chain, product, or certification surface changes.
