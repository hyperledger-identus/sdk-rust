## Context

The milestone has two directions: NeoPRISM should consume existing generic SDK
components, while missing generic behavior may move from NeoPRISM into
sdk-rust. A single branch can obscure which repository owns a change and can
make local deletion appear to prove upstream quality. The design therefore
separates classification, SDK delivery, and downstream adoption.

## Goals / Non-Goals

**Goals:**

- make "reusable module" an objective architecture gate;
- preserve low coupling, high cohesion, orthogonality, and downward-only
  dependencies;
- prevent donor, chain, product, runtime, and policy types from becoming SDK
  public contracts;
- reuse existing SDK capabilities before extracting new code;
- preserve immutable provenance and historical behavior evidence; and
- make every extraction/adoption slice independently reviewable and
  reversible.

**Non-Goals:**

- move NeoPRISM source in this change;
- redesign all NeoPRISM crates at once;
- publish sdk-rust crates or merge NeoPRISM to `main`;
- claim production consumer adoption; or
- use module size, file count, popularity, or a numeric score as a proxy for
  architectural fitness.

## Decisions

### Use mandatory gates followed by qualitative review

Every candidate passes all hard gates. A failure produces `adapt`,
`conformance-only`, `remain-downstream`, or `reject`; it cannot be offset by a
different strength. After the gates pass, reviewers decide the cohesive crate
or module boundary using change axis, dependency cost, portability, and release
unit evidence.

### Separate source disposition from delivery action

`extract`, `adapt`, `conformance-only`, `remain-downstream`, and `reject`
describe how donor material may inform the SDK. The next action may instead be
to adopt an already existing SDK component downstream. This prevents an old
`extract` label from authorizing duplicate code after sdk-rust has implemented
the capability independently.

### Prefer capability units over donor repository structure

The extraction unit is the smallest independently useful standards or domain
capability with one primary responsibility and one dominant reason to change.
Donor package, module, and file boundaries are evidence only. Tiny helpers do
not become SDK modules unless they belong to a named cohesive capability.

### Keep effects and policy outside the reusable core

Pure domain behavior may depend only on lower generic SDK layers and reviewed
narrow engines. Network, filesystem, database, executor, clock, and entropy
effects use minimal injected ports in the core and may have separately
selectable adapter crates. Chain, custody, consent, trust, UI, and deployment
policy remains downstream. Public APIs expose Identus-owned types and redacted
errors.

### Require consumer-shaped proof without inventing adoption

Two independent consumer-shaped uses are the default evidence that a public
module is reusable. They can be conformance examples or adapters, but must use
only the public minimal feature surface. A foundational standards primitive may
document an exception with two credible usage paths. Neither form claims a
consumer repository has adopted the module.

### Use an SDK-first, downstream-second transaction

Each missing generic capability receives its own sdk-rust issue, OpenSpec
contract, implementation, tests, and immutable merged revision. A separate
NeoPRISM issue then refreshes the beta branch, pins that revision, adapts one
surface, runs compatibility evidence, and only then removes duplicate local
code. The two repositories never share an uncommitted path dependency.

## Candidate assessment record

Every implementation issue records:

1. candidate name, donor repository, exact SHA, paths, history, and license;
2. named SDK capability, owner crate/layer, primary responsibility, and change
   axis;
3. hard-gate result with evidence and final source disposition;
4. public API ownership and dependency graph for minimal and enabled features;
5. effect, policy, target, input-bound, error, secret, trust-state, unsafe, and
   native-code boundaries;
6. normative source and donor fixture/vector mapping;
7. two independent consumer-shaped proofs or the foundational exception;
8. compatibility, migration, immutable revision, rollback, and downstream
   deletion conditions; and
9. commands passed, commands not run, and unresolved blockers.

## Alternatives rejected

- A single cross-repository PR cannot provide atomic rollback and violates
  repository authority boundaries.
- A broad `identus-neoprism-common` crate preserves donor coupling rather than
  discovering domain capabilities.
- Re-exporting NeoPRISM or external framework types makes their release cycle
  an SDK public compatibility promise.
- Forking existing SDK modules to preserve NeoPRISM APIs creates two generic
  models; a thin downstream compatibility facade is the migration boundary.
