# Repository portfolio semantic and exact-diff review

- **Date:** 2026-09-08
- **Issue:** #175
- **Develop base:** `ff924db61452d4a1fd2a8bd4a1bfd3d702519604`
- **Reviewer:** distinct read-only agent context
- **Result:** no unresolved blocker

## Semantic findings

1. The portfolio preserves ADR 0061's owned-facade rule. ADR 0066 explicitly
   supersedes only its Spruce-specific oracle disposition for one named narrow
   crate; the umbrella remains prohibited and no dependency is added.
2. The disposition vocabulary matches the existing factory contract:
   `conditional-adopt`, `spike`, `oracle`, and `not-adopt`. Reference use for a
   rejected production dependency is described as an allowed activity rather
   than a fifth lifecycle state.
3. The canonical ADR-before-use rule applies to the twelve accepted portfolio
   entries. New repositories may be researched read-only without circular
   approval; an issue/ADR becomes mandatory before production dependency or
   source-donor use.
4. Adoption evidence refers to every effective toolchain/target gate at the
   time of integration. It does not assume issue #173 has changed the current
   policy.
5. Spruce narrow crates and AnonCreds v1 are conditional candidates, not
   integrated dependencies. Askar remains a spike under #162. Candidate-level
   cone, target, advisory, unsafe/native, and conformance commands not run by
   this portfolio are named explicitly and remain future integration gates.
6. Procivis, Impierce, IOTA Identity, DIF did-key, and VCX are oracles. SICPA
   DIDComm, AnonCreds v2, OWF SD-JWT, and DIDKit are not adopted, with bounded
   read-only reference use where useful.
7. Every repository has a pinned SHA, dated evidence, allowed/prohibited use,
   and objective reconsideration trigger. The mutable Procivis standards page
   is labeled rather than represented as immutable repository evidence.
8. Normative standards and conformance suites continue to outrank implementation
   precedent. Framework trust, transport, storage, custody, consent, chain, and
   product policy do not cross into the SDK.

## Exact-diff findings

1. No Cargo manifest, lockfile, Rust source, Nix toolchain, workflow,
   support-policy value, public API, wire behavior, or consumer repository is
   changed.
2. Twelve consecutive ADRs occupy 0066 through 0077; 0065 remains available for
   issue #173's CI/toolchain implementation.
3. ADR 0061 receives only a forward link and exact Spruce supersession note.
   The negative ledger adds missing repository decisions and conditional paths.
4. The new OpenSpec capability contains objective scenarios and does not impose
   an ADR merely to begin ordinary source research.
5. Relative ADR/report links and issue/discussion references are internally
   consistent. Markdown, EditorConfig, YAML, shell, OpenSpec, factory,
   research, constraint, backlog, inventory, and support-policy structural
   checks pass.

## Corrections made during review

1. Removed an invented `watch` status and mapped Impierce to `oracle` and
   AnonCreds v2 to `not-adopt`.
2. Replaced hard-coded Rust 1.98 prerequisites with the effective toolchain and
   target policy at integration time.
3. Scoped the repository ADR requirement and separated research permission
   from production/source-donor activation.
4. Made the Spruce-specific supersession explicit and kept Askar at `spike`.
5. Added release links, labeled mutable vendor evidence, and recorded unrun
   candidate-level integration commands.
6. Removed the unrelated CI discussion from this OpenSpec change's completion
   tasks; issue #173 owns that implementation independently.

Verdict: READY after the final checks and guarded OpenSpec archive complete.
