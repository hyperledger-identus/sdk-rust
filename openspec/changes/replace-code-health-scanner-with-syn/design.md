# Design

## Context

The Python v1 scanner sanitizes source text, recognizes a whitelist of item
terminators, evaluates a subset of cfg metadata, projects byte spans to lines,
and resolves ordinary module declarations. Unsupported syntax remains
production, which is safe but leaves known precision gaps and makes every new
Rust construct a parser-maintenance risk.

The workspace already locks `syn` 2.0.118, `proc-macro2` 1.0.106, and `quote`
1.0.46 for verification/procedural-macro work. `identus-conformance` is a
non-published verification package, so its binary can depend on parser and JSON
tooling without entering an SDK crate or public API.

## Decisions

### One bounded classifier protocol

The `code-health-classifier` binary reads one UTF-8 JSON request from stdin and
writes one canonical JSON result to stdout. The protocol is versioned and caps
request bytes, file count, path bytes, per-file bytes, emitted spans/lines, and
module edges before retention. Diagnostics name a repository-relative file and
source location but never source contents.

Python invokes the binary through the exact command recorded in policy while
inside the repository-pinned Nix shell. The binary is internal tooling; parser
types and protocol structs never cross an SDK crate API.

### AST-owned population semantics

The classifier parses every authored source with `syn::parse_file`. Parse
failure is an actionable audit error. Outer attributes are evaluated with a
three-valued cfg model where `test=false`; unknown predicates remain
production. Recursive `cfg_attr` only changes inclusion when applicability and
the applied metadata are understood. Inner cfg attributes propagate to their
AST scope when proven false; unknown or unsupported metadata remains
production.

Visitors record complete attributed spans for items, fields, variants,
parameters, generic parameters, statements/expressions, match arms, and macro
nodes represented by `syn`. Macro token streams are opaque: cfg-looking tokens
inside them never become source attributes. A node is a test-only candidate
only when its own effective attributes or an inherited AST scope are proven
inactive in production.

Span projection is production-conservative. A nonblank authored line is
inline-test only when every non-whitespace source byte belongs to one or more
proven test-only spans. Any shipping token, comment, literal, unsupported
region, or span ambiguity on the same line keeps the whole line production.

### Module reachability reaches a fixed point

The classifier normalizes raw and ordinary identifiers to the same module
name. It resolves ordinary module paths and literal `#[path = "..."]`
overrides relative to their declaring module. A module file reached only from a
proven test-only edge and its test-only descendants becomes inline-test.
Active or unknown production reachability to the same file wins and propagates
through ordinary descendants until a fixed point. Ambiguous or escaping module
paths fail with an actionable error rather than hiding code.

### Contract v2 and migration evidence

The policy/report contract advances to v2 and records classifier name,
protocol, locked dependency evidence, and a digest of the complete per-file
line projection. A governed baseline regeneration is bound to the
implementation commit. Aggregate population equality is insufficient. A
checked-in migration report compares v1 and v2 file/line populations and
explains every delta by fixture or source location; no unexplained production
decrease is accepted.

Fast validation recomputes source fingerprint and populations through the AST
classifier but does not run `rust-code-analysis-cli`. Weekly/manual validation
uses the same classifier plus the pinned metrics engine to regenerate the full
report.

The migration PR uses an ancestry-preserving merge so the implementation
revision remains reachable after automatic branch deletion. A squash or rebase
integration requires repinning and regenerating the baseline from a durable
commit before the migration can be called complete.

## Risks and mitigations

- `proc_macro2` span locations are tooling-only and version-sensitive: lock the
  resolved versions and bind them into the contract/migration evidence.
- Macro expansion is unavailable: keep token streams opaque and authored-line
  production conservative.
- A parser upgrade can change spans: require an explicit baseline migration,
  delta report, adversarial fixtures, and review.
- Invoking Cargo in fast validation adds latency: build one small verification
  binary in the pinned shell, measure it, and never run the heavyweight metric
  engine on the fast path.
- Source offsets can split UTF-8 incorrectly: use `proc_macro2` byte ranges and
  validate every boundary against Rust strings before projection.

## Alternatives

- Extend the Python parser: rejected because it duplicates Rust grammar and
  grows security-sensitive exclusion logic.
- Use rustc private APIs: rejected because they require unstable compiler
  coupling and a much larger dependency/toolchain surface.
- Use tree-sitter-rust: rejected for this slice because `syn` is already locked,
  license-approved, target-portable, and used by repository verification.
- Keep v1 indefinitely: rejected because known syntax gaps make the population
  evidence less useful and harder to maintain.

## Rollback

Revert the v2 classifier, policy/report migration, and workflow shell change,
then restore the complete v1 Python scanner and baseline atomically. No SDK or
consumer rollback is required.
