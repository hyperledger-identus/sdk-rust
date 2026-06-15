# Identus SDK Rust Constitution

## Core Principles

### I. Rust Core Owns Product Semantics
The Rust workspace is the canonical implementation for Identus SDK behavior. TypeScript, Kotlin, Swift, React, and React Native packages are bindings, adapters, or developer-experience layers over the Rust core unless a platform capability is impossible to expose safely from Rust. Feature parity with `sdk-ts`, `sdk-swift`, and `sdk-kmp` is the entry bar, not the target state.

### II. Hexagonal Architecture Is Mandatory
Every capability is modeled as domain crates plus explicit ports and adapters. Domain crates must not depend on network, storage, platform UI, or binding code. Adapters may target HTTP, DIDComm transports, secure storage, SQLite, mobile key stores, WASM, UniFFI, N-API, or service runtimes, but must implement stable ports with contract tests.

### III. Standards-First Interoperability
Implementation choices must trace to public specifications or Identus protocol decisions. Required families include W3C DID Core, W3C VC/VP, PRISM DID, DIDComm v2, AnonCreds, DIF Presentation Exchange, OID4VCI, OID4VP, SIOPv2, SD-JWT VC, ISO mdoc/mDL, status/revocation lists, and OpenBadges 3.0. Any proprietary shortcut needs an issue and an explicit migration path.

### IV. Cross-Platform Bindings Without Forked Logic
The repository must support server, CLI, WASM/web, Node.js, mobile, and native app consumers from shared crates. Binding layers must be thin, documented, generated where practical, and tested against the same fixtures. A bug fixed in Rust must not require reimplementation in every language wrapper.

### V. Security, Privacy, and Conformance Gates
Cryptography, key management, wallet storage, credential verification, and protocol flows require negative tests, fixture-based conformance tests, and threat-model notes before implementation is considered complete. Verification must check temporal validity, issuer binding, key purpose, credential status, disclosure integrity, and audience/domain/challenge constraints where applicable.

### VI. Neoprism-Grade Quality
The SDK follows or improves on `neoprism` quality attributes: Cargo workspace hygiene, feature-gated crates, reproducible CI, file hygiene, scorecard/dependency review, coverage, structured tracing, OpenAPI/type generation where useful, Nix/devcontainer friendliness, and clear release automation.

### VII. SSI Domain Naming Is Mandatory
Public Rust crates, modules, traits, DTOs, APIs, documentation headings, and backlog items must use SSI domain terminology, not legacy Identus codenames. Prefer names such as cryptography, DID, DID resolver, credential, presentation, DIDComm, mediation, wallet, agent, trust, OpenID4VC, storage, adapter, and binding. Historical names from existing SDKs may appear only when citing legacy source evidence in migration maps, compatibility notes, or tests; they must not become new `sdk-rust` product names or abstractions.

### VIII. Type-Safe SSI Primitives Come First
Core SSI concepts must be represented by validated domain types before they cross crate, adapter, binding, or storage boundaries. DIDs, DID URLs, verification method IDs, key identifiers, credential identifiers, presentation definitions, status references, protocol thread IDs, and wallet record IDs must not be passed around as unchecked strings except at parsing or serialization edges. Parsers must return typed errors, preserve canonical string forms, reject malformed inputs early, and avoid panics, `unsafe`, implicit lossy conversion, or ambiguity between identifier kinds.

### IX. Every Increment Has Acceptance Criteria
Every implementation increment must be represented by at least one Spec Kit task before merge. Each such task must include concrete acceptance criteria that name the behavior or specification evidence delivered, the validation command or executable check that proves it, and any fixture, conformance, migration, or compatibility impact. Work discovered during an increment must be added to the backlog in the same increment instead of remaining only in chat or local notes.

## Required Engineering Constraints

- Public APIs must be versioned and documented with migration notes for all breaking changes.
- Public names must be reviewed for SSI-domain clarity before merge; legacy codenames are allowed only as historical source references and must be covered by the conformance naming audit.
- SSI primitives must expose narrow constructors and typed accessors; unchecked string constructors are allowed only in tests or explicitly named unsafe/test helpers.
- Crates must keep dependency surfaces small and use features for optional protocols or heavy targets.
- `no_std` compatibility should be considered for crypto/domain crates when feasible, but never at the expense of correctness.
- Storage adapters must support encrypted-at-rest mobile/server use cases and deterministic test stores.
- Protocol adapters must expose observability hooks without leaking secrets, claims, tokens, or private keys.
- FFI and WASM boundaries must use stable DTOs and error models; raw domain internals do not cross binding boundaries.

## Development Workflow

- New work starts with a Spec Kit specification under `specs/`.
- Plans must declare the owning crates, ports, adapters, fixtures, conformance sources, and migration impact on existing SDKs.
- Tasks must preserve independently testable user stories and avoid shared mutable implementation work that blocks parallel execution.
- Every code-bearing increment must add or update a task with explicit acceptance criteria before the code is committed.
- CI must run formatting, linting, unit tests, integration tests, generated artifact checks, file hygiene, dependency review, and security scans before release.
- PRs that affect code or dependencies require security review and DCO/GPG-signed commits according to workspace policy.

## Governance

This constitution supersedes ad hoc implementation preferences for `sdk-rust`. Amendments require a pull request that explains the reason, the migration impact, and any dependent template/spec updates. Reviews must verify compliance with the constitution before approving implementation work. Version changes follow semantic versioning: major for principle removals or incompatible governance changes, minor for new principles or material expansions, and patch for clarifications.

**Version**: 1.3.1 | **Ratified**: 2026-06-13 | **Last Amended**: 2026-06-13
