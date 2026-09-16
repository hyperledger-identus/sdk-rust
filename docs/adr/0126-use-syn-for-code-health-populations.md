# ADR 0126: use syn for code-health population classification

- **Status:** Accepted for implementation
- **Date:** 2026-09-16
- **Decision authority:** sponsor-directed issue
  [#275](https://github.com/hyperledger-identus/sdk-rust/issues/275)
- **Supersedes:** the v1 Python syntax-classifier portion of ADR 0115; its
  quality-budget, population separation, attention, and ratchet decisions remain
  effective
- **Related:** ADR 0115, ADR 0120, `SDK-ARCH-004`, `SDK-LIM-004`
- **Assessed revision:** sdk-rust
  `5dff6f38c861b858dd62dc8310246f7d485d3e92`

## Context

ADR 0115 intentionally adopted a conservative Python subset for the first
code-health baseline and assigned full Rust syntax to issue #275. Extending
that scanner would duplicate Rust grammar and turn exclusion logic into an
increasingly fragile security boundary. The workspace already locks the `syn`
parser family and has a non-published conformance package.

## Decision

Add a non-published `identus-conformance` binary using locked `syn` 2.0.118 and
`proc-macro2` 1.0.106 span locations. The binary owns cfg/cfg_attr evaluation,
AST node spans, production-conservative line projection, module resolution, and
production-wins reachability through a bounded versioned JSON protocol. No
parser type or classifier API enters an SDK crate.

The AST visitor covers items, fields, variants, parameters, generic parameters,
statements/expressions, match arms, and represented macros. Macro token streams
remain opaque. Unknown inclusion stays production. A line leaves production
only when all authored non-whitespace bytes are covered by proven test-only
spans. Raw identifiers normalize for module resolution, contained literal path
overrides are supported, and any active/unknown production path to a shared
module wins transitively. Reachability state propagates through nested item,
statement, expression, and match-arm scopes. A conditional `cfg_attr` that may
apply a module `path` override fails closed when its predicate is not proven
false, because selecting only one candidate could hide production code.

Python remains responsible for Git/source loading, subprocess orchestration,
metric evidence, and canonical report validation. It no longer parses Rust
boundaries. Fast CI invokes the helper in the pinned Nix shell without the
heavy metric engine; weekly/manual evidence uses the same population helper
before full metric regeneration.

The code-health policy/report advances to v2 and binds classifier identity,
protocol, and an exact digest of each file's population projection. Migration
requires an exhaustive v1/v2 delta report and a reachable baseline revision
from the durable target-branch history. The baseline revision identifies the
source tree being classified; it need not contain the current classifier,
because classifier identity, protocol, locked dependencies, and the complete
per-file projection digest bind the interpretation independently. Parser
upgrades require the same governed migration.

## Consequences

- Rust grammar ownership moves to a maintained parser already present in the
  workspace dependency cone.
- Population precision improves without relaxing conservative production rules.
- The unpublished conformance tool gains parser/JSON dependencies and fast CI
  incurs a small measured helper build/cache cost.
- Macro expansion remains out of scope; only authored AST is classified.
- Parser/span upgrades become explicit evidence migrations rather than silent
  baseline drift.

## Alternatives rejected

- Extending Python would preserve the duplicate parser and its maintenance risk.
- rustc-private APIs would require nightly/compiler coupling.
- tree-sitter-rust would add another grammar and native/tool dependency without
  a demonstrated capability gap.
- Keeping v1 would retain known precision gaps indefinitely.

## Verification and rollback

Focused Rust fixtures cover ordinary/raw identifiers, path overrides, generics,
shifts, labels, Unicode macros, nested comments, raw strings, recursive
cfg_attr, inner/outer attributes, same-line siblings, declaration/struct-literal
fields, variants, parameters, statements, match arms, associated-item cfg
inheritance, standard binary roots, path-adjusted inline modules, generated
module visibility, macro opacity, and shared reachability.
Protocol-bound, malformed-source, Python orchestration, baseline mutation,
fast timing, full regeneration, factory/OpenSpec, fmt, strict Clippy, workspace
tests, and compatible Nix checks provide integration evidence.

Rollback restores the v1 Python scanner, schema/policy/report, workflow, and
documentation atomically. No SDK public, wire, persisted, release, or consumer
migration is involved.
