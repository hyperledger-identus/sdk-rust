# Documentation, architecture and claim-boundary review

Review status: completed
Review date: 2026-09-26
Base: develop@cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16
Implementation head: df1ce997f93b5efeff82928e9f1218623100944f
Reviewed head: df1ce997f93b5efeff82928e9f1218623100944f
Specification commit: 521b771c5c1e1e5060414d9d202f1ee48298862b
Preimplementation commit: 4bd8b711067df296fa44e7847dea23974b2205d4
Unresolved blockers: none

## Findings

1. **Lifecycle honesty — accepted.** The handbook keeps the published crypto
   train and candidate-only DID train distinct. It corrects the stale M3
   approval table from the immutable release receipt while retaining open
   tokenless ownership/recovery hardening. No DID tag, release, workflow,
   registry artifact or publication is claimed.
2. **Architecture — accepted.** The tracked diagram and responsibility table
   place generic DID values, documents and resolver ports in `identus-did`.
   Axum/content-negotiation/OpenAPI mapping remains in the optional HTTP
   adapter. DID methods, ledgers, persistence, custody and product policy stay
   outside both packages.
3. **Adoption identity — accepted.** DID examples use full protected
   `develop@cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16`, commit the lockfile by
   policy and avoid branches, pull-request refs, the uncreated reserved tag,
   canonical `0.0.0`, or a registry selector.
4. **Evidence boundary — accepted.** Archive, API-origin and normalized SBOM
   hashes match the #384 verification receipt. The page distinguishes an API
   origin from a stable promise and an SBOM from advisory scanning,
   attestation, certification or a vulnerability-free claim.
5. **Security and privacy — accepted.** The change adds static Markdown and DOT
   only. It adds no executable client code, dependency, analytics, cookie,
   authentication, credential, secret, PII, network input or release
   authority.
6. **Navigation and rendering — accepted.** The page is reachable from the
   handbook summary, landing page, crate index, adoption guide, readiness page
   and limitations. The locked build rendered both diagrams and all pages;
   offline link validation reported zero errors.
7. **Delivery integrity — accepted.** Planning, immutable preflight and
   implementation are separate signed/DCO commits. The preimplementation
   receipt predates documentation changes and binds issue #386 and the exact
   protected base.

## Decomposition decision

The base-to-implementation diff spans 18 paths and 525 added lines. Eight paths
and 286 lines are mandatory OpenSpec planning/receipt records. Product output
is one 77-line review page, one 37-line diagram source and focused corrections
to eight small handbook pages. Splitting the page from the global corrections
would temporarily preserve contradictory lifecycle statements; splitting the
diagram would leave the new page unrenderable. There is no Rust, workflow,
release or dependency change, so no further functional decomposition is
warranted.

## Residual limitations

- Rust/compiler and platform qualification belongs to #387.
- Final exact-SHA M5 review belongs to #388.
- Trusted-publishing and ownership administration belongs to #344/#3.
- Site deployment happens only after merge to protected `develop`; the issue
  must retain that hosted receipt.
- Downstream adoption remains separately owned and is not changed here.

## Review decision

The change is cohesive, reversible, accurately scoped and suitable for
engineer review. No unresolved correctness, architecture, security, privacy,
compatibility, dependency, documentation or release finding remains.
