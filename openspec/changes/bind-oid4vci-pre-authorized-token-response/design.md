# Design

## Authority-preserving request construction

Add a crate-private consuming decomposition to
`CredentialOfferWithPreAuthorizedServer`. During request construction, copy
the already bounded ordered configuration IDs, then consume the matched state
into Credential Issuer Metadata and selected Authorization Server Metadata.
The grant-bearing offer and caller Transaction Code are dropped after the
zeroizing request form is built. The request retains only those public lineage
values, its endpoint, zeroizing form, and transaction-code-presence evidence.

## Consuming response transition

Add `PreAuthorizedTokenHttpResponseLimits`,
`PreAuthorizedTokenResponseLineage`, exclusive
`PreAuthorizedTokenResponseOutcome`, and request-bound success/error wrappers.
`PreAuthorizedTokenRequest::try_bind_response` consumes the request into
lineage and explicitly drops its form body before validating supplied remote
metadata.

Validation order is exact status (`200` or `400`), Content-Type bound/grammar,
Cache-Control bound/bare `no-store`, Pragma bound/bare `no-cache`, then the
status-selected existing bounded body parser. Body shape never selects a
branch. The unauthenticated current request does not accept `401`.

## Reuse and error surface

Extract a crate-private generic Token Endpoint header validator returning a
small internal failure enum. Both flow-specific binders map it to their own
stable public diagnostics. This removes duplicated grammar while preserving
existing Authorization Code API and error behavior.

Append eight fieldless pre-authorized diagnostics: invalid limits/status;
oversized/invalid Content-Type; oversized/invalid Cache-Control; and
oversized/invalid Pragma. Keep the append-only 171-row error baseline and the
explicit wildcard-free router.

## Risks and mitigations

- Secret retention: never retain complete offer JSON; drop the zeroizing form
  before remote parsing.
- Detached substitution: accept no replacement endpoint, metadata, offer or
  configuration scope.
- Outcome confusion: classify by exact status first.
- Resource exhaustion: independent positive header and existing body limits.
- Coupling: share only the private generic HTTP envelope validator; keep
  flow-specific public types and diagnostics.

## Rollback

Remove the additive response module/export/limits/errors/tests/ADR, the
private shared validator and added request lineage. Existing request
construction and response-core parsing remain available.

## Decomposition decision

The production-ready diff planner reports more than 12 paths and 1,000 text
lines because this issue includes its complete issue-first OpenSpec contract,
preimplementation receipt, ADR, conformance report/matrix, roadmap handoff and
focused negative tests. The runtime change is one inseparable typed transition:
request construction must preserve the same least-authority lineage that the
new consuming response binder returns, and its private header extraction must
prove the existing Authorization Code behavior unchanged. Splitting those
pieces would create an intermediate request API that either loses authority or
retains secrets without a consuming continuation. Issue #376 already owns the
independent fixture/provenance work, so no unrelated feature is included here.
