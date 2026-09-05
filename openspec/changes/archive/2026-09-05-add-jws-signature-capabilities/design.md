# Design: bounded JWS signature capability registry

## Context and source audit

Issue #98 advances `IDR-004` from
`develop@54f46e9b94ce3fe66f9e2c04c389051549ac39a4`. The accepted compact codec
already preserves exact signing input and explicit unverified state.

Normative interpretation is pinned to RFC 7515, RFC 7518, RFC 8037, RFC 8725
and the October 2025 RFC 9864 update. RFC 9864 registers fully specified JOSE
`Ed25519` and deprecates polymorphic `EdDSA`. RFC 8725 requires caller-selected
allowlists plus exact key/algorithm binding. RFC 7518 requires a 64-byte raw
`R || S` ES256 signature, not DER.

The existing `identus-crypto` audit found:

- `Ed25519PrivateKey::sign` returns 64 bytes and `Ed25519PublicKey` implements
  strict verification through `ed25519-dalek::verify_strict`;
- `P256PrivateKey::sign` and `P256PublicKey::verify` use DER for inherited
  compatibility, so additive fixed-width primitive methods are required;
- `PublicKeyJwk` validates public-only OKP/EC shape, canonical 32-byte
  coordinates and structural curve/type compatibility, but deliberately does
  not bind algorithm or prove an EC point is on-curve.

No donor code or fixture is copied. RFC 8037 Appendix A is a normative vector;
other consumer-shaped fixtures are independently constructed. Oxid remains
read-only at `bfe3b481568dc738f0732c2b27548fab8721fd95` with pre-existing
`.claude/` and `.pi/taskflows/`; Lace ID Portal remains read-only at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with pre-existing
`.pi-subagents/`, `.pi/` and `tmp/`.

## Decisions

### D1 — fully specified algorithms lead; legacy is explicit

`JwsAlgorithm` is closed to `Ed25519`, `ES256` and `LegacyEdDsa`. The wire
spellings are `Ed25519`, `ES256` and `EdDSA`. The recommended registry contains
only the first two. A caller must deliberately add the legacy suite and bind
its Ed25519 key to `LegacyEdDsa` to accept existing RFC 8037-era values.

No aliasing occurs: these are separate allowlist entries even though two use
the same primitive. This follows RFC 9864 while retaining a narrow migration
path for existing wallets.

### D2 — key binding is a borrowed capability

`JwsVerificationKey<'a>` borrows `PublicKeyJwk` and binds it to one algorithm.
Construction validates the expected curve/type and, when present, the JWK
`alg` extension. Borrowing avoids cloning potentially extended JWK data. The
selected key remains authorization input supplied by the caller; `kid`
matching and DID verification relationships remain #99/#5 work.

### D3 — registry membership is the verifier allowlist

`JwsSignatureSuite` is synchronous, object-safe, `Send + Sync`, and receives
only exact public bytes plus a borrowed public JWK. `SignatureSuiteRegistry`
owns at most 16 boxed capabilities, rejects zero/excessive capacity,
duplicates and overflow, and dispatches by exact `JwsAlgorithm` equality.
The recommended constructor starts with only `Ed25519` and `ES256` but retains
the hard maximum as capacity, allowing callers to add legacy or custom suites
as explicit policy decisions.

The registry borrows the unverified value so a caller can try another
authorized key after a failed operation. A successful capability produces an
owned `VerifiedCompactJws` by taking one bounded clone, recording the algorithm
and original value. The type is cryptographic evidence only, not a claim, DID,
trust or authorization result.

### D4 — signer port is synchronous and custody-free

`JwsSigner` is synchronous, object-safe and `Send + Sync`. It receives only
the exact public signing input and returns `[u8; 64]` or
`Rejected`/`Unavailable`; invalid widths are unrepresentable at this boundary.
This is the smallest common seam for software keys, secure elements, HSMs and
agent-owned providers without guessing a Rust async runtime or remote
transport. Outer orchestration can schedule blocking work. A later async port
requires two concrete consumers and cancellation semantics.

The software adapters borrow typed `identus-crypto` private keys. They neither
accept raw secret bytes nor own key lifecycle. Header/signer equality is
enforced before a compact value is returned.
Because every closed signer returns 64 bytes, signing preflights the decoded
signature limit and calculated unpadded-base64url compact length before
invoking the capability. This avoids an HSM, remote-provider or agent side
effect for an operation that local bounds already make impossible. Attachment
reuses the same length validation.

### D5 — P-256 fixed-width primitives remain in crypto

Add `sign_fixed` and `verify_fixed` to the typed P-256 keys. They use the same
RustCrypto implementation as the retained DER API and expose the primitive's
canonical 64-byte representation. JOSE calls these methods and does not parse
DER or depend directly on `p256`. Existing DER behavior remains unchanged.

The root internal dependency disables `identus-crypto` default features.
`identus-jose` requests only `ed25519` and `secp256r1`, while the existing DID
edge explicitly retains the all-on compatibility surface and the entropy
adapter needs no algorithm feature. Selecting the crypto package itself also
retains its all-on default. This keeps unrelated derivation, COSE, X25519 and
secp256k1 code out of JOSE-only consumers without broadening this slice into a
DID/crypto-test feature refactor.

### D6 — static failures and bounded linear dispatch

New `JoseError` variants carry no untrusted values. `SignerFailure` is a small
static enum. Registry lookup is linear over at most 16 entries; this avoids a
hash dependency and makes duplicate behavior deterministic. A release-only
diagnostic measures verification throughput without a hardware threshold.

## Risks and trade-offs

- Accepting legacy `EdDSA` could prolong deprecated wire use. It is a distinct
  opt-in suite, never recommended, and visibly named legacy in Rust.
- A public verifier capability can be implemented incorrectly. Registry
  membership is an explicit caller trust decision; built-in suites and their
  conformance evidence are the SDK-provided path.
- Synchronous remote signers may block. Runtime scheduling, cancellation and
  retry behavior differ across mobile, HSM and agent transports and would be
  premature in this chain-neutral slice.
- `PublicKeyJwk` does not validate EC points structurally. The built-in P-256
  suite performs point validation when reconstructing the primitive key.
- The verified type can be cloned, but each clone preserves the same bounded
  immutable verification evidence and does not grant trust or authorization.
- The existing DID edge continues to activate the full crypto default. A
  separate feature-hygiene issue can narrow it after crypto integration tests
  are individually feature-gated; JOSE consumers do not inherit that cost.

## Rollback and migration

This is additive and unreleased. Reverting removes the capability module,
fixed-width P-256 methods and inward JOSE-to-crypto edge. Existing compact
parsing and DER crypto APIs remain unchanged. Consumer adoption and migration
from `EdDSA` to `Ed25519` are separate downstream issues.
