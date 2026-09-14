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
Eighth review-remediation head: `1521c242ffac901f37dea0766b1510ae2f5180dc`
Final review-remediation head: `8ef23489d4cded5cb635b970de9c4e1da360d711`
Scheduler-truth remediation head: `8086d4a162a630eba61071b10bd8b01a90608cdf`
Scheduler-authority consistency head: `dfa531735f4cf19248b9a3c45217bdff3d4c4236`
Historical-authority clarification head: `19f9927bac6d77196b0d4452c85b6bd8e79230f0`
Supersession-notice clarification head: `afb417143f6501dd270ce25d1376f6506d1bcdc9`
Spaced-attribute characterization head: `64f9d4eae32c62ed2eed45e14458b1f394c2a064`
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
   source lines remain production. Only byte-contiguous `#[` attributes enter
   v1; whitespace-separated openers remain production and cannot seed module
   inheritance. Ordinary
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
   Git tree. The slow command reruns the pinned analyzer and compares the
   complete report when invoked locally or by explicit external scheduling.
   Its YAML schedule is not execution evidence while reserved empty `main`
   remains GitHub's default branch; activation belongs to issue #276. Exact
   recursive schemas, a policy-pinned report digest and
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
- The slow workflow is ready to run but GitHub will not execute its declared
  schedule from non-default `develop`; local/external execution remains the
  evidence path until issue #276 establishes scheduling authority.

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
#275 tracks a future non-published `syn` classifier and owns exact inner
`#![cfg(...)]` scope classification; v1 retains those scopes as production.
Head `8086d4a` then makes the slow-lane execution state machine-readable:
`local-or-external`, `inactive-pending-276`, with a desired weekly cadence.
It also requires the workflow to disclose why its GitHub trigger is inactive
and adds mutation tests for both the workflow and policy claims. Independent
review found the global machine contract still conflicted with current support,
factory, target-plan and consumer prose. Head `dfa5317` aligns those authorities
with local/external execution pending #276 and makes the validator reject stale
active wording and the old target-plan policy. Review found no remaining blocker
in those current sources. Head `19f9927` adds a dated, exact inactive-status
banner to every accepted ADR/research source that retains historical cadence
wording and binds that bounded authority set in the checker. It also binds the
corrected browser, crypto and DID evidence prose. Review found no remaining
blocker except ADR 0064's top current supersession notice. Head `afb4171`
aligns and validator-binds that last notice. Head `64f9d4e` adds exact item and
out-of-line module characterization for whitespace-separated attribute openers;
both remain production under v1 and issue #275 owns exact support. No remaining
blocker exists in the final remediated diff.

## Review decision

The slice is cohesive, behavior-preserving, reproducible and independently
reversible. No correctness, architecture, security, privacy, dependency,
compatibility or scope blocker remains.
