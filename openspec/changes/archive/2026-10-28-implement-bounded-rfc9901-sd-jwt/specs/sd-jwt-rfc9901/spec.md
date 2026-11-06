# sd-jwt-rfc9901 delta

## ADDED Requirements

### Requirement: RFC 9901 compact values use staged SDK-owned states

The SDK SHALL parse only the RFC 9901 compact serialization into SDK-owned
unverified values. Issuer-signature verification, disclosure processing, and
holder key-binding verification SHALL produce distinct consuming state types;
an earlier state SHALL NOT expose an API that represents later evidence as
verified.

#### Scenario: structurally valid credential has not been verified

- **WHEN** a bounded SD-JWT compact value parses successfully
- **THEN** the result exposes bounded structural metadata but cannot produce a verified receipt or disclosed claims until the issuer signature and disclosure graph pass their separate stages

#### Scenario: an issuer signature is accepted

- **WHEN** the caller supplies an algorithm-bound public key and allowed JOSE verification suite for the exact issuer JWS
- **THEN** the engine returns an issuer-verified state without making a trust, authorization, status, or holder-binding decision

### Requirement: issuance and disclosure selection preserve exact RFC semantics

The SDK SHALL issue object and array disclosures, insert caller-requested
decoys, and select disclosures using RFC 9901 digest and compact rules. Salt,
digest, and signing effects SHALL be explicit. Disclosure order SHALL NOT
change the disclosed claim result where RFC 9901 defines order as irrelevant,
and the engine SHALL preserve the exact issuer JWS bytes used by key binding.

#### Scenario: selected claims form a presentation

- **WHEN** a holder selects a valid closed set of claim paths from an issued or issuer-verified credential
- **THEN** the presentation contains exactly the required disclosure closure, the original issuer JWS, and no unselected disclosure

#### Scenario: decoys are requested

- **WHEN** issuance requests a bounded number of decoy digests at valid object locations
- **THEN** the decoys are indistinguishable in shape from disclosure digests while never resolving to retained claim material

### Requirement: disclosure graphs reject ambiguity and unused material

Disclosure processing SHALL validate each encoded disclosure, compute its
digest under the issuer-declared supported algorithm, and resolve one finite
claim graph. Duplicate encodings, duplicate digest references, digest
collisions, ambiguous placements, cyclic or over-depth graphs, reserved-claim
misuse, and presented disclosures unused by the resolved graph SHALL fail with
a stable redacted category.

#### Scenario: a presentation includes an unused disclosure

- **WHEN** a disclosure digest is not reachable exactly once from the issuer payload or another reachable disclosure
- **THEN** processing fails without exposing the disclosure, salt, claim name, claim value, or digest through diagnostics

#### Scenario: two disclosures resolve the same digest

- **WHEN** distinct presented disclosure bytes collide under the selected digest or one digest is referenced ambiguously
- **THEN** processing fails closed rather than selecting one value by order

### Requirement: key binding is exact, bounded, and policy-neutral

The SDK SHALL construct and verify the RFC 9901 key-binding JWT using exact
`sd_hash`, protected type, audience, nonce, and issued-at inputs. The engine
SHALL verify the holder signature through caller-supplied JOSE capabilities
and SHALL compare caller-supplied expectations without selecting holder keys,
trust, replay, clock-skew, or authorization policy.

#### Scenario: holder key binding matches expected inputs

- **WHEN** the key-binding JWT signature, type, `sd_hash`, audience, nonce, and issued-at expectation all pass
- **THEN** the verified state records only redaction-safe evidence classes and exposes no audience, nonce, signature, key, or compact credential through its receipt or diagnostics

#### Scenario: key-binding inputs do not match

- **WHEN** any key-binding type, hash, audience, nonce, issued-at, algorithm, key, or signature expectation fails
- **THEN** verification returns a stable correlation-safe failure without indicating the secret comparison value

### Requirement: every untrusted path has finite public resource limits

The SDK SHALL apply public limits to compact bytes, issuer and key-binding JWS
bytes, disclosure bytes and count, salts, decoys, JSON bytes, members, nodes,
depth, selected claims, retained output, and aggregate work. The narrowest
cheap limit SHALL be checked before proportional decode, allocation, hashing,
graph traversal, serialization, or signature work.

#### Scenario: encoded input exceeds a byte boundary

- **WHEN** an encoded compact or disclosure segment exceeds its configured limit
- **THEN** the engine rejects it before base64 decoding or JSON allocation

#### Scenario: valid syntax exceeds aggregate work

- **WHEN** otherwise valid disclosures or nested claims would exceed the configured node, depth, retained-output, or work budget
- **THEN** the engine returns a resource-limit category before partial verified output becomes observable

### Requirement: errors and diagnostics do not disclose correlating material

Public errors SHALL be a non-exhaustive static taxonomy with stable SDK error
codes. `Debug`, `Display`, error sources, serialization, and verification
receipts SHALL NOT contain compact credentials, disclosures, salts, claim
names or values, digests, keys, signatures, audience, nonce, or issuer and
holder identifiers. Bounded counts, stage, algorithm class, and limit kind MAY
be exposed only where they cannot reconstruct or correlate input.

#### Scenario: malformed private disclosure fails

- **WHEN** a disclosure contains malformed or forbidden private material
- **THEN** every returned and formatted diagnostic is independent of the disclosure bytes and values

### Requirement: offline issue-to-verification journey uses public SDK seams

The SDK SHALL provide executable offline evidence that issues through injected
entropy, digest, time, and signing capabilities; stores the result in a
bounded generic credential envelope; selects claims through the presentation
domain; constructs holder key binding; verifies issuer and holder evidence;
and returns a redacted receipt without network, persistence implementation,
trust policy, or product runtime.

#### Scenario: complete offline journey succeeds

- **WHEN** deterministic test effects and valid issuer and holder public keys drive the public APIs from claims through verification
- **THEN** the final receipt proves issuer signature, disclosure processing, selection, and holder binding while the envelope and presentation surfaces remain format-neutral

### Requirement: conformance and quality evidence are provenance-bound

The implementation SHALL carry immutable RFC 9901 provenance, clean-room
negative and exact-boundary vectors, property tests, bounded fuzz targets,
normal and adversarial benchmarks, and differential evidence from at least two
independent exact-version oracles. Disagreements SHALL be resolved against RFC
9901 rather than majority behavior. Compile-only target evidence SHALL remain
distinct from runtime, packaging, FFI, device, and certification claims.

#### Scenario: an oracle disagrees with the RFC contract

- **WHEN** an immutable oracle accepts input that the pinned RFC or SDK resource contract rejects
- **THEN** the disagreement is recorded as oracle divergence and does not weaken the SDK requirement

#### Scenario: a portable target only compiles

- **WHEN** the crate compiles for WASM, iOS, or Android without executing a target runtime journey
- **THEN** evidence reports compile-only support and retains the runtime-support limitation
