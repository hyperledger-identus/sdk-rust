# Design

## Registry shape

`docs/architecture/quality-evidence-declarations.toml` is the canonical machine
registry. Its closed top level owns schema/registry identity, issue, status,
evaluation date, allowed class/lane/cadence vocabularies, and declaration
tables. The first stable declaration is `did.syntax.quality.v1`.

Each declaration identifies capability, slice/packet, owner, issue, risk,
targets, #420 vector IDs, limitations, and lifecycle. It contains exactly four
closed obligation tables ordered as property, fuzz, benchmark, and
differential. Stable IDs are additive; replacement uses explicit
`supersedes`/`replaced_by` links.

## Obligation contract

All obligations use one exact field set so omissions cannot hide behind TOML
shape differences. Class-specific `details` entries use closed keys:

- property: `invariant`, `domain`, and `cases`;
- fuzz: `harness`, `corpus`, `seed`, `bounds`, and `sanitizer`;
- benchmark: `operation`, `fixture`, `environment`, `statistic`, `samples`,
  and `threshold`; and
- differential: `oracle`, `oracle-revision`, `normalization`, and `deviations`.

A satisfied obligation names safe repository selectors, one non-executed exact
command, a non-`none` lane/cadence, target set, budget, immutable receipt,
evidence revision/date, and no debt. A required obligation carries the same
executable plan but no success receipt and must name owned debt. A
not-applicable obligation carries none of those claims and instead names the
risk rationale and reviewing issue.

## DID seed decisions

- Property is satisfied by the deterministic ASCII grammar,
  constructor/deserializer equivalence, and exact-limit tests in
  `crates/did/tests/did_syntax.rs`; it routes to `focused/on-change`.
- Fuzz is satisfied by both DID lexical libFuzzer targets, the pinned
  corpus/dictionaries/limits, and natural scheduled run `36517752658`; it
  routes to `slow/weekly-or-manual` with an eight-day evaluation budget.
- Benchmark is reviewed not applicable because A1 has no named consumer
  performance outcome, comparable environment, or threshold.
- Differential is satisfied by the `did.syntax.v1` packet and Rust conformance
  loader; it routes to `fast/per-pull-request` because the normal test already
  executes in the required line.

All four classes are explicit without asserting that all four execute on every
pull request.

## Validation and rendering

`scripts/check-quality-evidence-declarations.py` uses only the Python standard
library. It rejects unknown fields, invalid/duplicate IDs, incomplete or
misordered class sets, control-bearing commands, unsafe/symlinked selectors,
unresolved selector symbols or vector IDs, class-detail mismatch, incoherent
disposition/receipt/debt combinations, invalid route/cadence, malformed
revisions/run URLs/dates, receipts stale at `evaluated_at`, and invalid
lifecycle links.

The validator renders deterministic Markdown grouped by declaration and route
to `docs/architecture/quality-evidence-plan.md`; normal checking fails on
render drift. `scripts/factory quality-plan [capability]` prints the same
non-executing plan and never runs stored commands. Mutation tests change each
critical relationship independently and include an additive second synthetic
declaration to prove the registry is not a DID-only allowlist.

The checker resolves #420 vector IDs directly from the local catalog. It does
not fetch GitHub or decide standards authority. Future #422 cross-contract
validation will resolve quality IDs from consumer-visible changes.

## Template integration

The factory issue template and evidence receipt request stable quality
declaration IDs. OpenSpec planning guidance requires those IDs or an explicit
`pending:<proposed-id>` value owned by the current issue. The registry remains the
single source for commands, rationale, and lane routing; templates do not copy
the four obligation bodies.

## Stop boundary

This change stops after metadata, offline validation, deterministic planning,
and template/factory integration. It does not execute a consumer test, change
CI schedules, add a testing dependency, publish evidence, or mutate SDK-TS.
