## MODIFIED Requirements

### Requirement: The Identus facade closes candidate tolerance gaps

The SDK SHALL enforce required/non-empty collections, identifier grammar and
uniqueness, references, paths/values, format identity, metadata capability,
and every configured bound before invoking the private engine.

#### Scenario: tolerated candidate input violates Final

- **WHEN** metadata is missing, non-empty metadata lacks an exact capability,
  an identifier contains a dot, a collection is empty, or a reference is unknown
- **THEN** the SDK rejects the query even if the private candidate would accept it

#### Scenario: work would exceed policy

- **WHEN** query, metadata, credential, claim, path, value, set, capability,
  candidate, or product work exceeds a limit
- **THEN** evaluation fails before unbounded engine or callback work
