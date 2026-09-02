# Governance

The Identus SDK for Rust is governed as a Hyperledger Identus repository. This
document defines repository-specific technical decision making. It complements,
and does not replace, the [Hyperledger governing documents](https://toc.hyperledger.org/governing-documents/)
or the [Identus project policies](https://github.com/hyperledger-identus/.github).
If policies conflict, Hyperledger Foundation and project-level policy wins.

## Principles

- Public, reviewable decisions and an open contribution process.
- Maintainer authority remains human and accountable; agents produce evidence
  and proposals but do not obtain governance authority.
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
referenced from [MAINTAINERS.md](MAINTAINERS.md). They triage work, approve
architecture and releases, protect the repository boundary, appoint reviewers,
and merge accepted changes.

### Component stewards

A maintainer may record one or more component stewards in CODEOWNERS or an
issue. A steward supplies domain review for a crate or standard but does not
gain merge authority unless they are also a maintainer.

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

Contributors—including LLM agents—may propose, implement, test and review work.
They must follow the same issue, provenance, DCO, signature and evidence rules.
Agents cannot approve their own work, change repository policy, publish crates,
manage secrets or merge without explicit human maintainer authority.

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
| Administrative | typo, documentation clarification, dependency patch with no behavior change | PR and one maintainer approval |
| Component | additive implementation within an accepted contract | issue, tests/evidence, PR, component review and one maintainer approval |
| Architecture or compatibility | new crate, public API family, dependency direction, wire format, MSRV, feature policy | public design discussion, ADR, two maintainer approvals |
| Security/crypto/privacy | algorithm/profile, secret boundary, parser limits, FFI, vulnerability remediation | threat/evidence update, independent security reviewer and two maintainer approvals; private path when embargoed |
| Governance/release authority | maintainer policy, publishing ownership, protected environments, 1.0/LTS | public proposal and absolute majority of active maintainers |

Approvals must come from humans who did not author the whole change. Review
requirements are minimums; maintainers may require additional standards,
consumer or security review.

## Design process

Use GitHub Discussions for standards interpretation, architecture alternatives
and cross-repository coordination. A maintainer converts an accepted direction
into an issue with the slice contract from the
[SDK blueprint](docs/architecture/sdk-rust-blueprint.md). The implementation PR
links that issue and any ADR.

An ADR is required when a decision:

- creates or removes a public crate or dependency edge;
- chooses between plausible standards interpretations;
- affects serialized data, error taxonomy or public API compatibility;
- changes MSRV, supported targets, unsafe-code policy or release strategy;
- accepts a material security/privacy trade-off;
- imports or rejects a substantial donor implementation.

ADRs are immutable after acceptance. A later ADR may supersede one and must
explain migration and compatibility impact.

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

A governance change needs a public proposal, at least seven calendar days for
review unless it fixes an urgent policy conflict, and approval by an absolute
majority of active maintainers. The PR records why the change is compatible
with Hyperledger and Identus policy.
