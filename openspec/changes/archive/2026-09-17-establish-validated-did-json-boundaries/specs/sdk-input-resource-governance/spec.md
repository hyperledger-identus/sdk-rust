## ADDED Requirements

### Requirement: Native DID JSON rejection cleanup is SDK-enforced after entry

The inventory and limitation index SHALL distinguish allocation before typed
SDK entry from cleanup after a native DID constructor takes ownership. Every
public native DID construction path that accepts already-owned recursive JSON
and can return validation failure SHALL dismantle rejected arrays and objects
iteratively across the document, resolution/dereferencing, query-option, and
registration families.

Accepted public DID domain objects SHALL retain recursive JSON only within the
existing SDK budgets. Bounded wire-slice parsers SHALL remain the preferred
hostile-byte boundary because cleanup cannot prevent earlier allocation.

#### Scenario: Native caller transfers hostile-depth JSON

- **WHEN** typed SDK construction takes ownership and validation rejects the
  recursive value
- **THEN** the SDK dismantles it iteratively without requiring a caller depth
  bound solely for rejection cleanup

#### Scenario: Allocation happened before SDK entry

- **WHEN** a caller or deserializer allocated hostile JSON before typed entry
- **THEN** the inventory and `SDK-LIM-007` continue to assign that allocation
  to the outer owner and do not represent cleanup as preallocation protection

## REMOVED Requirements

### Requirement: Native DID JSON rejection cleanup remains consumer-guarded

**Reason:** Issues #315 and #297 establish a validated context boundary and
iterative rejection cleanup across the complete public native DID family.

**Migration:** Native callers no longer need a depth bound solely to destroy a
rejected DID JSON tree. They still own allocation before SDK entry and should
use bounded wire-slice parsers at hostile byte boundaries.
