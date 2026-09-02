## Context

The exact implementation base is
`develop@1d890fdc61b68ca6cd3c53dbfb39e3c03bb2ed1f`. The workspace has fourteen
packages: five implemented experimental foundations (`core`, `derive`,
`crypto`, `did`, `adapters-entropy`), one verification-only package
(`conformance`) and eight component-marker placeholders (`agent`, `bindings`,
`credentials`, `messaging`, `openid4vc`, `presentations`, `trust`, `wallet`).

The current `crate-ring-layout` specification predates the selected baseline's
implementation work. It still calls crypto, DID, entropy and conformance
"stubs" and requires placeholder manifests to declare future dependency
graphs. That contract is both factually stale and contrary to a low-coupling
bootstrap: unused dependency edges communicate architecture that has not been
accepted.

Repository governance delegates routine reversible delivery but keeps
governance, publishing, protected settings and release authority human. The
live GitHub observations recorded in #25/#26 are therefore inputs to truthful
status, not actions authorized by this change.

## Goals / Non-Goals

**Goals:**

- provide one compact, reviewable source of truth for local governance evidence
  and package maturity;
- prevent an inherited package or version from becoming publishable by
  omission;
- reduce placeholder dependency cones to the minimum truthful shape;
- expose the real implemented API families as experimental inventory rather
  than a stability promise;
- make the contract deterministic, offline and cheap enough for every factory
  run;
- leave a measured receipt and stop at the requested 70–80% quality point.

**Non-Goals:**

- changing maintainers, governance, licensing, release authority or support
  commitments;
- making the repository public, changing branch protection, enabling GitHub
  security settings or creating environments;
- reserving names, publishing crates or activating `main`;
- exact rustdoc item/API-diff generation, remote settings polling or signed
  external-policy snapshots;
- porting or changing crypto, DID, credential, protocol, FFI or wallet
  behavior;
- modifying any donor or consumer repository.

## Decisions

### 1. One repository-local bootstrap inventory

`docs/architecture/sdk-bootstrap-inventory.toml` is the normative local data.
It records repository identity, branch roles, release/publishing state,
canonical governance files and policy references, protected external follow-up
issues and every workspace package. Package entries carry path, layer,
classification, public API status and owner issue.

The adjacent Markdown document explains the actual public API families and the
limits of each classification. The TOML remains compact enough to review and
validate without parsing prose.

### 2. Three maturity classes, not future-product guesses

The only classes are:

- `implemented`: code with a real experimental runtime or macro surface;
- `verification`: repository test/guard code that is not a consumer runtime
  capability;
- `placeholder`: a component marker retained from the seed with no accepted
  namespace, API, dependency graph or support promise.

Classification says what exists now. Future issue references coordinate the
next decision but do not reserve a crate name.

### 3. Publication is denied in native Cargo metadata

The workspace declares `publish = false`, and every member explicitly inherits
it. The validator checks both sides so adding a package without the inheritance
or changing the workspace default fails before `cargo publish` can be used.
Issue #3 remains the only path to approved namespace and trusted-publishing
ownership.

### 4. Placeholder graphs stay minimal

A placeholder `lib.rs` contains only crate documentation, the
`identus_core::Component` import and `COMPONENT` constant. Its normal dependency
set is exactly `identus-core`; it has no build/dev dependencies, features,
targets, bins or examples. This removes speculative coupling while preserving
the current marker and layer guard. A future component issue replaces the
placeholder contract when real code lands.

### 5. Validation is offline and fail-closed for demonstrated drift

`scripts/check-bootstrap-inventory.py` uses Python standard-library TOML and
filesystem inspection only. It validates exact root schema/enums, required
governance paths and immutable policy-reference shapes, complete unique
workspace package/path coverage, Cargo publication inheritance and placeholder
source/manifest constraints. It does not call GitHub, Cargo or donor tools.

Focused tests run the validator against temporary repository copies and prove
rejection of missing, duplicate and unknown package entries, path drift,
publishable packages, placeholder public-source drift and speculative
dependencies. The factory invokes the canonical positive check.

### 6. Live controls remain explicitly external

The inventory records `external-action-required` and links #26. It does not
encode transient API observations as desired-state truth and cannot claim that
the repository is public or `develop` protected. That separation lets the
local evidence merge without bypassing protected administrator authority.

## Risks / Trade-offs

- Lightweight source-shape validation is intentionally narrower than a Rust
  parser. It catches added public items and extra files/dependencies in current
  placeholders; generated rustdoc/API inventory is deferred.
- Central inventory duplicates some layer data from `LAYER_RULES`. Exact
  package/path coverage and layer comparison make drift visible rather than
  silently accepting two truths.
- Keeping placeholders in the workspace retains small build and naming debt.
  Removing/renaming them is a later namespace/component decision; stripping
  speculative edges delivers the immediate cohesion benefit without widening
  scope.
- `publish = false` must be deliberately reversed per accepted package later.
  That friction is intended before organization-controlled publishing exists.

## Migration Plan

Create and review this contract first. Then add inventory/data validation,
apply Cargo publication denial, minimize placeholder manifests, update current
spec/roadmap wording and run focused plus full repository gates. Sync the
capability specs, archive the change, open one signed issue-linked PR to
`develop`, merge only after green CI, and remove the feature worktree/branches.
Rollback is a normal revert with no runtime or consumer data migration.
