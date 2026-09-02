# Semantic preflight review

This review evaluates issue #25 and the specification before implementation.
It does not claim that repository-local evidence, live GitHub controls or
`IDR-001` are delivered.

| Dimension | Result |
| --- | --- |
| Sponsor intent | Advances the remaining R0 governance/API-inventory slice before new component ports |
| Issue-first rule | Focused issue #25 and protected follow-up #26 exist before implementation |
| Exact base | `develop@1d890fdc61b68ca6cd3c53dbfb39e3c03bb2ed1f` |
| Repository boundary | No donor build, code copy or consumer mutation is authorized |
| Protected authority | Governance semantics, settings, visibility, publication and release remain human-owned |
| Architecture | Placeholder graphs become smaller; no new public crate or dependency edge is introduced |
| Compatibility | Implemented APIs are inventoried but unchanged and explicitly experimental |
| Security | Native Cargo publication denial reduces accidental supply-chain exposure |
| Performance | Offline stdlib validation and smaller placeholder graphs keep the fast lane cheap |
| Rollback | Metadata, docs and checks revert without runtime or data migration |

## Findings

- Blockers: 0.
- Resolved in scope: live GitHub state cannot be proven by repository files;
  #26 owns administrator action and authoritative post-change evidence.
- Resolved in design: package existence and layer membership could be mistaken
  for acceptance; maturity and public API status are explicit independent
  fields.
- Resolved in design: the inherited crate-ring spec required unused future
  edges; that requirement is removed and placeholders become minimally
  coupled.
- Resolved in design: a central inventory could silently drift from Cargo or
  `LAYER_RULES`; exact package, path and layer comparisons fail closed.
- Resolved in design: disabling publication only in prose is weak; Cargo-native
  `publish = false` plus per-member inheritance is required.
- Follow-up: exact generated public-item/API diffs, signed external policy
  snapshots and live settings polling are intentionally outside the 70–80%
  boundary.
- Verdict: READY to implement issue #25 after strict structural validation.

# Final local review

A distinct post-implementation pass reviewed the complete branch delta against
issue #25, the active OpenSpec change, the current capability specs and the
machine-readable inventory. The pass inspected publication behavior,
placeholder source and dependency shape, package/layer coverage, governance
authority, factory integration and claims about live GitHub state.

## Findings resolved

- The first final Nix run found a blueprint sentence beginning with `#26`,
  which Markdown interpreted as a malformed heading. The prose now uses
  explicit `issue #26` and `issue #3` references; the complete Nix matrix
  subsequently passed.
- The validator's success message initially hard-coded `14 packages` even
  though its contract derives exact package coverage from Cargo. The count is
  now read from the validated inventory so routine package changes cannot leave
  misleading success output.
- Exact-head hosted review demonstrated that checking only the dependency key
  allowed a placeholder to redirect `identus-core` to a registry or alternate
  source. The validator now requires the exact workspace-inheritance value,
  with an adversarial registry-substitution test.
- Exact-head hosted review demonstrated that `package.build` could point to a
  non-`.rs` source omitted by the placeholder file scan. Package-level custom
  build scripts are now rejected explicitly, with an executable-source
  regression.
- A hosted signing finding named commit `188b12a6`, which is not in the PR's
  four-commit set or local branch history. GitHub's authoritative commit API
  reports all four PR commits verified with reason `valid`; every commit has a
  DCO trailer and the DCO gate passes. No foreign commit was rewritten.

## Final result

| Dimension | Result |
| --- | --- |
| Inventory coverage | All 14 Cargo packages occur exactly once with matching path and `LAYER_RULES` layer |
| Publication safety | Workspace denial and explicit member inheritance are enforced; a dry run is rejected |
| Placeholder cohesion | Eight placeholders contain only one marker and depend only on `identus-core` |
| Existing APIs | Five implemented surfaces remain experimental; conformance remains verification-only |
| Governance truth | Local records are required; public/protected GitHub activation remains external under #26 |
| Repository boundary | No donor, consumer, `main`, live-setting or release mutation is present |
| Blocking findings | 0 |

Verdict: READY for receipt, archive, signed corrective commit, pull request and
hosted Linux CI.

## Deliberate quality boundary

This slice stops at the requested 70–80% point. The cheap local contract is
fail-closed for every demonstrated inventory, publication, layer and
placeholder drift. Generated rustdoc/API-diff baselines, signed external policy
snapshots and live settings polling would add a second source and slower lane;
they remain deferred until a component or governance issue demonstrates the
need. Issue #26 already owns the required live repository activation.
