# Design

## Public transition

Add `try_create_authorized_jwt_credential_request` beside the existing method.
It borrows a `TokenResponseWithAuthorizationDetails`, checks detail and dataset
indices, and confirms the detail's configuration ID appears in the matched
offer. Product code remains responsible for choosing the indices.

## Shared encoder

A private selector enum carries either a configuration ID or a Credential
Dataset identifier. A single private builder validates Bearer syntax, proof
count/size, complete Authorization bytes and complete JSON body bytes, then
writes exactly one selector field followed by the unchanged `proofs.jwt`
array. This prevents branch drift while making invalid combinations
unrepresentable.

## Errors and privacy

Out-of-range detail, out-of-range identifier and configuration mismatch use
distinct fieldless errors. The identifier is never placed in an error or Debug;
its only retained copy is inside the zeroizing JSON body.

## Risks and mitigations

- Cross-offer token reuse: exact configuration agreement fails closed.
- Selector ambiguity: a private enum emits one field only.
- Duplicate construction logic: one shared encoder owns common bounds.
- Replay: explicitly remains outside this stateless encoding slice.

## Rollback

Remove the additive method, private selector branch, three errors, tests, ADR
and canonical delta. The existing public method and wire behavior remain.
