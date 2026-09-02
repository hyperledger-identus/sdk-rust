# Maintainers

This repository is maintained by the Hyperledger Identus project. To avoid a
stale or contradictory people list, the active and emeritus maintainers are
defined in the canonical
[Identus MAINTAINERS.md](https://github.com/hyperledger-identus/.github/blob/main/MAINTAINERS.md).
Hyperledger permits a repository maintainer file to reference the project's
canonical list.

## Repository scopes

| Scope | Responsibility | Expected GitHub role/team |
| --- | --- | --- |
| Maintainer | Repository, roadmap, review, merge and release accountability | Maintain / `identus-maintainers` |
| Triage | Issue classification and contributor support | Triage |
| Component steward | Required domain review for recorded crates or standards | Read or Triage; no implicit merge authority |
| Security response | Private vulnerability handling under Identus security policy | Private security infrastructure |
| Release manager | Protected release environment and signing for an assigned release | Time-bounded release authority |

The canonical Identus list determines who holds Maintainer scope. Repository
settings may grant narrower operational scopes but cannot silently create a
maintainer.

## Duties

SDK-Rust maintainers additionally agree to:

- protect the chain-neutral Layer 1 boundary;
- keep standards/profile pins and conformance evidence current;
- review dependency, crypto, parser, privacy and FFI risk;
- maintain contribution, security, release and support policies;
- ensure crates.io ownership is organization-controlled and recoverable;
- keep at least two informed reviewers for every stabilized component;
- participate in vulnerability response and release coordination;
- document inactivity, succession and emeritus transitions promptly.

## Adding or removing maintainers

Follow the process in the canonical Identus maintainer policy and the
[Hyperledger maintainer guidelines](https://toc.hyperledger.org/guidelines/MAINTAINERS-guidelines.html).
A repository-only maintainer change cannot bypass the project process.

Component-steward assignments may be changed by a normal PR with approval from
a maintainer and the proposed steward. Security-response membership follows the
private and public process in the Identus security policy.
