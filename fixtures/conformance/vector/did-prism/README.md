# PRISM DID Vector Fixtures

These fixtures come from the Identus docs repository:

`documentation/develop/cloud-agent/deterministic-did-creation.md`

They pin deterministic PRISM DID seed and key derivation behavior before the
Rust implementation exists. The docs currently provide mnemonic, seed,
derivation path, private key, and compressed public key. The method-specific DID
hash and long-form DID should be added after the Rust PRISM protobuf serializer
is implemented.

`prism-operation-vdr-lifecycle.json` captures the Docker-free PRISM create,
update, deactivate, and resolve lifecycle expected by the SDK core. It also
records VDR ports, states, metadata, secure-depth handling, and the
infrastructure fixture that must be used for NeoPRISM adapter tests.
