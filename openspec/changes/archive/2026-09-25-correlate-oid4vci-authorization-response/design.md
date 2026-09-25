# Design

## State transition

`AuthorizationRequest::try_into_authorization_response(query, limits)` consumes
the request and returns `AuthorizationResponseOutcome`:

- `Authorized(CorrelatedAuthorizationCode)` retains the request, a zeroizing
  authorization code and closed issuer-identification evidence;
- `Error(AuthorizationErrorResponse)` retains a classified exact error code,
  optional explicitly untrusted developer fields and the same closed issuer
  evidence, but no request secrets.

The success state provides the only path toward a later code-exchange request.
No response outcome is Clone or generally serializable.

## Parsing order

1. Validate positive limits and complete encoded query bytes.
2. Reject empty input; split bounded fields and strict-form-decode bounded
   names/values.
3. Reject empty names and every decoded duplicate; retain only recognized
   fields while validating/discarding extensions.
4. Require returned state and compare exact decoded text with the request.
5. Apply the selected server metadata's RFC 9207 support flag and exact issuer
   comparison.
6. Require exactly one success/error branch and validate branch-specific
   syntax and limits.
7. Construct a redacted owned outcome, dropping input values not needed by
   the selected branch.

This ordering prevents attacker-supplied codes or descriptions from becoming
usable before transaction correlation succeeds.

## Metadata projection

The existing scanner recognizes
`authorization_response_iss_parameter_supported` only as a boolean. The core
stores `Option<bool>` and exposes advertised/effective accessors; effective
omission is false. Type confusion uses the existing static invalid-metadata
contract and unknown metadata remains bounded/discarded as before.

## Grammar reuse

The exact form decoder remains private in `form`. NQSCHAR and URI-reference
character checks move from the Token Error Response module into a private
`oauth` module and retain token-error regression tests. The new parser uses
`fluent_uri::UriRef`, already in the dependency cone, for `error_uri`.

## Errors

A focused private Authorization Response catalogue owns static diagnostics for
invalid limits, encoded size/count/components, form/duplicate structure,
state/issuer correlation, invalid branch, and invalid/oversized code/error
roles. Remote text never appears in errors. The public router remains explicit
and wildcard-free.

## Rollback

A focused revert removes the response types/parser, RFC 9207 metadata
projection, private OAuth factoring, diagnostics and evidence. The #354
request constructor and every earlier state remain usable and unchanged.
