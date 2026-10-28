# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/546
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep effects at adapters. SDK-SEC-003 and
SDK-LIM-007 require finite hostile-input work. ADRs 0168, 0179, 0180, 0181,
0183, and 0184 control the existing request, response, security, and verifier
authorization lineage. The Final matrix redirect-response row is affected.

## Introduced or changed constraints

- Every response constructor consumes verifier-authorized request lineage.
- Redirect modes use only the exact signed bounded `redirect_uri`; direct-post
  modes use only the exact signed bounded `response_uri`; mixed destinations
  fail closed.
- `fragment`, `query`, and `form_post` share the existing DCQL cardinality,
  deterministic response parameter, exact state, and redaction rules.
- Existing destination query or fragment data is preserved only when it cannot
  collide with generated response parameters; ambiguity fails closed.
- Direct-post acknowledgement validation consumes sent response material and
  cannot be detached from its exact target/body lineage.
- An optional redirect is bounded syntax and opaque response-code evidence, not
  navigation authority, entropy proof, trust, or successful redemption.
- No new dependency, unsafe code, runtime, transport, browser, persistence,
  product policy, or global state enters the SDK.

## Introduced or changed limitations

The SDK constructs inert redirect/form material but does not navigate, render
HTML, execute HTTP, prevent URL leakage, or decide whether a front-channel
mode is acceptable for a product. It validates acknowledgement syntax and
correlation only; it cannot prove verifier processing, randomness, session
fixation resistance, or response-code redemption. Portable evidence is
compile-only unless a target runtime test is explicitly recorded.

## Consumer and product impact

Consumers must make exhaustive response-mode matches forward-compatible,
provide scheme-complete verifier trust policy for the exact redirect URI, and
apply transport, browser, privacy, navigation, and session policy. Rust-native
types remain canonical; foreign-language migration belongs in thin adapters.

## Activation and rollback

Activation requires planning readiness, issue preflight, clean-room vectors,
portable/local/Nix evidence, independent CLI review, signed+DCO commits,
protected CI, and guarded merge. Before publication, rollback removes the new
variants and acknowledgement successor while retaining current direct-post
behavior and its verifier-authorization gate.

## Evidence

Issue #546 and parent #541 direct the work. Research and normative decisions
are in `research.md`; successful implementation evidence is recorded only at
closeout.
