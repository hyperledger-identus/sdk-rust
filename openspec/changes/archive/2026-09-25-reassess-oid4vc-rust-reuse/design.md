# Context

`identus-oid4vci` implements bounded OID4VCI 1.0 Final states. The repository
does not yet expose an OID4VP engine. Full Rust OID4VC implementations couple
protocol models to HTTP runtimes, broad SSI stacks, git dependencies, or a
higher MSRV. `siros-dcql` is a much narrower pure-Rust query and selection
engine, but accepts unbounded JSON and intentionally tolerates some invalid
identifier and metadata shapes.

# Goals

- Decide separately for OID4VCI, OID4VP orchestration, and DCQL mechanics.
- Test a narrow candidate through its public API without changing release
  graphs.
- Prove that resource limits and diagnostic redaction can remain SDK-owned.
- Preserve a cheap rollback and an objective future adoption gate.

# Decisions

## Keep the experiment outside the workspace

The fixture has its own exact manifest and lock. An explicit checker runs it;
ordinary workspace builds and release packaging do not discover it.

## Put a bounded adapter in front of candidate parsing

The experiment rejects an oversized query before `DcqlQuery::from_json`,
maps every candidate error to a small local category, and returns only
SDK-shaped summaries. It deliberately tests the candidate's tolerant missing
`meta` and identifier grammar so the mismatch is visible rather than silently
accepted as future policy.

## Decide per layer

Full OID4VC frameworks, OID4VCI replacement, OID4VP orchestration, and DCQL
selection are independent decisions. A successful DCQL spike cannot activate
the complete framework or define public SDK wire types.

# Compatibility matrix

| Surface | Effect |
| --- | --- |
| Root workspace, locks, releases | Unchanged |
| `identus-oid4vci` API and wire behavior | Unchanged |
| OID4VP public API | Not created |
| Research fixture | Exact locked candidate and bounded adapter |
| Fast CI | Unchanged; explicit research checker only |
| Portable targets | Recorded only when actually compiled |

# Rollback

Delete the isolated fixture, script, ADR, and research capability. No consumer
or migration is affected.
