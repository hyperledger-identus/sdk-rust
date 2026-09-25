# Design

## Public surface

- `CredentialEndpointResponseLimits` composes existing immediate, deferred and
  Credential Error HTTP limit values.
- `JwtCredentialRequest::try_into_credential_endpoint_response` consumes the
  request and accepts status, effective Content-Type, borrowed body and limits.
- `CredentialEndpointResponseOutcome` is a closed enum of request-bound
  `Issued`, `Deferred`, and `Error` states.
- Each bound state exposes the originating proof count and a borrowed/consuming
  response accessor; Debug remains redacted.

## Dispatch and precedence

1. Read the trusted local proof count, then drop the consumed request so its
   zeroizing Authorization/body are erased before remote parsing.
2. Classify status without inspecting media/body: 200, 202, 400, or invalid.
3. Apply the selected existing Content-Type bound and JSON grammar.
4. Invoke only the selected bounded existing response parser.
5. For 200, reject credentials exceeding the request proof count.
6. Wrap the parsed state with the exact proof-count evidence.

## Internal reuse

Extract the current immediate binder to a crate-private function parameterized
by proof count. The old borrowed method and the new consuming 200 branch call
it. The 202 branch directly reuses `DeferredCredentialResponseCore::parse` and
the 400 branch calls `CredentialErrorResponseCore::parse_http_response`.

## Error and privacy model

Append only `InvalidCredentialEndpointHttpStatus`. Existing media/body/count
errors remain branch-specific and deterministic. No error/Debug/Display value
contains endpoint, bearer token, proof, transaction, credential or remote body.

## Rejected alternatives

- Mutating the borrowed method would break compatibility and retain replay.
- Parsing body shape before status permits ambiguous branch selection.
- Treating 401 as a Credential payload error conflates RFC 6750 authorization
  errors with section 8.3.1.2.
- Retaining bearer authority for deferred continuation is a separate lifetime
  decision assigned to #368.
