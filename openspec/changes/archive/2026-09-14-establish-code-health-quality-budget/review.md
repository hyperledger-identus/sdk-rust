# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-14
Develop base: `707a5a22c3fad18724d5c5cac953e7387f7e49d8`
Implementation head: `f5c057e7b67a7b3c94560a0b03bca92a43110b6d`
Review-remediation head: `34a197692af144cd53eeafed31175b8a9f8d2af4`
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
   attribute group, and terminates comma-delimited fields and variants without
   consuming the next item. Ordinary out-of-line modules inherit a test-only
   declaration recursively, including preclassified `tests.rs` trees. The
   evaluator treats `test` as false and unknown target/feature predicates
   conservatively as production. Deleted, modified or untracked Rust invalidates
   an exact working-tree audit.
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
mutation and parser fixtures for each failure mode. The corrected baseline moves
the `conformance` guard tree and other inherited modules from production into
inline-test evidence. Review found no remaining blocker in the remediated diff.

## Review decision

The slice is cohesive, behavior-preserving, reproducible and independently
reversible. No correctness, architecture, security, privacy, dependency,
compatibility or scope blocker remains.
