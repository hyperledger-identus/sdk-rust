# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/545
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep dependencies inward and effects at the
edge. SDK-SEC-003 and SDK-LIM-007 require finite hostile-input work. ADRs 0158,
0168, 0181, and 0183 define the current JAR, routing, encryption, and security
state boundaries. The Final matrix rows owned by #545 are affected.

## Introduced or changed constraints

- The full signed client identifier, including prefix, remains exact and is
  never normalized into an audience or response identity.
- Prefix classification uses only the first colon. Known-but-unsupported and
  unknown prefixes fail closed before trust-port invocation.
- The signed-JAR flow rejects `redirect_uri`; it does not weaken ADR 0183 to
  accommodate a prefix whose Final processing requires an unsigned request.
- `origin` is rejected outside a separately specified DC API profile.
- Verifier authorization consumes security evidence and binds the exact key
  that verified the JWS to the exact correlated client identifier.
- Scheme-specific cryptography, resolution, PKI, Federation and issuer trust
  stay behind caller-owned async ports. Port rejection or unavailability fails
  closed with distinct static errors.
- DID syntax reuses `identus-did`; JOSE syntax reuses `identus-jose`. Protocol
  crates do not duplicate these generic components or import donor DTOs.
- Metadata and verifier-info are accepted only from the Final-permitted source,
  under explicit byte/count/string/reference limits. Federation ignores inline
  `client_metadata` for trust decisions.
- `verifier_info` credential references must identify DCQL credential queries.
- Existing encrypted-response key selection remains separate from verifier
  trust and retains its exact algorithm/key restrictions.
- Public diagnostics are static and redacted. No new unsafe code, runtime,
  transport, persistence, global state, root store, or external dependency.

## Introduced or changed limitations

The SDK evidence says that a caller-owned port accepted one exact verifier/key/
request binding. It does not say why, expose a trust score, select product
policy, record consent, validate credentials, or authorize network I/O. A port
adapter must implement the relevant Final scheme completely; accepting inputs
without doing so violates its contract.

`redirect_uri`, `origin`, and extension prefixes are classified but unsupported
in this slice. Metadata is syntax/evidence, not display-safe content. Portable
evidence is compile-only unless a target runtime test is explicitly recorded.

## Consumer and product impact

Consumers supply a trust adapter implementing the selected Final scheme,
including external resolution, root/issuer policy, and exact key binding. They
continue to own transport, persistence, telemetry, consent, and UI. The SDK
exposes Rust-native types; language compatibility belongs in thin adapters.

## Activation and rollback

Activation requires planning readiness, issue preflight, clean-room vectors,
portable/local/Nix gates, independent CLI review, signed+DCO commits, protected
CI and guarded merge. Before publication, rollback removes the verifier state
and typed successors together; it never restores a route that bypasses verifier
authorization.

## Evidence

Issue #545 and parent #541 direct the change. Research sources, dependency, and
protocol decisions are in `research.md`; implementation commands are recorded
only after successful execution and immutable artifacts are linked at closeout.
