# oid4vp-dcql-query Specification

## Purpose
TBD - created by archiving change adopt-bounded-oid4vp-dcql. Update Purpose after archive.
## Requirements
### Requirement: DCQL extraction is a consuming bounded transition

The SDK SHALL consume a signature-verified Request Object into an explicitly
DCQL-valid state only after extracting one bounded `dcql_query`, rejecting a
simultaneous `scope`, and validating complete Final-profile structure.

#### Scenario: one valid query is present

- **WHEN** a verified payload contains one bounded object-valued `dcql_query` and no `scope`
- **THEN** the transition returns a DCQL-valid state without claiming full request validity

#### Scenario: query selection is ambiguous or unsupported

- **WHEN** `dcql_query` is missing/non-object or `scope` is also present
- **THEN** the transition fails with a static category before engine execution

### Requirement: The Identus facade closes candidate tolerance gaps

The SDK SHALL enforce required/non-empty collections, identifier grammar and
uniqueness, references, path/value shapes, and every configured bound before
invoking the private engine.

#### Scenario: tolerated candidate input violates Final

- **WHEN** metadata is missing, an identifier contains a dot, a collection is empty, or a reference is unknown
- **THEN** the SDK rejects the query even if the private candidate would accept it

#### Scenario: work would exceed policy

- **WHEN** query, credential, claim, path, value, set, candidate, or product work exceeds a limit
- **THEN** evaluation fails before unbounded engine work or result allocation

### Requirement: Private engine types cannot escape

The SDK SHALL use exact `siros-dcql 0.3.0` only behind Identus-owned public
credential, path, limit, error, query, and result types.

#### Scenario: a valid inventory is evaluated

- **WHEN** bounded credentials satisfy holder-binding, format, and claim rules
- **THEN** the result reports bounded per-query matches and capped satisfying combinations in deterministic order

#### Scenario: a caller resolver fails

- **WHEN** claim resolution reports missing, malformed, or type-mismatched data
- **THEN** the engine receives the corresponding private path outcome and public errors remain static

### Requirement: DCQL evidence is redacted and narrowly claimed

The SDK SHALL keep verifier query content, credential identifiers, claim values,
and candidate diagnostics out of public diagnostics and SHALL NOT claim request
validity, verifier trust, credential authenticity, consent, or response safety.

#### Scenario: hostile identifiers and values fail

- **WHEN** verifier or credential canaries appear in invalid input or resolver failures
- **THEN** `Debug`, `Display`, errors, metadata, and metrics expose only static categories and bounded counts

