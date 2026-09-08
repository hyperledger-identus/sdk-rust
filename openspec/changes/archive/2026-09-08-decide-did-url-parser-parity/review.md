# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Evidence head: 2e3d49165ec7c79e2d113ddcda24b047fe7dd26b
Specification parents: dc52c269cb7cdfe4fa8a852687a95bc3bf5eb976 and
2628ebab452c997f306882e0c8b733b4b29b0027
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `develop`-to-evidence-head diff, the current
SDK DID parser and facade, every committed corpus case, exhaustive generator,
candidate source and package metadata, both resolved dependency graphs, target
checks, advisory results and the resulting ADR/portfolio decision. It also
verified that the harness is an isolated unpublished workspace and that no
generated target artifact is tracked.

## Findings

1. **Normative parity — rejected.** Exact `did_url_parser 0.3.0` disagrees in
   7 of 30 attributable corpus cases and 95 of 17,284 generated comparisons.
   It accepts surrounding ASCII controls/space, a terminal-colon identifier
   and 22 `%+<hex>` spellings outside the DID Core/RFC 3986 language.
2. **Representation and resource parity — rejected.** The candidate stores the
   original untrimmed input after validating a trimmed view, copies an owned
   `String`, and offers no pre-allocation 2 KiB/4 KiB ceiling. A wrapper would
   therefore retain the SDK's validation, bounds and facade machinery.
3. **Invariant boundary — rejected.** Public unchecked setters can construct a
   DID rejected by the candidate parser. That conflicts with immutable SDK
   validated values and prevents exposure of upstream types at the public API.
4. **Safety and cohesion — rejected.** Public relative joining contains an
   unsafe UTF-8 conversion, while query helpers add HTML-form semantics not
   consumed by the generic lexical layer. Keeping only the wanted parser path
   would not remove the current implementation.
5. **Supply chain — understood.** The candidate is focused, pure Rust,
   `no_std + alloc`, MIT OR Apache-2.0 and has a three-package minimal normal
   graph. Its normal consumer lock audits cleanly. The published development
   lock reaches an advisory through old test-only `proptest`/`rand`; this is
   reproducibility evidence rather than a runtime vulnerability claim.
6. **Maintenance and target fit — conditional.** Candidate tests, alloc-only
   builds and WASM/Android/iOS targets pass on Rust 1.98.1. Strict Clippy finds
   three lifetime-syntax warnings. Maintenance evidence does not override the
   independent semantic and resource stop conditions.
7. **Repository impact — accepted.** No production source, root manifest,
   release lockfile, public API, serialization or error changes. The retained
   parser remains 455 lines for the used capability versus 1,042 candidate
   source lines including unused surfaces.

## Residual limitations

- The SDK continues to own and maintain its small security-sensitive parser.
- The local full-system gate cannot build x86_64-linux on this host; hosted
  `fast` remains the merge authority for Linux.
- Candidate sanitizers, Miri, Windows and historical compiler matrices were
  intentionally not run after independent adoption stop conditions failed.
- A corrected future release requires a new issue and the ADR 0086
  reconsideration checks; this change does not authorize an upstream patch.

## Review decision

Retain the bounded dependency-free parser under ADR 0008 and record exact
`did_url_parser 0.3.0` as `retain-local` under ADR 0086. The evidence is
reproducible, isolated and sufficient for this bounded dependency decision.
No unresolved correctness, architecture, security, privacy, compatibility or
supply-chain blocker remains for delivery.
