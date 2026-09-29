# Exact-diff review

## Scope reviewed

- Base: `e15314797408b5d7904b5075081dadb144254276`
- Implementation: `738a7d993a8fbd415729bad492fe5e14dc032630`
- Production diff: `crates/presentations/src/model/selection.rs`
- Characterization diff: `crates/presentations/tests/presentation.rs`
- Planning and evidence: this OpenSpec change

## Architecture and cohesion

The public constructor remains the aggregate boundary: it rejects invalid
collection cardinality before any other work, revalidates the candidate set
against the exact request, and retains the owned result. The private validator
now owns only cross-object correlation. Its phase methods each have one reason
to change: duplicate identity, one selection's query/candidate/claim integrity,
or request-wide coverage. It neither exposes a reusable public abstraction nor
couples format-specific proof generation into the model.

Finding: no blocking architecture or cohesion issue.

## Behavioral and security review

The implementation preserves every return site and loop order from the former
constructor. The characterization matrix deliberately combines faults so a
reordering cannot pass by merely retaining individual error coverage. The
validator borrows existing bounded inputs, performs no I/O, accepts no remote
callback, emits no diagnostics, and retains no additional data. Error variants
remain static and redacted through the existing presentation error contract.

Finding: no blocking behavior, resource, privacy, or security issue.

## Rust review

The lifetime belongs on the private validation owner and expresses that no
state escapes construction. The extraction adds no allocation, clone, trait
object, generic bound, unsafe block, or dependency. Methods return early with
the existing typed errors, and the coordinator remains readable without a
helper chain that obscures ownership.

Finding: no blocking Rust correctness or maintainability issue.

## Residual limitations

- Duplicate and coverage checks intentionally remain bounded linear scans; the
  public limits make extra indexing state unjustified for this refactor.
- The unchanged `validate_candidates` function retains its pre-existing
  cyclomatic attention signal. It is outside this exact constructor boundary
  and is not claimed as an improvement.
- Canonical code-health evidence must be rebound after protected squash merge.

## Decision

Local review passed. The exact diff is suitable for protected implementation
delivery, followed by a distinct canonical-evidence closeout.

