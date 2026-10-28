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

The closest stable candidate, `jose-rs 0.10.0`, passes a public-encrypt/private-decrypt
round trip and host, WASM, iOS and Android compilation. Its exact graph still
pulls about 120 packages and mandatory `rsa 0.10.0-rc.19`, which the repository
audit rejects under RUSTSEC-2023-0071. Disabling default features does not remove
that direct and transitive edge. The modular candidate is smaller, but its
current release candidate requires recipient private material for ECDH
encryption and contains unsafe JSON code. A promising alternative requires Rust
1.99 while the SDK supports Rust 1.89. None is a sound SDK default today.

Issue #538 and ADR 0192 therefore retain the passing independent engine as an
isolated oracle but reject production adoption. This is not an advisory waiver:
the dependency never enters the workspace lock, and the differential debt stays
open. The decision is deliberately reversible through explicit gates in ADRs
0181 and 0192.

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
  → DCQL selection → response JSON → JWE port → compact envelope → exact form
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

The initial compact-header profile is deliberately closed. An adapter must
emit exactly `alg`, `enc`, `epk` (`kty`, `crv`, `x`, `y`) and an optional
non-empty `kid`. Registered extensions such as `typ`, `cty`, `apu`, `apv`,
`zip`, and `crit`, as well as unknown members, are rejected until a separately
specified profile defines their semantics and limits.

## Supported and deferred surface

| Surface | Current state |
| --- | --- |
| One public P-256 recipient | Supported |
| Direct `ECDH-ES` + `A128GCM` | Supported boundary |
| Compact JWE and response-only form | Supported boundary |
| JWK selection | `alg=ECDH-ES` required; optional `use=enc`; optional `key_ops` must contain `deriveKey` or `deriveBits` |
| JWK key identifier | Optional, non-empty printable ASCII without spaces, at most the configured bound |
| Multiple eligible recipients | Rejected as ambiguous; unsupported public keys are skipped, private members reject the request |
| Built-in cryptographic engine | Deferred to a separately qualified adapter |
| X25519/P-384, key wrap, CBC-HMAC, compression, multiple recipients | Not supported |
| HTTP, trust, consent, presentation verification, wallet authentication | Outside this crate |

## Evidence and maintenance

- focused E2E: `crates/oid4vp/tests/encrypted_direct_post_response.rs`;
- JOSE grammar and redaction: `crates/jose/tests/jwe_port.rs`;
- fuzz: `scripts/fuzz-oid4vp.sh smoke encrypted_direct_post_response`;
- benchmark: `cargo bench -p identus-oid4vp --bench encrypted_direct_post_response`;
- isolated engine E2E: `python3 scripts/qualify-jwe-engine.py --mode all`;
- static fail-closed contract: `python3 scripts/check-jwe-engine-qualification.py .`;
- normative packet:
  `docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json`.

The packet contains inert compact material and is not independent encryption
or decryptability evidence. The isolated engine E2E encrypts with a public-only
P-256 JWK, checks `alg`, `enc`, `kid`, public `epk` and the empty direct-agreement
segment, then decrypts with the corresponding private test JWK and compares the
plaintext. It also compiles the exact package for host, WASM, iOS and Android
and requires the exact current audit rejection. Because the engine is not in the
production graph, this satisfies independent feasibility evidence but not the
production-adapter debt owned by #538.

Reconsider the engine decision only through ADRs 0181 and 0192's stable-version,
dependency-cone, public-recipient, unsafe-code, advisory and portable-runtime
gates. Replacing an engine must not change the OID4VP DTOs or errors.
