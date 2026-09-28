# Context

`VerifiedRequestObject` is cryptographic evidence, not request validity. The
next transition must extract one DCQL query, close known candidate tolerance
gaps, bound worst-case work, and reuse the candidate engine without exporting
its models or diagnostics.

# Decisions

## Consuming query transition

`VerifiedRequestObject::into_dcql_query(limits)` consumes signature evidence,
parses the already bounded payload, rejects simultaneous `scope`, requires a
`dcql_query` object, serializes only that object under the query byte bound,
strictly validates it, and returns `ValidatedDcqlQuery`. The name deliberately
does not claim full Authorization Request validity.

## Strict SDK facade before private parsing

The facade walks `serde_json::Value` under independent collection and scalar
bounds. It enforces required/non-empty `credentials`, `meta`, path/value arrays,
identifier grammar/uniqueness, claims/claim-set and credential-set references,
and rejects non-empty `meta`/trusted authorities for the initial exact-format
policy. Unknown properties are ignored as Final requires, but still counted by
the existing complete-payload scanner.

Only after strict validation is the bounded query string passed to
`siros_dcql::DcqlQuery::from_json`. Candidate errors are collapsed to one
static invalid-query category.

## SDK-owned evaluation port and result

An object-safe `DcqlCredential` exposes bounded id/format text, holder-binding
evidence, and claim resolution over SDK-owned `DcqlPathComponent` values.
Private adapters implement the candidate trait and translate path/error types.
Evaluation performs checked work-budget products before the engine runs.

The SDK maps per-query candidates, selected claim ids/paths, satisfiability,
and capped combinations into Identus-owned results. It omits verifier `purpose`
from this selection-only slice. Custom metadata/trust policy and consent remain
later capabilities.

## Redaction and ownership

Retained query/engine state has manual count-only `Debug`. Outcome types also
use count/shape-only diagnostics. Credential and verifier identifiers are
available only through explicit sensitive accessors. No raw claim value is
retained in the outcome.

# Alternatives rejected

- Exposing candidate types couples semver, bounds, and diagnostics.
- Parsing with the candidate first permits known invalid/tolerated shapes.
- Implementing local path/set selection duplicates a focused reviewed engine.
- Adding custom format policy now broadens trust/format ownership.
- Treating DCQL validation as full request validity creates a dangerous state
  overclaim.

# Rollback

Remove the DCQL module, dependency, error variants, limits, ADR, and specs. The
existing signed-JAR transition remains intact.
