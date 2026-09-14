# sdk-governance-evidence Specification

## MODIFIED Requirements

### Requirement: Governance evidence is local and release authority stays protected

The inventory SHALL require the repository's license, maintainer inheritance,
governance, contribution, DCO, security, release, conduct, ownership and
repository-settings records. It SHALL identify human maintainer release
authority and SHALL record live GitHub activation as
`active-with-enterprise-deviation` under #26 while the dated settings receipt
identifies each active control and remaining enterprise-owned deviation.

#### Scenario: Required governance record disappears

- **WHEN** a required local governance path is missing or empty
- **THEN** offline structural validation fails

#### Scenario: Local evidence records partial live activation

- **WHEN** the protected `develop` ruleset and repository security/community
  controls are active but enterprise policy prevents a scanning control
- **THEN** IDR-001 records the live merge boundary and the remaining deviation
  explicitly without treating local evidence as authority to bypass it
