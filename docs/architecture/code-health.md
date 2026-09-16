# Code-health evidence contract

The code-health audit is an architecture review input, not a scorecard. Run it
inside the pinned shell:

```bash
./bootstrap.sh -- python3 scripts/code-health-audit.py --output /tmp/code-health.json
```

The command uses the unpublished `syn` classifier and
`rust-code-analysis-cli 0.0.25`, reads the policy and hotspot dispositions in
[`code-health.toml`](code-health.toml), and emits canonical JSON. The v2
baseline records an immutable, durable `develop` source revision. Its revision,
source fingerprint, classifier identity and protocol, execution command, exact
per-file population-projection digest, and whole
canonical-report digest are policy-pinned. The exhaustive v1-to-v2 comparison is in
[`code-health-v2-migration.md`](code-health-v2-migration.md).

Fast validation reloads the pinned Git tree and uses the same classifier to
recompute the authored fingerprint, exact generated exclusions, complete
per-file inline-test line sets, and line/file populations without running the
metric engine. Aggregate equality cannot hide a line reclassification:

```bash
nix develop --command python3 scripts/code-health-audit.py \
  --check-report docs/architecture/code-health-baseline.json
```

Slow validation uses the Nix-pinned analyzer to regenerate and compare the
complete report, including function counts and attention signals:

```bash
./bootstrap.sh -- python3 scripts/code-health-audit.py --verify-baseline
```

Run that command locally, manually in GitHub, or through the native weekly slow
workflow on protected default `develop` under ADR 0120. `main` remains minimal
and explicitly protected. The slow-run freshness audit, rather than the YAML
cron declaration alone, is the operational execution evidence.

Source-only Nix archives lack Git history. Their factory structural check uses
`--policy-only`, which still enforces the exact schema and policy-pinned report
digest; the fast Git checkout and locally or externally invoked slow
regeneration provide the tree bindings.

A live audit rejects tracked or untracked Rust that differs from the recorded
revision. Commit the intended source first or run the command from a clean
worktree; unrelated documentation changes do not invalidate source evidence.
The baseline source revision must be a durable ancestor of the target branch so
it remains available after branch deletion and under merge, squash, or rebase
integration. It identifies the historical source snapshot, not the classifier
implementation commit. The pinned classifier identity, protocol, dependency
versions, and exact per-file projection digest bind the current classifier's
interpretation of that snapshot. Full validation also requires the pinned
revision to be an ancestor of the reviewed checkout, preventing temporary
feature-branch objects from becoming durable evidence.

## Populations

- **Production** begins as every authored Rust file under `crates/*/src`.
  A filename such as `src/tests.rs` has no special trust and remains production
  unless syntax-aware module reachability proves that it is test-only.
- **External test** is authored Rust in Cargo's intrinsic `crates/*/tests` and
  `crates/*/benches` target trees.
- **Inline test** is classified from the complete `syn` AST. The visitor covers
  items, declaration, struct-literal and struct-pattern fields, variants,
  function, method, closure and bare-function-type parameters including
  variadics, generic parameters, statements and expressions, match arms,
  impl/trait/foreign items, and represented macros.
  `cfg` and recursively applied `cfg_attr` use a three-valued evaluation with
  `test = false`; unknown syntax or inclusion remains production. Inner and
  outer attributes share those semantics.
  A source line is inline-test only when every authored non-whitespace byte is
  covered by proven test-only AST spans. Mixed test/shipping lines therefore
  remain production. Merged spans are consumed through one monotonic cursor,
  keeping projection linear in source bytes plus spans. Macro token streams are
  deliberately opaque: attributes inside a macro body are never interpreted,
  while an outer cfg on the macro node itself is classified normally.
  Test-only out-of-line modules recursively resolve ordinary `name.rs` and
  `name/mod.rs` layouts, raw identifiers, nested module contexts, and literal
  `#[path = "..."]` overrides. Resolution carries whether a source is entered
  as a target root or nested module and preserves path-adjusted inline-module
  directories. Exact library/binary roots are derived from Cargo manifests at
  the audited revision and supplied through classifier protocol v2; an
  ordinary helper named `main` does not become a target. Cargo roots remain
  production even if a test-only edge reaches them. A
  fixed-point reachability pass applies the
  production-wins rule whenever any active or unknown production edge reaches
  a shared module or descendant. Reachability state propagates through nested
  item, associated-item, statement, expression, struct-field-value, and
  match-arm scopes. Conditional module paths are evaluated separately with
  `test = false` and `test = true`: edges false in both configurations are
  disabled and omitted before path resolution, while unknown predicates or a
  production edge that selects different paths fail closed. Only direct
  `path = "..."` attributes, including values recursively applied by
  `cfg_attr`, affect module paths; unrelated nested metadata does not.
  Malformed Rust, invalid spans, ambiguous module targets, unsupported
  predicate forms, and incomplete coverage fail closed with path/location
  diagnostics and never echo source text.
- **Generated** Rust is excluded only when `code-health.toml` names its exact
  path and an exact marker present in the first ten lines. Generic phrases in
  comments never cause exclusion. Excluded sources remain available to the
  module-resolution graph so an authored `mod generated;` edge is still valid;
  only their metric lines are omitted.

`authored_nonblank_lines` includes comments and documentation deliberately: it
measures review surface, not executable SLOC. Function `sloc`, cognitive, and
cyclomatic values come from the pinned engine. Populations are never averaged
or added together to imply a production size.

## Attention and disposition

The v2 attention prompts are function SLOC above 100, cognitive or cyclomatic
complexity above 15, and module authored nonblank production lines above
1,000. A module-size signal matters only with evidence of at least two
independent invariants or change axes. Each named hotspot has exactly one
disposition: `decompose`, `deduplicate`, `document-exception`, or
`defer-with-owner`.

No numeric signal fails CI. Structural failures are limited to an unavailable
or wrong engine for a live audit, invalid config/report shape, analysis errors,
or an unclassified declared hotspot.

## Touched-scope ratchet

Review the semantic responsibility and its call cluster at base and head—not
only the edited function. Record every new or worsened attention signal and
give it a disposition and owner. A change cannot claim improvement from:

- moving or renaming code;
- adding forwarding wrappers;
- formatting or comment deletion;
- moving behavior behind macros or generated output;
- splitting one responsibility across files; or
- deleting, moving, or reclassifying tests.

Deduplicate only when the implementations share the same invariant, resource
bounds, error projection, owner, and likely change cadence. Otherwise document
the intentional parallel behavior. Public API stability, wire compatibility,
dependency direction, feature closure, characterization evidence, and
discoverable ownership outrank a smaller metric.
