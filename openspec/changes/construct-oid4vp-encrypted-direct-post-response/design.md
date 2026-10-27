# Design

## Architecture

```text
verified JAR payload
  ├─ response_mode = direct_post.jwt
  ├─ client_metadata.jwks ── bounded selection ──┐
  ├─ enc preferences/default ────────────────────┤
  └─ response_uri/state/DCQL ────────────────────┤
                                                  v
                                  ValidatedAuthorizationRequest
                                                  │ consume
caller-selected bounded presentations ────────────┤
                                                  v
                                      Final response JSON
                                                  │
                                                  v
                              identus-jose JweEncrypter port
                                                  │
                                  bounded compact JWE result
                                                  v
                           response=<percent-encoded-compact-JWE>
```

The dependency direction remains OID4VP → JOSE → crypto/core. The JOSE port
knows algorithms, key material and bounded bytes but no OpenID claims. OID4VP
knows metadata, payload and response modes but no engine implementation.

## SDK-owned JWE boundary

`JweRecipientPublicKey` validates a 65-byte uncompressed P-256 SEC1 point and
optional bounded `kid`. `JweEncryptionRequest` owns that key, exact algorithm
enums, zeroizing plaintext and maximum compact bytes. `JweEncrypter` returns
`CompactJwe`, whose constructor validates five non-empty/allowed compact
segments as applicable and the caller's bound. Errors map to one static
`EncryptionFailed` category at the protocol boundary.

The trait is synchronous because the operation is CPU/randomness only. HSM or
remote key services are not required for public-recipient encryption; an
adapter needing async work must perform it outside this narrow implementation
or motivate a separate capability.

## OID4VP metadata and transition

The verified request parser retains a bounded `client_metadata` object only for
`direct_post.jwt`. The parser rejects duplicate members before serde, limits
object/array/string/node/depth work, and admits one supported public key after
deterministic selection. `encrypted_response_enc_values_supported` is optional;
absence selects A128GCM and presence must include exactly one supported choice
without duplicates or empty values.

Success payload construction shares the current DCQL/cardinality and JSON
streaming code. Plain mode emits `vp_token` and state as form members. Protected
mode puts those members in the plaintext JSON, calls the port, then emits only
`response`. Neither branch performs HTTP.

## Verification

- official/clean-room metadata and response examples;
- negative keys, algorithms, use/operations, private members, ambiguity,
  duplicate JSON, malformed points, mode mismatch and output limits;
- property tests for accepted key/metadata normalization and compact envelope
  grammar;
- fuzz targets for metadata selection and returned compact material;
- benchmarks for selection, payload construction and port boundary overhead;
- E2E test spanning invocation through form decode with an exact request
  recorder, plus isolated oracle encrypt/decrypt qualification;
- diagrams and adopter guide kept next to the canonical spec.

## Engine adapter follow-up

A production adapter must be a separately selectable crate or feature, pin one
exact engine/version, map all errors to SDK categories, pass native and portable
runtime evidence, and never change OID4VP DTOs. Replacing the engine must not be
a protocol source break.

