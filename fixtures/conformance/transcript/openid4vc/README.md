# OpenID4VC Transcript Fixtures

These fixtures seed Docker-free transcript replay for OpenID4VC protocol
porting. They are not production HTTP captures; they are schema-stable,
redacted source evidence that each supported flow can be classified and mapped
to expected states before network, browser, device, OIDC provider, federation,
or trust-anchor infrastructure is introduced.

## Fixture Policy

Each fixture file contains:

- `schema_version`: fixture schema version.
- `specification_ids`: conformance catalog ids covered by the transcript.
- `source`: public specification references or Identus repository evidence.
- `owner_crate`: `identus-openid4vc`.
- `participants`: issuer, wallet, holder, verifier, authorization server, and
  trust roles using fixture-safe identifiers.
- `messages`: ordered logical protocol messages with `id`, `flow`, `type`,
  `from`, `to`, and redacted body notes.
- `expected_states`: state names the future protocol runner must assert.
- `redaction_policy`: guarantee that no private keys, claims, credentials,
  signatures, bearer tokens, or production identifiers are embedded.

The first Rust executable check validates schema fields and flow coverage.
Later increments must replay these transcripts through OpenID4VC state
machines and add positive and negative vectors for signed payloads.
