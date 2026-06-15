# DIDComm Transcript Fixtures

These fixtures seed Docker-free transcript replay for DIDComm protocol porting.
They are not production wire fixtures yet; they are schema-stable source
evidence that each supported protocol family can be parsed, classified, and
mapped to an expected state transition before pack/unpack is introduced.

## Fixture Policy

Each fixture file contains:

- `schema_version`: fixture schema version.
- `specification_ids`: conformance catalog ids covered by the transcript.
- `source`: Identus repository evidence or public specification reference.
- `participants`: logical DIDComm peers using fixture-safe DID values.
- `messages`: ordered plaintext messages with `id`, `type`, `from`, `to`,
  optional `thid`/`pthid`, and minimal body notes.
- `expected_states`: state names the future protocol runner must assert.
- `redaction_policy`: guarantee that no private keys, claims, tokens, or
  production identifiers are embedded.

The first Rust executable check only parses every message `type` through
`identus-messaging`. Later increments must replay these transcripts through
protocol state machines and add negative fixtures.
