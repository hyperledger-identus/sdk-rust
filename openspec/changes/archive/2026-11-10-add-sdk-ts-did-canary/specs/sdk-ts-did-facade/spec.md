## ADDED Requirements

### Requirement: The DID facade exposes capability-oriented plain values

The SDK SHALL provide a strict additive TypeScript facade for DID and DID URL
parsing whose public exports use standards/capability names. Generated WASM
classes, initializer functions, memory, raw errors, lifecycle methods, and the
historical Apollo, Castor, Mercury, Pluto, Pollux, and Edge Agent names SHALL
NOT appear in the public export or declaration snapshots.

#### Scenario: a valid DID crosses the public boundary

- **WHEN** a target provider parses a valid DID
- **THEN** the caller receives a frozen plain value containing exact serialized,
  method, and method-specific identifier fields and no WASM ownership surface

### Requirement: Generated ownership and errors are closed safely

The facade SHALL free each generated success or error object exactly once,
including exceptional projection and observer paths. It SHALL map only the
closed `did.invalid_did` and `did.invalid_did_url` codes into a normal redacted
TypeScript error and SHALL NOT expose caller input, generated diagnostics,
stacks, memory, or implementation types.

#### Scenario: projection or observation fails

- **WHEN** plain-value projection or a configured observer throws
- **THEN** the raw WASM value is still freed, the parse result follows the
  documented observer-isolation policy, and no raw object escapes

### Requirement: Target loaders are explicit and independent

Browser and Node.js SHALL use separately generated, hashed, API-versioned WASM
artifacts and deterministic loaders. Browser SHALL expose ESM. Node.js SHALL
prove both ESM import and CommonJS require without loading mismatched or duplicate
WASM instances. Runtime environment guessing SHALL NOT select a loader.

#### Scenario: package entries load on their target

- **WHEN** browser ESM, Node ESM, and Node CommonJS fixtures import the private
  package entries
- **THEN** each entry initializes its target artifact once, validates binding
  API version 1, and passes the same DID contract assertions

### Requirement: React web reuses target providers

React web integration SHALL own initialization and lifecycle only. Client
execution SHALL reuse the browser provider; SSR and hydration SHALL cross the
Node.js contract and reuse the same public values. React code SHALL NOT contain
a second DID parser or a second Rust artifact.

#### Scenario: server output hydrates in a client

- **WHEN** a React fixture parses during Node SSR and hydrates through the
  browser entry
- **THEN** both phases observe the same public DID value and lifecycle states
  without duplicated portable semantics

### Requirement: Shared vectors and rollback are provider-neutral

One TypeScript assertion body SHALL execute every active case in the exact
`did.syntax.v1` packet against each applicable Rust-backed target provider.
Rollback SHALL select an alternate provider before initialization; a Rust parse
failure SHALL NOT fall through to another parser.

#### Scenario: the Rust route rejects input

- **WHEN** the selected Rust provider returns a stable parse error
- **THEN** the facade returns that mapped error, records only privacy-safe result
  metadata, and does not invoke the rollback provider

### Requirement: Type and package evidence is deterministic

The package SHALL use exact locked dependencies, strict TypeScript, no public
explicit `any`, runtime boundary validation, deterministic declaration/export
snapshots, and reproducible assembly. Browser, Node.js, and React web SHALL have
independent runtime receipts bound to one exact repository head.

#### Scenario: generated or public API drifts

- **WHEN** a generated declaration, facade export, package entry, lockfile, or
  artifact identity differs from the reviewed snapshot
- **THEN** the owning target check fails before its evidence can be aggregated

### Requirement: Resource evidence remains measurement-only

The canary SHALL record reproducible artifact size, initialization, ordinary and
near-limit parse latency, and memory evidence where reliable. Until a separate
target-support decision accepts budgets, those measurements SHALL NOT activate
support, publication, certification, or production claims.

#### Scenario: target measurements pass

- **WHEN** every diagnostic measurement is produced within its bounded protocol
- **THEN** the receipt reports values and environment identity while retaining
  `target-not-supported` status

### Requirement: React Native is explicitly outside the WASM canary

React Native SHALL emit a structural not-applicable receipt for this package and
SHALL NOT install a native toolchain or reuse browser WASM as runtime evidence.

#### Scenario: the canary target plan includes React Native

- **WHEN** issue #492 target selection is evaluated
- **THEN** the React Native result names ADR 0163 and the later TurboModule
  boundary without claiming build, runtime, package, or support evidence

### Requirement: Rust-core evidence is preserved independently

Rust-core validation SHALL complete before fail-closed binding impact planning
and independent target jobs. A selected target failure, cancellation, skip,
timeout, stale head, or missing receipt SHALL keep the protected `fast`
aggregator red but SHALL NOT erase, relabel, or retroactively fail the completed
Rust-core result.

#### Scenario: Node.js fails after Rust core succeeds

- **WHEN** the exact-head Node.js target fails after a green Rust-core job
- **THEN** the Rust-core receipt remains green and independently visible while
  the terminal integration verdict fails

### Requirement: Consumer adoption remains a separate gate

The repository-local package and fixtures SHALL remain private, unpublished,
opt-in, and non-default. They SHALL NOT claim that the separately owned SDK-TS
repository, a named product, or an npm consumer has adopted the route.

#### Scenario: local canary evidence is complete

- **WHEN** every repository-local target and quality gate passes
- **THEN** issue #492 remains partial until the separate SDK-TS rehearsal and
  rollback evidence are accepted
