# did-core delta

## ADDED Requirements

### Requirement: Native owned-JSON rejection cleanup is iterative

Every public DID constructor or builder `build` path that takes ownership of caller-provided recursive JSON SHALL arm one internal cleanup guard before any validation branch.
A covered value includes a `serde_json::Value` or JSON map. If construction
fails, all owned arrays and objects SHALL be dismantled through one iterative
worklist without recursive destruction. Successful construction SHALL preserve
the exact accepted value, public API, serde representation, accessor behavior,
and stable redacted error taxonomy.

The covered family SHALL include verification methods, services and document
builders; resolution problems and metadata; dereferenced content and content
metadata; resolution/dereferencing query options and their builders; and
registration public data.

#### Scenario: Resource validation rejects hostile depth

- **WHEN** a native caller transfers a 32,768-level JSON tree to any covered
  constructor and existing depth validation rejects it
- **THEN** the constructor returns its existing redacted error and destroys the
  rejected tree iteratively without exhausting the process stack

#### Scenario: Another field fails before JSON validation

- **WHEN** a covered candidate contains hostile-depth owned JSON but an earlier
  non-JSON invariant fails first
- **THEN** the same guard dismantles every owned JSON root iteratively

#### Scenario: A bounded value is accepted

- **WHEN** the candidate satisfies the existing byte, node, depth, member,
  string, and semantic limits
- **THEN** the guard yields the unchanged original value and all public/wire
  behavior remains identical
