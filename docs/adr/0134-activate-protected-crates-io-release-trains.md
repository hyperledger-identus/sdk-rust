# ADR 0134: activate protected crates.io release trains

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-22
- **Issue:** [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326)
- **Namespace closeout:** [#3](https://github.com/hyperledger-identus/sdk-rust/issues/3)
- **Release hardening:** [#335](https://github.com/hyperledger-identus/sdk-rust/issues/335)
- **Supersedes operational parts of:** ADR 0113
- **Review no later than:** before the next crate release train

## Context

The isolated `identus-derive`, `identus-core`, and `identus-crypto`
`0.1.0-rc.1` candidate now has deterministic archives, complete package and
API evidence, an accepted compiler/target matrix, public architecture docs, a
green external consumer canary, and a successful natural weekly slow run from
protected `develop`. ADR 0113 intentionally denied publication until this
explicit decision.

Crates.io publication is immutable. crates.io trusted publishing is the desired
long-term authentication model, but it can be configured only after a crate's
first release creates its namespace. The first train therefore needs a narrow
bootstrap exception without turning a long-lived registry token into routine
CI authority.

## Decision

1. Activate explicit `0.1.0-rc.1` metadata and crates.io permission for exactly
   `identus-derive`, `identus-core`, and `identus-crypto`. Retain workspace
   `0.0.0` and `publish = false` defaults for every other member.
2. Use exact internal registry requirements and fixed publication order:
   derive, core, crypto.
3. Preserve the deterministic candidate builder as a non-publishing boundary.
   It emits the reviewed archives, clean publication workspace, and immutable
   receipt but never reads a credential or mutates a remote system.
4. Publish only through `.github/workflows/publish-crates.yml` from signed immutable tag
   `crypto-v0.1.0-rc.1` and an exact full SHA contained in protected `develop`.
5. Protect the job with GitHub environment `crates-io`, protected-ref policy,
   self-review prevention, disabled administrator bypass, and independent
   `identus-maintainers` approval.
6. Permit environment secret `CARGO_PUBLISH` only for the namespace-creating
   bootstrap. Later releases use the official crates.io OIDC action pinned to
   an immutable revision; neither mode falls back to the other.
7. Verify package checksums and availability after each upload, stop before
   dependants on failure, preserve a partial receipt, attest the exact archives,
   and create the GitHub release only after the full train succeeds.
8. Configure trusted publishers for all three crates, verify organization
   ownership and recovery/yank duties, and revoke the bootstrap token before
   issues #3 and #326 close.
9. Construct every byte-affecting publication workspace outside any Git
   worktree and copy only its clean source tree into evidence. Repeat exact
   tag/SHA/develop binding after the environment approval wait.
10. Keep candidate artifact identity stable across attempts of one workflow
    run, retain available failure evidence, and accept an existing GitHub
    release only when its tag is the exact approved release tag.

## Consequences

Consumers gain three registry-addressable, chain-neutral experimental crates
without turning the monorepo into a single versioned product. Release managers
pay a deliberate two-person approval and receipt cost only on release trains;
ordinary fast CI remains unchanged.

The first upload temporarily depends on one secret. That exception is bounded
to the protected environment and closes only when all trusted publishers are
configured and the secret is revoked. A partial publication cannot be rolled
back by deletion; the train must stop and maintainers decide whether to yank
before a corrected version is released.

## Alternatives rejected

- **Publish every workspace crate:** creates unsupported namespaces and APIs.
- **Local maintainer upload:** loses hosted immutable evidence and environment
  approval.
- **Permanent registry token:** unnecessarily expands credential lifetime.
- **Automatic tag-triggered upload:** couples tag creation too closely to the
  irreversible step and omits an explicit mode/receipt review.
- **Third-party release orchestrator now:** adds a broader dependency and
  policy surface than the three-package first train requires.

## Verification and rollback

Package, candidate, workflow, and publisher mutation tests bind the exact
scope, versions, tag/SHA, protected environment, credentials, action pins,
order, retry identity, post-approval rebinding, failure evidence, and receipts.
Actionlint, factory checks, Cargo package verification,
the accepted compiler/target matrix, protected PR CI, tag signature, and
environment approval are mandatory.

Before publication, revert this ADR and activation. After publication, never
retag or overwrite: preserve evidence, stop adoption, yank only by maintainer
decision, and publish a corrected version through the full release process.
