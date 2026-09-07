# Governance

The Identus SDK for Rust is governed as a Hyperledger Identus repository. This
document defines repository-specific technical decision making. It complements,
and does not replace, the [Hyperledger governing documents](https://toc.hyperledger.org/governing-documents/)
or the [Identus project policies](https://github.com/hyperledger-identus/.github).
If policies conflict, Hyperledger Foundation and project-level policy wins.

## Principles

- Public, reviewable decisions and an open contribution process.
- Product sponsors and maintainers define objectives and protected boundaries.
  Agents have standing authority for reversible routine decisions inside those
  boundaries, while governance and release authority remains human and
  accountable.
- Standards and executable conformance evidence outrank implementation
  precedent.
- Chain-neutral core remains independent from chain families and products.
- Security, privacy and interoperability concerns can block a release even
  when implementation tests pass.
- Decisions should be reversible until an external compatibility commitment is
  deliberately released.

## Roles

### Maintainers

Maintainers are the people listed by the canonical Identus maintainer policy
referenced from [MAINTAINERS.md](MAINTAINERS.md). They set project boundaries,
approve protected governance and release decisions, protect the repository
boundary and appoint reviewers. Routine backlog selection, architecture within
the recorded mandate and integration into `develop` may be performed by a human
or agent under ADRs 0003 and 0004.

### Component stewards

A maintainer may record one or more component stewards in CODEOWNERS or an
issue. A steward supplies domain review for a crate or standard but does not
gain protected decision or release authority. Eligible `develop` merges follow
the same issue-linked, CI-gated policy for every contributor.

### Security response team

The Identus security response team handles private reports under
[SECURITY.md](SECURITY.md). Crypto, parser, FFI and release changes require a
reviewer with the relevant security competence. Security response membership
is not inferred from ordinary repository access.

### Release managers

Release managers are maintainers assigned for a release. They verify the
release receipt, operate the protected publishing environment, sign the release
and coordinate disclosure or rollback. The author of a release change cannot
be its sole release approver.

### Contributors and engineering agents

Contributors—including LLM agents—may select, propose, specify, implement, test
and review routine work within the standing product mandate. They must follow
the same issue, provenance, DCO, signature and evidence rules. Agents may make
reversible component and architecture decisions, record them in OpenSpec or an
ADR, publish focused branches and merge eligible pull requests into `develop`
after a distinct local review pass. Agents cannot change project strategy or
governance, publish crates, manage secrets, disclose vulnerabilities, promote
to `main`, change protected repository settings or bypass protection without
the applicable human authority.

## Sources of truth

When sources disagree, use this order:

1. Hyperledger Foundation policy and applicable law/license;
2. accepted final standard and errata;
3. official conformance suites and published test vectors;
4. accepted repository ADR or versioned profile decision;
5. current Identus compatibility requirements;
6. independent interoperable implementations;
7. consumer implementations as evidence, not specification.

## Decision classes

| Class | Examples | Minimum decision path |
| --- | --- | --- |
| Administrative | typo, documentation clarification, dependency patch with no behavior change | agent- or human-created issue, local review, PR and green required CI |
| Component | additive implementation within a bounded contract | issue, reviewed OpenSpec contract, tests/evidence, component review, PR and green required CI |
| Architecture or compatibility | new crate, public API family, dependency direction, wire format, MSRV, feature policy | issue, ADR and contract, local/specialist review, PR and green required CI; escalate a material public commitment or unresolved product choice |
| Security/crypto/privacy | algorithm/profile, secret boundary, parser limits, FFI, vulnerability remediation | threat contract, independent security review and green required CI; escalate unresolved risk and use the private path when embargoed |
| Governance/release authority | maintainer policy, publishing ownership, protected environments, 1.0/LTS | public proposal and absolute majority of active maintainers before implementation; releases remain protected |

The first four decision paths are operational evidence gates, not human
approval queues. Agents may create their issues and contracts and proceed when
the decision is reversible, within the product mandate and free of unresolved
blocking risk. Protected strategy, governance, legal-risk acceptance, release
and externally binding decisions come from accountable humans under the
applicable project policy. Once any required protected decision is recorded,
the implementation PR may be merged into `develop` by a human or agent after
its required review and CI gates pass. A contract or risk-specific ruleset may
require additional standards, consumer or security review.

## Design process

Use GitHub Discussions for standards interpretation, architecture alternatives
and cross-repository coordination when asynchronous input is useful. A human or
agent converts the standing roadmap or a new direction into an issue with the
slice contract from the [SDK blueprint](docs/architecture/sdk-rust-blueprint.md).
The implementation PR links that issue and any ADR. A Discussion vote or human
acceptance ceremony is not required for routine work inside the mandate.

An ADR is required when a decision:

- creates or removes a public crate or dependency edge;
- chooses between plausible standards interpretations;
- affects serialized data, error taxonomy or public API compatibility;
- changes MSRV, supported targets, unsafe-code policy or release strategy;
- accepts a material security/privacy trade-off;
- imports or rejects a substantial donor implementation.

ADRs are immutable after integration. A later ADR may supersede one and must
explain migration and compatibility impact.

## Constraints and limitations

Material cross-cutting constraints and known unsupported surfaces are indexed
in [sdk-constraints.toml](docs/governance/sdk-constraints.toml) and explained
in [constraints-and-limitations.md](docs/governance/constraints-and-limitations.md).
The index references controlling ADRs, specifications and policy files rather
than replacing them.

Every qualifying OpenSpec change declares whether its constraint impact is
`none`, `routine` or `material`. A target, candidate, planned surface or review
date is not an effective promise. A new or changed material consumer/product
outcome requires an exact durable sponsor or responsible-maintainer direction
before implementation or activation, unless an existing effective entry or
roadmap decision already authorizes that exact outcome. Routine reversible
choices remain within standing agent authority and require no format approval.

Exceptions remain separate, bounded records with an owner, scope, rationale,
exit trigger and security/maintenance cost. They do not silently weaken the
indexed base rule.

## Consensus and deadlock

Maintainers seek lazy consensus: after the documented review period, a proposal
with the required approvals and no unresolved blocking objection may proceed.
A blocking objection must identify the violated requirement, evidence gap or
safer alternative; preference alone is not a veto.

If the required maintainers cannot resolve an objection, the proposal remains
unaccepted. The maintainers may request mediation from the wider Identus
maintainer group or the Hyperledger TOC as applicable. No agent or release
deadline breaks a governance deadlock automatically.

## Conflicts of interest

Reviewers disclose material employer, vendor or personal interests when a
decision selects a paid service, assigns publishing control, changes a security
exception, or materially advantages a specific implementation. A conflicted
maintainer may provide expertise but should not be the deciding approval.

## Repository operations

`develop` is the active integration branch and the only branch to which normal
feature pull requests are directed. It is protected from deletion, force-push
and unsigned commits. `main` remains protected, intentionally minimal and
outside the integration/release flow until a later ADR defines promotion and
activates it. Required controls are specified in
[repository-settings.md](docs/governance/repository-settings.md).

Pre-1.0 candidates may be tagged from a protected `develop` revision after the
full release gate. Populating or releasing from `main` requires an explicit
architecture/release decision; it is never an automatic mirror of `develop`.

All releases use protected, organization-controlled automation described in
[RELEASING.md](RELEASING.md). Personal tokens and local `cargo publish` are not
the normal release path.

## Changing this document

A change to protected decision classes, maintainer authority or release policy
needs a public proposal, at least seven calendar days for review unless it fixes
an urgent policy conflict, and approval by an absolute majority of active
maintainers. During the pre-release bootstrap, an explicit project-sponsor
direction recorded in a public issue and ADR may establish or refine routine
operational delegation without that waiting period when it does not override
Hyperledger or Identus policy. Routine wording and workflow maintenance follows
the ordinary issue-linked, reviewed and CI-gated `develop` path.
