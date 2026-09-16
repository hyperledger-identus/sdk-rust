# ssi-upstream-program delta

## ADDED Requirements

### Requirement: IDR-004 delivery is functional and evidence-bounded

The canonical backlog SHALL mark IDR-004 `delivered` when every named
chain-neutral crypto and JOSE surface has immutable implementation evidence,
the executable Apollo comparison contains no open capability gap, standards
and compatibility vectors plus fuzz/coverage evidence pass, portable target
evidence is stated at its honest tier, and remaining limitations have separate
owners.

IDR-004 delivery SHALL NOT imply registry publication, SemVer or support
activation, production consumer adoption, language-binding support, mobile or
browser runtime proof, certification, Apollo deprecation, or NeoPRISM code
removal. Those outcomes SHALL remain under IDR-011, IDR-044, consumer adoption,
and downstream lifecycle authority.

#### Scenario: Functional evidence is complete

- **WHEN** every IDR-004 outcome is implemented or explicitly accepted with
  immutable evidence and the Apollo manifest reports zero gaps
- **THEN** IDR-004 may retain closed issue #286 as its delivered evidence owner

#### Scenario: Release or adoption is incomplete

- **WHEN** the candidate remains unpublished or a downstream adoption PR is
  open
- **THEN** IDR-004 may remain functionally delivered while release and
  lifecycle claims remain false under their separate owners

#### Scenario: Hardening remains independently owned

- **WHEN** a delivered surface has a bounded non-functional hardening issue
- **THEN** the report names that issue and limitation without representing the
  underlying functional capability as absent or the hardening as complete
