# sdk-input-resource-governance delta

## ADDED Requirements

### Requirement: Native DID JSON rejection cleanup is SDK-enforced after entry

The inventory and limitation index SHALL distinguish allocation that occurs in
a caller, serde, transport, FFI bridge, or language runtime before typed SDK
entry from cleanup after a native DID constructor takes ownership. Every public
native DID construction path that accepts already-owned recursive JSON and can
return a validation error SHALL dismantle the rejected tree iteratively across
the complete document, resolution/dereferencing, query-option, and
registration families.

Accepted retained values SHALL remain under the existing SDK depth, node,
member, collection, byte, string, and semantic limits. The bounded wire-slice
parsers SHALL remain the preferred hostile-byte boundary because iterative
rejection cleanup cannot retroactively prevent outer allocation.

#### Scenario: Native caller owns a hostile-depth JSON tree

- **WHEN** a caller transfers recursive JSON directly to a DID constructor and
  validation rejects it
- **THEN** the SDK dismantles the owned tree iteratively without requiring a
  caller depth bound solely for rejection cleanup

#### Scenario: Allocation happened before SDK entry

- **WHEN** a caller or deserializer already allocated hostile JSON before
  invoking the typed constructor
- **THEN** the inventory and `SDK-LIM-007` continue to assign that allocation
  to the outer owner without representing iterative cleanup as preallocation
  protection

## REMOVED Requirements

### Requirement: Native DID JSON rejection cleanup remains consumer-guarded

**Reason:** Issue #297 makes rejection cleanup iterative across the complete
public native DID constructor family. Retaining the former caller depth
obligation would misrepresent implemented SDK behavior.

**Migration:** Native callers no longer need a depth bound solely to destroy a
rejected DID JSON tree. They still own allocation before SDK entry and should
use bounded wire-slice parsers at hostile byte boundaries.
