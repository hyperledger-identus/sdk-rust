# SDK-TS DID resolution evidence contract

Issue [#599](https://github.com/hyperledger-identus/sdk-rust/issues/599) owns
the private browser, Node.js, and React web canary for DID resolution and DID
URL dereferencing. The canonical portable contract is Rust-owned; TypeScript
injects one asynchronous host effect that returns raw JSON text and performs
no automatic retry, parser fallback, method selection, or ambient network
access.

## Immutable inputs

- SDK-Rust implementation base: `9b4e463bf37881fd282b5d405fd9272524f34b99`.
- SDK-TS donor: `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
  (`8.1.4`). Every donor source and test has an exact digest and disposition in
  `docs/conformance/sdk-ts-did-resolution-donor.toml`.
- Normative source: W3C DID Resolution Candidate Recommendation Draft,
  2026-08-28.
- Portable packet: `did.resolution.v1`. The catalog digest is authoritative;
  fixture input is public clean-room data.

## Closed portable boundary

Rust validates structured resolution or dereferencing options, prepares the
request, and parses a bounded, duplicate-safe result. Resolution parsing is
correlated with the requested DID. The host owns DNS, TLS, redirects, SSRF,
authentication, retry, timeout, cancellation, response streaming, and the
preallocation required to produce the raw text. Native non-JSON dereferencing
bytes are excluded rather than guessed as text or base64.

The TypeScript facade owns target-neutral orchestration, the outer raw-text
length check, deterministic projection and deep freezing, generated-resource
disposal, redacted stable errors, and non-sensitive observations. It does not
restore the donor's legacy Castor, Mercury, or Pluto names.

## Evidence state

The packet, donor ledger, adapter mapping, compatibility record, resource
boundary, TS1 row, and quality declaration are canonical registration. The
four required quality obligations are source-bound to their reviewed
implementation revisions:

- property and mutation coverage:
  `packages/identus-sdk/tests/provider.test.mjs` at
  `5aff2744b7637471c7b9d8a86aa2498185a8306c`;
- bounded sanitizer fuzzing: `fuzz/fuzz_targets/did_resolution.rs` at
  `f1b9a458adcee5fe0fd354ce649d343ed8a392a5`;
- deterministic Node measurement harness with web/Node artifact accounting:
  `packages/identus-sdk/tests/measurements.mjs` at
  `808f8922110af12e6f8815dcbc8886b426922ed6`;
- cross-target differential assertions:
  `packages/identus-sdk/tests/resolution-vector-assertions.mjs` at
  `808f8922110af12e6f8815dcbc8886b426922ed6`.

The focused provider, OpenSpec, factory-structure, support-policy, quality
registry, and mutation commands passed locally against the final remediation commit
on 2026-11-17. Differential, fuzz, measurement, isolated-target, and full Nix
execution remain explicit candidate-closeout gates rather than being inferred
from source-bound evidence. Source-bound evidence is not a claim of external
consumer rehearsal, publication, support, production readiness, or React
Native applicability.

## Security and resource non-claims

The canary does not provide a transport, a resolver registry, DID method
semantics, trust policy, cache backend, persistence, credentials, DIDComm,
agent runtime, React Native/native bindings, publication, certification, or
production support. Correlating inputs, raw result text, DID values, document
content, problem details, host errors, stacks, and WASM memory are never
observability fields.
