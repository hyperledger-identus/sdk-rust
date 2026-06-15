# ADR: Plugin And Extension Compatibility

Status: Accepted

Date: 2026-06-13

## Context

The TypeScript SDK exposes a plugin mechanism through `Plugin`, `PluginManager`,
and `Plugins.Task`. Source evidence from `repos/sdk-ts` shows two core
extension shapes:

- Module extension: plugins add named modules to a task context with
  `addModule(key, module)`.
- Protocol task dispatch: plugins register task constructors by protocol id or
  by composite `type/id` lookup.

Internal TypeScript plugins use that mechanism for DIDComm, AnonCreds, DIF,
OIDC, and OEA compatibility packages. The current registry is dynamic and uses
`Map<string, any>` for modules plus task constructors for handlers. That is
ergonomic for TypeScript, but it cannot be the core extension model for
`sdk-rust` because it erases ownership, error types, capability permissions,
secret handling, and conformance coverage.

The Rust SDK still needs compatibility with the existing TypeScript extension
surface while it becomes the semantic core for TypeScript, React, React Native,
Swift, Kotlin, browser, mobile, server, Cloud-Service, Mediator, and NeoPRISM
composition.

## Decision

`identus-bindings` owns the cross-language extension contract. Protocol and
credential semantics remain in the relevant Rust crates:

| Extension family | Rust owner |
|---|---|
| DIDComm message, mediation, pickup, and problem-report handlers | `identus-messaging` |
| AnonCreds credential and presentation handlers | `identus-credentials`, `identus-presentations`, `identus-trust` |
| Presentation Exchange and presentation verification handlers | `identus-presentations`, `identus-credentials`, `identus-trust` |
| OpenID Connect and OpenID4VC helpers | `identus-openid4vc`, `identus-trust`, `identus-adapters` |
| Legacy compatibility handlers | Owner crate for the underlying SSI capability, exposed through `identus-bindings` |

The stable Rust boundary is an extension registry with typed descriptors:

- `ExtensionRegistry`: installs modules and task descriptors.
- `ExtensionModuleDescriptor`: names a module, owning crate, version,
  capabilities, required permissions, and target availability.
- `ProtocolTaskDescriptor`: maps protocol ids, message types, credential
  formats, and task kinds to an owner crate and handler entry point.
- `ExtensionContext`: gives tasks typed access to agent, wallet, storage,
  resolver, transport, clock, entropy, and HTTP adapter ports.
- `ExtensionResult`: returns typed success payloads or redaction-safe typed
  errors.

The registry may expose TypeScript-compatible names through generated wrappers,
but public Rust APIs must use SSI domain terminology. Historical source names
are allowed only in migration and source-evidence text.

## Compatibility Rules

- Existing TypeScript `PluginManager.register(plugin)`, `getModules()`, and
  `findProtocol(type, id)` behavior maps to generated wrapper facades over the
  typed registry.
- Dynamic module values in wrappers must cross into Rust as typed module
  descriptors or opaque host handles, never as untyped core state.
- Protocol dispatch keys must normalize to typed identifiers:
  DIDComm message type, credential format, presentation format, OpenID4VC flow,
  trust/status mechanism, or explicit extension task kind.
- Wrapper-hosted custom code may orchestrate calls, but it must not implement
  credential verification, DID resolution, DIDComm state transitions,
  OpenID4VC state transitions, trust policy, status checking, or secure storage
  semantics.
- Extension installation must declare capabilities and permissions before use.
- Extensions must not receive raw private keys, bearer tokens, unredacted
  credential claims, raw secure-store records, or transport credentials unless
  the owning adapter explicitly grants an opaque handle.
- Errors, logs, task events, and result DTOs must be redaction safe.

## Source Evidence Mapping

| TypeScript source | Rust compatibility mapping |
|---|---|
| `Plugin.addModule(key, module)` | `ExtensionModuleDescriptor` plus optional opaque host handle |
| `Plugin.register(pids, task)` | `ProtocolTaskDescriptor` with typed protocol ids and task kind |
| `PluginManager.getModules()` | Generated wrapper facade returning allowed module handles |
| `PluginManager.findProtocol(type, id)` | Registry lookup by typed protocol id or composite task key |
| `Plugins.Task` | `ExtensionTask` facade around Rust state-machine or adapter ports |
| Internal DIDComm plugin | `identus-messaging` protocol inventory and transcript fixtures |
| Internal AnonCreds plugin | `identus-credentials`, `identus-presentations`, `identus-trust` vector and interop fixtures |
| Internal DIF plugin | `identus-presentations` Presentation Exchange and DCQL fixtures |
| Internal OIDC plugin | `identus-openid4vc` transcript fixtures and HTTP adapter ports |
| Internal OEA plugin | Compatibility aliases that resolve to current SSI owner crates |

## Conformance Requirements

The extension compatibility layer must have:

1. A generated inventory of TypeScript plugin exports and task registrations.
2. Static conformance tests proving every task registration has an owner crate,
   capability id, fixture coverage, and typed error family.
3. Transcript replay for DIDComm and OpenID4VC protocol tasks without Docker by
   default.
4. Vector replay for credential, presentation, status, and trust tasks without
   infrastructure by default.
5. Wrapper tests proving TypeScript compatibility names dispatch through the
   Rust registry rather than duplicated wrapper logic.
6. No-secret-leak tests for module descriptors, task context, logs, events,
   result DTOs, and errors.

## Consequences

- TypeScript plugin users get a migration path without freezing Rust around
  untyped TypeScript internals.
- Protocol and verification behavior remains portable to Swift, Kotlin, React
  Native, server, Mediator, Cloud-Service, and NeoPRISM composition.
- Extension points become auditable: each installed capability has ownership,
  permissions, source evidence, fixtures, and typed errors.
- Wrapper packages can add developer ergonomics, but parity claims still depend
  on Rust conformance fixtures and the wrapper API parity gate.
