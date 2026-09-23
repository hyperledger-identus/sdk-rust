# factory-operations

## MODIFIED Requirements

### Requirement: Contribution provenance has local and hosted enforcement

The factory SHALL validate issue branch grammar, Conventional Commit scope,
DCO and accepted commit-signature provenance, breaking-change footers and
staged secret safety through repository-owned local hooks. Mandatory
invariants SHALL also run in hosted CI because local hooks are bypassable.

#### Scenario: Invalid outgoing commit is pushed

- **WHEN** an authored outgoing commit lacks the required signature, DCO or
  valid subject, or its branch lacks an issue identity
- **THEN** pre-push fails before network mutation and names the violated rule

## ADDED Requirements

### Requirement: Accepted signature envelopes are declared and enforced fail-closed

The tracked contribution policy SHALL declare the accepted commit-signature
envelopes. Hosted provenance SHALL reject a commit that GitHub does not report
as verified and valid, a commit without a signature, and a verified commit
whose envelope is not in the declared set. The declared set SHALL include the
GitHub-verifiable OpenPGP and SSH envelopes. Local verification SHALL remain
mechanism-agnostic and SHALL report the cause of a verification failure,
including a missing allowed-signers configuration, instead of a
mechanism-specific message.

#### Scenario: Declared SSH envelope is hosted-verified

- **WHEN** a pull request commit carries an SSH signature that GitHub reports
  as verified and valid and the declared set contains the SSH envelope
- **THEN** contribution provenance passes for that commit

#### Scenario: Verified commit uses an undeclared envelope

- **WHEN** GitHub reports a commit as verified and valid but its signature
  envelope is not in the declared set
- **THEN** provenance fails and the diagnostic names the rejected envelope
  rather than reporting an unverified signature

#### Scenario: Declared set is missing or malformed

- **WHEN** the contribution policy requires a signature but declares no usable
  envelope set
- **THEN** provenance fails closed with a policy-defect diagnostic and no
  commit is accepted
