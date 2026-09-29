# factory-operations

## ADDED Requirements

### Requirement: Linked-issue verification tolerates bounded transient provider failures

The hosted pull-request policy SHALL verify the referenced issue's exact
repository identity and SHALL distinguish issue resources from pull-request
resources. It MAY retry a recognized transient GitHub transport or service
failure no more than twice after the initial attempt. It SHALL fail closed on
retry exhaustion and SHALL NOT retry permanent policy or identity failures.

#### Scenario: Transient lookup recovers

- **WHEN** an initial linked-issue lookup fails with a recognized transient
  provider error and a bounded retry returns the exact repository issue
- **THEN** issue verification succeeds without a workflow rerun

#### Scenario: Transient failures are exhausted

- **WHEN** every permitted lookup attempt fails transiently
- **THEN** issue verification fails visibly after the bounded attempt count

#### Scenario: Referenced identity is not a repository issue

- **WHEN** lookup returns a pull request, a mismatched URL, or a permanent
  missing/permission/authentication failure
- **THEN** verification fails immediately without accepting or masking it
