# OID4VP encrypted response boundary

Issue #535 and ADR 0181 define the first OpenID4VP Final `direct_post.jwt`
boundary. This page is the adopter-oriented knowledge view; the canonical
requirements remain in OpenSpec.

## Elevator summary

SDK-Rust owns the exact OpenID4VP protocol rules and a small, replaceable JWE
capability interface. It does not duplicate encryption algorithms or make a
large JOSE library part of every wallet. Applications can adopt the stable
protocol contract now, while a separately qualified engine adapter can be
added or replaced without changing protocol DTOs.

## Responsibility map

```text
identus-oid4vp
  owns: Final metadata, key selection policy, DCQL lineage, response JSON/form
  does not own: ECDH, AES-GCM, randomness, network, trust, consent
        │
        ▼
identus-jose
  owns: bounded P-256/ECDH-ES/A128GCM request/result and encrypter port
  does not own: OpenID claims, a concrete engine, transport, key custody
        │
        ▼
future adapter
  owns: one qualified Rust engine, randomness/runtime evidence, error mapping
```

## Why no engine is embedded yet

The stable candidate passed target compilation but brought roughly 80 new
packages and unrelated algorithms. The modular candidate was smaller, but its
current release candidate requires recipient private material for ECDH
encryption and contains unsafe JSON code. Neither is a sound SDK default today.
The decision is deliberately reversible and has explicit reconsideration gates
in ADR 0181.

## Final-profile facts

- `direct_post.jwt` means unsigned encryption in OID4VP 1.0 Final.
- The verifier public JWK supplies `alg`; the protected header repeats optional
  `kid`; metadata supplies `enc` or defaults to `A128GCM`.
- The plaintext is the normal Authorization Response JSON.
- The POST form contains only `response=<compact-JWE>`.
- Encryption protects confidentiality and integrity; it does not authenticate
  the wallet.

## E2E acceptance path

```text
invocation → request_uri envelope → signed JAR → metadata/key selection
  → DCQL selection → response JSON → JWE port → compact envelope → form decode
```

Every arrow through the port boundary is covered by the versioned
`oid4vp.encrypted-direct-post-response.v1` packet, exact request recording,
negative limits, compact-header validation and redaction checks. The packet's
ciphertext is deliberately inert: it proves the protocol/adapter contract, not
ECDH or AES-GCM correctness. Network delivery remains an adapter concern and is
not simulated as protocol success.

## Adopter sequence

1. Verify the signed Request Object and call
   `into_authorization_request_with_jwe_limits`.
2. Apply verifier trust, disclosure and End-User consent policy outside the
   crate, then construct bounded `DirectPostQueryResult` values.
3. Supply a `JweEncrypter` implementation and call
   `into_encrypted_direct_post_response`.
4. Explicitly reveal the inert target and form body only in the transport
   adapter after applying DNS, TLS, redirect, timeout and SSRF policy.

The encrypter receives only an SDK-owned `JweEncryptionRequest`: normalized
public SEC1 bytes, optional bounded `kid`, closed algorithm enums, zeroizing
plaintext and a maximum compact size. It must return `CompactJwe`; its
`Debug`, errors and the protocol response expose lengths/categories rather
than keys, plaintext, state, presentations or ciphertext.

## Supported and deferred surface

| Surface | Current state |
| --- | --- |
| One public P-256 recipient | Supported |
| Direct `ECDH-ES` + `A128GCM` | Supported boundary |
| Compact JWE and response-only form | Supported boundary |
| Built-in cryptographic engine | Deferred to a separately qualified adapter |
| X25519/P-384, key wrap, CBC-HMAC, compression, multiple recipients | Not supported |
| HTTP, trust, consent, presentation verification, wallet authentication | Outside this crate |

## Evidence and maintenance

- focused E2E: `crates/oid4vp/tests/encrypted_direct_post_response.rs`;
- JOSE grammar and redaction: `crates/jose/tests/jwe_port.rs`;
- fuzz: `scripts/fuzz-oid4vp.sh smoke encrypted_direct_post_response`;
- benchmark: `cargo bench -p identus-oid4vp --bench encrypted_direct_post_response`;
- normative packet:
  `docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json`.

Reconsider the engine decision only through ADR 0181's stable-version,
dependency-cone, public-recipient, unsafe-code, advisory and portable-runtime
gates. Replacing an engine must not change the OID4VP DTOs or errors.
