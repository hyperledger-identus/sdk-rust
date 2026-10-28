# Design

## State lineage

`UnverifiedRequestObject -> VerifiedRequestObject ->
RequestObjectSecurityEvidence -> VerifierAuthorizationEvidence ->
ValidatedAuthorizationRequest`.

Signature verification retains the exact public key used. Security validation
keeps audience/time/replay semantics unchanged. The new consuming async step
classifies the exact client identifier, validates scheme evidence and metadata,
and calls a trust port with the exact verified key and signed response
destination. Only its success state can
enter DCQL/routing.

## Typed scheme evidence

An SDK-owned enum distinguishes pre-registered, Federation, DID, verifier
attestation, X.509 SAN DNS, and X.509 hash. It retains no secret and exposes the
original identifier only through an explicit sensitive accessor. Redirect URI,
origin, and unknown prefixes have dedicated static failures.

The port input borrows the full/original identifier, scheme, protected header,
exact verified public key, inline metadata, trust chain, and verifier-info. It
cannot mint the successor. Its result is only accepted/rejected/unavailable;
the SDK remains owner of evidence state and redaction.

## Metadata

The verified request payload is parsed after the existing bounded scanner has
accepted it. `client_metadata` must be an object when applicable. Federation
does not supply it to the trust decision. `trust_chain` is a bounded non-empty
array only for Federation. `verifier_info` is a bounded non-empty array of
objects containing a bounded non-empty `format`, object-or-string `data`, and
optional non-empty unique `credential_ids`; every reference must match a DCQL
credential query id. Unknown metadata members remain bounded opaque JSON.

The existing encryption-plan parser continues to select one closed recipient
for `direct_post.jwt`; verifier authorization neither expands algorithms nor
claims that an encryption key authenticates the verifier.

## Evidence

Clean-room E2E cases cover each supported/unsupported scheme, full-ID and key
binding, port rejection/unavailability, header mismatch, metadata-source rules,
verifier-info references, ambiguity, redaction and direct-post encryption
regression. Generated property cases sweep first-colon and collection bounds.
Fuzzing targets the classification/metadata transition. A benchmark measures
ordinary and near-limit local validation plus a deterministic accepting port.
