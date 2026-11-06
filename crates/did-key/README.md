# identus-did-key

Bounded, effect-free `did:key` creation, parsing, and local resolution for the
SDK's strict v0.9 profile.

The crate accepts caller-owned Ed25519, X25519, P-256, and secp256k1 public
keys. It does not generate or store keys, derive X25519 material, perform I/O,
or register itself globally. See the SDK handbook for the support and evidence
matrix. Non-canonical or weak Ed25519 public keys and non-canonical or
low-order X25519 public keys are rejected, and
resolver negotiation adds DID Core and Multikey contexts only for
`application/did+ld+json`.
