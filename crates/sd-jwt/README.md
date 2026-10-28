# identus-sd-jwt

Bounded RFC 9901 compact SD-JWT mechanics for portable Identus adopters.

The crate issues and verifies the stable base format, processes object and
array disclosures, selects the minimal disclosure closure, and creates or
verifies holder key binding. Parsing, issuer authentication, disclosure
processing, selection, and final verification are represented by different
Rust types so unverified input cannot be mistaken for verified evidence.

```text
untrusted compact
      │ parse under byte/count/work limits
      ▼
UnverifiedSdJwt
      │ verify issuer JWS with caller-selected key
      ▼
IssuerVerifiedSdJwt
      │ process the finite disclosure graph
      ▼
DisclosureVerifiedSdJwt ── select ──► SdJwtPresentation
      │                                      │ bind exact bytes
      │ verify received key binding          ▼
      └──────────────────────────────► VerifiedSdJwt
```

Cryptography is delegated to `identus-jose` and `identus-crypto`. Entropy,
signers, verification keys, audience, nonce, and time are caller-owned effects
or policy inputs. Diagnostics and receipts do not expose claims, salts,
digests, compact values, keys, audience, or nonce.

This experimental unpublished slice does **not** implement SD-JWT VC profile
semantics, JWS JSON Serialization, HTTP, DID resolution, trust, replay,
storage, bindings, certification, or product policy. See issue #568, ADR 0193,
and the active OpenSpec change `implement-bounded-rfc9901-sd-jwt` for the
normative boundary and evidence plan.
