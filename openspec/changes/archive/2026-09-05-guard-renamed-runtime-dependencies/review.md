# Pre-implementation review

- **Date:** 2026-09-05
- **Issue:** #93
- **Base:** `develop@04c209ac1b0fe66173d91aee6c2fc0bd7258fe12`
- **Scope:** runtime-edge identity resolution in verification code only

## Findings

1. The review finding is reproducible from the collector: it compares only the
   dependency alias against canonical workspace names.
2. Resolving the optional `package` field is the smallest complete correction;
   Cargo already requires it for renamed dependencies.
3. Keeping resolution in the shared collector applies it uniformly to layer
   checks, production-boundary checks and exact conformance-leaf assertions.
4. A set of canonical identities preserves deterministic deduplication when
   aliases or target tables repeat one package.
5. No production code, runtime behavior, public API or dependency changes.

Verdict: READY to implement after strict OpenSpec validation.

# Post-implementation review

- **Date:** 2026-09-05
- **Reviewed head:** implementation diff after `56af019`
- **Result:** no unresolved finding

## Findings

1. The collector reads the explicit `package` field only from dependency table
   declarations and otherwise uses the alias, matching Cargo rename semantics.
2. Canonical package identity is tested against the existing workspace set
   before insertion; external aliases do not enter the architecture graph.
3. Hash-set collection deduplicates canonical, renamed and repeated
   target-specific declarations before the existing stable sort.
4. The regression combines a top-level renamed foundation edge, a repeated
   target-specific wallet edge, an unauthorized target-specific DID edge and a
   renamed target dev-only edge.
5. The result proves both broad layer inspection and the stricter wallet-only
   leaf invariant see renamed runtime edges, while dev-only edges stay absent.
6. No production source, public API, dependency, layer allowance or runtime
   artifact changed.

Verdict: READY to archive and deliver under #93.

# Hosted-review amendment

- **Date:** 2026-09-05
- **Finding:** PR #94 review thread `PRRT_kwDOQ45BA86ffw9Z`
- **Reviewed implementation:** `d38b4cf`

## Finding and disposition

The initial implementation trusted a member-local `package` field even when
the declaration also set `workspace = true`. Cargo 1.85 ignores that local
override and inherits package identity from the matching root
`[workspace.dependencies]` entry, so the guard could model a different edge
from Cargo's resolved graph. The finding was accepted before merge.

The amended collector now builds the internal root key-to-package map and uses
it for every inherited top-level and target-specific runtime dependency.
Ordinary non-inherited renames retain their explicit package identity. A
regression combines a deceptive member-local override with a renamed root
entry and proves that both resolve to the Cargo-effective identities.

Verdict: RESOLVED locally; rerun the full pinned and hosted gates on the amended
head before merge.

# Current-head hosted-review amendment

- **Date:** 2026-09-05
- **Finding:** PR #94 review thread `PRRT_kwDOQ45BA86ff7f1`
- **Reviewed implementation:** `c30bed5`

## Finding and disposition

The first inheritance amendment correctly canonicalized root package identity
for the architecture graph, but reused that canonical set in the external
dependency-source guard. A renamed internal root key could therefore be
misclassified as external. The finding was accepted before merge.

The guards now derive two deliberately distinct views from the same root map:
canonical package identities for layer edges and root dependency keys for
internal/external classification. A cross-guard regression proves a member
inheriting `core-alias` produces canonical graph edge `identus-core` without
producing an external-dependency entry.

Verdict: RESOLVED locally; rerun full pinned and hosted gates before merge.
