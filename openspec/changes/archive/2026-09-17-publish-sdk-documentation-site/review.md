# Local architecture, security, and factual review

- **Review date:** 2026-09-17
- **Reviewed implementation:** PR #327 delivered tree
- **Result:** passed after local and hosted review findings were resolved

## Architecture and product truth

- The handbook describes only the intended `identus-derive`, `identus-core`,
  and `identus-crypto` candidate closure. It does not turn other workspace
  packages into release or support promises.
- Dependency arrows point toward the generic center. Chain, protocol, wallet,
  custody, trust, runtime, storage, and certification concerns remain at their
  owning boundaries.
- Crate capabilities, feature names, public examples, and non-goals were
  checked against the three manifests and current public source.
- The release page distinguishes implemented code, reproducible candidate,
  published artifact, supported software, and certified product evidence.

## Build and publication security

- The site is static, contains no analytics or mutable browser dependency, and
  generates diagrams from reviewed DOT sources.
- Nix supplies mdBook, Graphviz, and lychee from the locked package graph. The
  output rejects symbolic links and broken local links.
- Every Pages action is pinned to an exact commit. The build job has only
  `contents: read`; the isolated deployment job alone receives `pages: write`
  and `id-token: write`, through the `github-pages` environment.
- Deployment runs only for a path-matching push to protected `develop`; there
  is no feature-branch or manual publication route. The build job uploads the
  artifact directly without Pages API authority. It does not populate `main`,
  publish a crate, or handle a secret.

## Operability and rollback

The public URL, workflow run, source revision, and issue links form the
deployment receipt. Rollback is bounded to disabling Pages and reverting the
workflow, site, Nix module, script, README link, ADR, and capability spec. No
consumer repository or persisted application state is mutated.

No unresolved blocking local finding remains. Hosted CI, exact-head review,
Pages activation, public retrieval, and engineer release approval remain
independent delivery and milestone gates.
