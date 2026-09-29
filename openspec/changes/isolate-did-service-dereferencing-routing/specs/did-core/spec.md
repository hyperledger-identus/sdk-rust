# DID service dereferencing ownership

## ADDED Requirements

### Requirement: Generic service dereferencing preserves phased authority

The SDK MUST keep generic DID service selection and result routing in one
private owner that expands an optional selector, selects matching services in
document order, rejects an empty selection, and routes the result according to
endpoint-forcing and accepted-media policy.

#### Scenario: Services are selected and returned as a filtered document

- **WHEN** bounded service and service-type selectors match document entries
  and neither a relative reference nor fragment forces endpoint output
- **THEN** the same services are retained in source order and returned with the
  same effective DID document content type
- **AND** no additional service or endpoint interpretation is performed.

#### Scenario: Endpoint output is requested or forced

- **WHEN** `text/uri-list` is accepted or `relativeRef` or a DID URL fragment
  forces endpoint output
- **THEN** the same selected services are passed to existing endpoint
  projection and URI-resolution policy with identical metadata and errors.

#### Scenario: Selection and representation are both invalid

- **WHEN** selector expansion fails or no service matches while the explicit
  representation is also unsupported
- **THEN** the same selector or `notFound` error is returned before media
  routing and no caller-controlled value enters diagnostics.

#### Scenario: Service dereferencing remains bounded and cohesive

- **WHEN** the private owner selects services and routes a result
- **THEN** existing document, selector, endpoint, relative-reference, and
  content limits remain authoritative
- **AND** no new network retrieval, allocation class, ambient I/O, dynamic
  dispatch, synchronization, dependency, metric waiver, or forwarding-only
  helper chain is added.
