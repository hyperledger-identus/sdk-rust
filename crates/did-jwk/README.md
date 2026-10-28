# identus-did-jwk

Bounded, effect-free `did:jwk` creation, parsing, and local resolution over the
SDK's public JWK and DID types.

The first profile supports Ed25519, X25519, P-256, and secp256k1 public JWKs.
It rejects private members, `key_ops`, invalid public material, incompatible
explicit `use`, duplicate JSON names, non-canonical base64url, and inputs above
the documented work ceilings. It performs no key generation, signing, I/O,
storage, implicit registry mutation, or product policy.
