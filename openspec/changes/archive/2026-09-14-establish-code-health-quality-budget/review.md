# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-14
Develop base: `707a5a22c3fad18724d5c5cac953e7387f7e49d8`
Implementation head: `f5c057e7b67a7b3c94560a0b03bca92a43110b6d`
First review-remediation head: `34a197692af144cd53eeafed31175b8a9f8d2af4`
Second review-remediation head: `88bcb1700170b184760e388634f5ebbf49aeb108`
Third review-remediation head: `6096cab17a54882cf98e7044d8174610208ccdc4`
Fourth review-remediation head: `2b948ebe47eb03fbf2207d6ba8b7ca2ca859c785`
Fifth review-remediation head: `f1826db71de8f370bfb5cf3dd49f0725be2602bb`
Sixth review-remediation head: `2acf13f4ccb18450b7ebc21294330ac095fd2c9e`
Seventh review-remediation head: `3bafdbb7ebc3392e381316b28c3ec3afd627c32e`
Final review-remediation head: `1521c242ffac901f37dea0766b1510ae2f5180dc`
Planning head: `0a7ded352110474150e409aabcb3a2c4edfed661`
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-implementation diff, issue #270,
ADR 0115, the report/config pair, population scanner and tests, factory/Nix
integration, all changed DID production and test code, Cargo manifests and the
dependency lock. It explicitly checked that wallet `run_exact`, derive output,
OID4VCI limits and error architecture, issue #7 and issue #168 semantics were
not changed.

## Findings

1. **Population integrity — accepted after correction.** The lexical pass
   blanks nested comments and Rust string/character forms while retaining
   offsets, balances attributes/items, includes a complete contiguous outer
   attribute group, and terminates supported comma-delimited nodes without
   consuming the next item. Ambiguous angles, unmatched container delimiters,
   comma-less members, nested block expressions and unrecognized macro forms
   receive no test span; macro definition/invocation token contents and mixed
   source lines remain production. Ordinary
   out-of-line modules inherit a test-only
   declaration recursively with correct nested inline-module context. Only
   Cargo `tests/` and `benches/` targets are intrinsically external tests;
   `src/tests.rs` and its tree require proven test-only reachability. Macro token
   trees cannot create module edges, `#[path]` overrides fail closed, and outer
   line/block docs share their test-only item's span. The evaluator treats
   `test` as false and unknown target/feature predicates conservatively as
   production. Raw module names normalize to their ordinary source filename,
   and Unicode Rust lifetimes or labels cannot corrupt lexical boundaries.
   Deleted, modified or untracked Rust invalidates an exact working-tree audit.
2. **Metrics and anti-gaming — accepted.** The Nix lock supplies exact
   `rust-code-analysis-cli 0.0.25`; sorted paths and canonical JSON make output
   reproducible. A fast gate resolves the policy-pinned revision and recomputes
   source fingerprint, exclusions, file counts and line populations from that
   Git tree. The weekly slow gate reruns the pinned analyzer and compares the
   complete report. Exact recursive schemas, a policy-pinned report digest and
   exact path/marker generated exclusions reject coordinated evidence editing
   and arbitrary marker prose. Numeric attention signals do not fail CI. Finite
   dispositions, semantic call-cluster review and explicit prohibited shortcuts
   prevent a line-count score from rewarding wrappers, file splitting or test
   movement.
3. **Architecture — accepted.** The sole production refactor moves one
   identical invariant from cache and registry into the private resolution
   ownership module. No public helper, new crate, feature or dependency edge is
   introduced. The remaining named hotspots are classified but unchanged.
4. **Behavior and wire compatibility — accepted.** Public-path
   characterization tests assert each standard error kind, absent document,
   empty document metadata and exact JSON. The helper preserves the same two
   infallible validated constructors and panic messages.
5. **Security and privacy — accepted.** The analyzer executes no repository
   source and records only paths, spans, counts, classifications and a digest.
   It emits no source bodies, secrets or runtime values. DID failure diagnostics,
   redaction, resource bounds, unsafe posture and external I/O are unchanged.
6. **Scope and dependency direction — accepted.** The base-to-head Cargo
   manifest and lock diff is empty. Only the default development shell gains a
   locked host analyzer. Searches and the exact Rust diff confirm no wallet,
   derive, OID4VCI, #7 or #168 implementation entered the slice.

## Residual limitations

- The repository wrapper is deliberately conservative rather than a full
  rustc configuration evaluator: unknown target and feature predicates remain
  production.
- Test-only module reachability implements ordinary Rust layout and rejects
  `#[path]` overrides rather than emulating the full rustc module loader.
- Static metrics and token similarity cannot prove cohesion, correctness,
  security or semantic duplication; maintainers still own the disposition.
- Macro expansion is not attributed as authored source and generated Rust is
  reported separately only when an exact policy-allowlisted path contains its
  exact marker in the first ten lines.

## Independent PR review remediation

The first pushed head was not accepted: independent review demonstrated that
its report validator trusted mutable evidence, its generated-source heuristic
was over-broad, and its population scanner mishandled comma-delimited items and
test-gated out-of-line modules. Head `34a1976` resolves every finding and adds
mutation and parser fixtures for each failure mode. Follow-up review then found
two more hiding paths: nested inline modules lost their resolution context, and
source files named `tests.rs` were trusted without a test-only declaration.
Head `88bcb17` closes both, isolates macro token trees, rejects custom path
loading, includes outer doc attributes and closes schema primitive types. A
final mixed-reachability review showed that a file could still be both test-
and production-reachable. Head `6096cab` makes active/unknown production edges
win at a fixed point, including descendants, and retains enclosing inline-module
context for nested test declarations. The corrected baseline moves the
`conformance` guard tree and only proven test-exclusive source modules into
inline-test evidence. Hosted review finally identified brace-delimited item
macros as an unbounded item-end case. Head `2b948eb` recognizes qualified item
macro paths, balances the brace token tree, and accepts the optional trailing
semicolon without consuming the next shipping item. A final cfg review found
comments were parsed as tokens and `cfg_attr` application was ignored. Head
`f1826db` preserves literal values while blanking nested comments, accepts raw
strings and raw identifiers, and applies nested `cfg_attr` with conservative
true/false/unknown semantics. Subsequent adversarial review showed that broader
member/expression heuristics would become a partial Rust parser. Heads
`2acf13f` and `3bafdbb` replace them with a strict no-hiding terminator
whitelist, make mixed lines production-wins, normalize raw module identifiers,
evaluate stable cfg booleans, preserve Unicode/unknown cfg conservatism, and
recognize Unicode Rust lifetimes. Head `1521c24` additionally prevents
attribute-like macro token-tree contents from being interpreted as source
attributes, including the demonstrated macro that consumes a literal
`#[cfg(test)]` token and emits its captured item without the attribute. Issue
#275 tracks a future non-published `syn` classifier. Review found no remaining
blocker in the final remediated diff.

## Review decision

The slice is cohesive, behavior-preserving, reproducible and independently
reversible. No correctness, architecture, security, privacy, dependency,
compatibility or scope blocker remains.
