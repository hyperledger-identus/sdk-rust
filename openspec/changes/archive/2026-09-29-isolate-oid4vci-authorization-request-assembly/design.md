# Design

## Private ownership boundary

`CredentialOfferWithAuthorizationRequestInput::try_into_authorization_request`
remains the consuming public entry point. One private assembly owner borrows
the validated predecessor and owns the limits plus bounded Authorization
Details while deriving the selected endpoint, existing-query state, optional
issuer state, fixed parameter set, checked total length, and exact zeroizing
request URI.

Semantic methods may own complete preparation, size, and render phases. Existing
`validate_existing_query`, `build_authorization_details`, `request_parameters`,
`query_len`, `BoundedJson`, and form helpers retain their full grammars. Small
helpers are acceptable only for complete invariants; there is no helper per
parameter or conditional.

## Exact assembly order

The design preserves this order:

1. require the selected server's Authorization Endpoint;
2. parse it, identify any existing query, and validate that complete query
   under count/name/value/collision rules;
3. construct bounded Authorization Details with conditional issuer location;
4. identify optional offered issuer state and enumerate fixed managed
   parameters in their current order;
5. compute encoded managed-query and complete request lengths with checked
   arithmetic, then enforce the final URI ceiling;
6. allocate one zeroizing string at the exact length and render the endpoint,
   separator, and managed form fields; and
7. consume the unchanged predecessor into the public result.

The owner cannot normalize or reorder existing query bytes, add fields, alter
form or JSON grammar, expose intermediate state, invoke transport, or broaden
trust evidence.

## Characterization boundary

Before production movement, a compact matrix binds invalid or colliding
endpoint query before too-small Authorization Details and final URI ceilings,
Authorization Details overflow before final URI overflow, and final URI
overflow before allocation/rendering. Existing tests continue to bind exact
bytes, conditional locations and issuer state, individual limits, static
errors, redaction, and predecessor lineage.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched signal
must disappear without an equivalent owner/helper or module signal,
forwarding-only chain, generated code, moved tests, waiver, new allocation, or
weaker threshold. Canonical evidence is rebound to the protected
implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are query/details/size priority drift, parameter reordering,
separator changes, conditional-location or issuer-state drift, extra copies,
lost checked arithmetic, and diagnostic disclosure. The matrix, exact byte
corpus, allocation/redaction review, and exact diff make those visible.
Rollback inlines the private owner without consumer or stored-data migration.
