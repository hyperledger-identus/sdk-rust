# Design

## Measurement boundary

`crates/crypto/examples/crypto_baseline.rs` uses only stable `Instant` and
`black_box`. Keys, messages, mnemonics, signatures and derivation parents are
created once before measurement. Each named operation warms one batch, then
records at least 20 batch samples as integer nanoseconds per operation.
The CLI caps a run at 10,000 samples to bound memory and repeated cryptographic
work from malformed or accidental invocations.

The fixed matrix covers SHA-256/SHA-512, Ed25519 sign/verify, X25519 agreement,
secp256k1 sign/verify, P-256 sign/verify, BIP-39 seed derivation, secp256k1
BIP-32 hardened child derivation, and Cardano V2 private/public child
derivation. Entropy generation is deliberately excluded.

## Artifact contract

The example emits one JSON document on stdout. Metadata includes schema,
measurement-only status, explicit unavailable Apollo comparison, Git revision,
Rust compiler, OS, architecture, CPU description, feature profile, sample
count and warm-up batches. Each operation records its fixed batch size, p50,
p95, minimum and maximum nanoseconds per operation.

Environment metadata enters through bounded environment variables populated by
`scripts/benchmark-crypto.sh`. The runner validates sample and metadata shape,
builds release mode, atomically writes an optional output file, validates the
JSON, and never echoes secret input.

## CI placement

The existing weekly/manual `slow` workflow gains one Ubuntu benchmark job. It
runs independently from the host/target check matrix and uploads the JSON with
a retention period. Pull-request fast CI is unchanged. The workflow action is
pinned to an immutable commit.

## Parity report integration

The Apollo parity TOML gains one exact `performance` table. Its validator
requires the measurement-only status, issue, sample minimum, safe harness and
runner paths, and immutable evidence links. The renderer emits the explicit
non-comparison and limitations.

## Failure and rollback

Invalid CLI input, missing metadata, operation failure, malformed JSON or fewer
than 20 samples fails nonzero and publishes no final artifact. Rollback removes
the whole diagnostic surface; no library migration is required.
