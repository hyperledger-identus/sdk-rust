# Code-health evidence contract

The code-health audit is an architecture review input, not a scorecard. Run it
inside the pinned shell:

```bash
./bootstrap.sh -- python3 scripts/code-health-audit.py --output /tmp/code-health.json
```

The command requires `rust-code-analysis-cli 0.0.25`, reads the policy and
hotspot dispositions in [`code-health.toml`](code-health.toml), and emits
canonical JSON. `docs/architecture/code-health-baseline.json` records the
immutable issue #270 starting revision. Its revision, source fingerprint and
whole canonical-report digest are policy-pinned. Fast validation reloads that
Git tree and recomputes the authored fingerprint, exact generated exclusions,
and line/file populations without running the metric engine:

```bash
python3 scripts/code-health-audit.py \
  --check-report docs/architecture/code-health-baseline.json
```

Weekly slow validation uses the Nix-pinned analyzer to regenerate and compare
the complete report, including function counts and attention signals:

```bash
./bootstrap.sh -- python3 scripts/code-health-audit.py --verify-baseline
```

Source-only Nix archives lack Git history. Their factory structural check uses
`--policy-only`, which still enforces the exact schema and policy-pinned report
digest; the fast Git checkout and weekly regeneration gates provide the tree
bindings.

A live audit rejects tracked or untracked Rust that differs from the recorded
revision. Commit the intended source first or run the command from a clean
worktree; unrelated documentation changes do not invalidate source evidence.

## Populations

- **Production** is authored Rust under `crates/*/src`, excluding dedicated
  `src/tests.rs` or `src/tests/` files and syntax-recognized test-only items.
- **External test** is authored Rust in `crates/*/tests` plus dedicated source
  test files.
- **Inline test** is an item whose `cfg` predicate is definitively false when
  `test = false`. Its complete contiguous outer-attribute group is included.
  A test-only out-of-line `mod name;` recursively classifies the ordinary
  `name.rs` or `name/mod.rs` module tree. Other predicates are unknown, so
  `cfg(any(test, feature = "diagnostics"))` remains production.
- **Generated** Rust is excluded only when `code-health.toml` names its exact
  path and an exact marker present in the first ten lines. Generic phrases in
  comments never cause exclusion.

`authored_nonblank_lines` includes comments and documentation deliberately: it
measures review surface, not executable SLOC. Function `sloc`, cognitive, and
cyclomatic values come from the pinned engine. Populations are never averaged
or added together to imply a production size.

## Attention and disposition

The v1 attention prompts are function SLOC above 100, cognitive or cyclomatic
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
