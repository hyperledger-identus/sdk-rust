# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation has no `did:web` method owner.

`identus-did` already provides bounded duplicate-safe DID document parsing,
exact DID values, resolution result types, and an executor-neutral boxed-future
port convention. `identus-did-resolver-http` is an inbound resolver server and
is intentionally not a client transport. The delivered method crates prove
that method ownership belongs in independently selectable packages.

## Normative sources

| Source | Immutable revision | Role |
| --- | --- | --- |
| `w3c-ccg/did-method-web` | `ea423c114e6f2537498ee6f94e8d794c64f60c18` | method syntax, HTTPS mapping, resolution, and exact ID check |
| W3C DID Core 1.0 | Recommendation 2022-07-19 | DID document JSON, resolution, media types, and ID binding |
| RFC 1035 / 1123 / 2181 | published RFC revisions | ASCII DNS name constraints |
| RFC 3986 | published RFC revision | URI syntax and percent-encoding vocabulary |

The pinned method draft maps a bare domain to
`https://<domain>/.well-known/did.json`, maps a path DID to
`https://<domain>/<segments>/did.json`, requires an encoded port colon, and
requires the resolved document ID to match the requested DID. The first SDK
profile deliberately narrows ambiguous draft syntax rather than silently
accepting unsafe URL forms.

## Build-versus-adopt evidence

Spruce `did-web 0.5.1` is maintained and useful as an oracle, but its production
dependencies include Reqwest and the broader SSI DID stack. That makes its HTTP
runtime and transport policy part of the method package and is incompatible
with the SDK's injected-effect and target boundaries. The Impierce stack is a
second behavior oracle but carries product/runtime coupling. Neither is adopted
as a production dependency. No source is copied wholesale.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| SDK-owned `identus-did-web` core | `adopt` | Exact narrow profile, existing bounded DID parser, explicit effect boundary, and no new runtime dependency. | A maintained narrow crate satisfies every profile, port, bound, target, and error contract. |
| Existing `identus-did` parser | `adopt` | Already enforces byte, duplicate-name, structural, and semantic document limits. | The method needs a representation generic DID cannot consume. |
| Caller-injected boxed-future fetch port | `adopt` | Matches repository conventions and lets each target own runtime/security policy. | A shared SDK transport abstraction is separately approved and proves lower coupling. |
| Spruce `did-web 0.5.1` | `oracle` | Maintained, but embeds Reqwest and a broader SSI dependency cone. | A feature-minimal release removes runtime ownership and matches this strict profile. |
| Impierce DID stack | `oracle` | Independent evidence with disproportionate product/runtime coupling. | A standalone portable method package is qualified. |
| Concrete Reqwest adapter | `not-adopt` | Selects HTTP, TLS, executor, redirect, and target policy inside the core. | A later named consumer owns an adapter issue. |

## Selected first profile

- Lowercase ASCII FQDN labels using letters, digits, and interior hyphens;
  at least two labels; no trailing dot, underscore, IP literal, user info,
  query, or fragment.
- Optional uppercase canonical `%3A` followed by a decimal port in `1..=65535`;
  no leading zero and no explicit port `443` because it is noncanonical.
- Optional non-empty colon-delimited path segments containing the conservative
  generic DID `idchar` ASCII subset; dot segments, percent escapes, and `~` are rejected.
- Exact HTTPS URL derivation with no normalization delegated to a URL library.
- Successful response status `200`, bounded media type, and JSON-compatible
  `application/did+json`, `application/did+ld+json`, or `application/json`.
- Bounded body parsed by `DidDocument::from_json_slice`; its root `id` must be
  byte-for-byte equal as a validated DID to the requested identifier.

The strict domain/path decisions are SDK profile constraints, not claims that
all identifiers accepted by every historical implementation are invalid.

## Compatibility and dependency evidence

The additive crate depends inward only on `identus-core` and `identus-did` plus
standard-library futures. Generic DID and sibling methods never depend on it.
No external HTTP, URL, DNS, TLS, SSI, or oracle type crosses the public API.
Compatibility is additive and unreleased. Direct and locked dependency cones
must prove those absences. Target evidence covers host, Rust 1.89 MSRV,
no-default, WASM, iOS, Android, rustdoc, strict Clippy, workspace and Nix gates.
The exact assessed version and feature surface is Spruce `did-web 0.5.1` with
its published defaults and Reqwest production dependency. License and
provenance are recorded separately. Supply-chain evidence must inspect the
resolved dependency cone, RustSec state, unsafe code, and native-code/build
scripts; the selected core intends to add none of those categories. Public and
wire compatibility remain additive because no existing facade changes.

## Effect and security boundary

The method core prepares an inert request containing only the exact HTTPS URL
and accepted media types. A caller adapter performs retrieval and returns a
bounded response value. The adapter owns name resolution, private-network and
SSRF decisions, TLS roots and versions, redirects, proxy behavior, connection
and read limits, cancellation, cache, retry, privacy, and telemetry. The core
does not follow a returned location or accept an effective URL.

## Security, privacy and maintenance evidence

Inputs have explicit identifier, authority, path-count, segment, URL,
media-type, and response-body ceilings. Authority validation occurs before URL
allocation or effect invocation. Status and media type are checked before JSON
parsing. The fetch port is invoked at most once and failures are fixed
categories. DNS, private-network ranges, TLS, redirects, proxies, timeouts,
cancellation, cache, retries, telemetry and privacy remain adapter concerns.
Maintenance belongs to SDK-Rust DID maintainers; source drift triggers a new
decision rather than silent widening.

## Evidence plan

Conformance fixtures cover the pinned mapping examples and strict negative
profile. Properties cover parse/format/URL stability. Fuzzing targets method
parsing and response binding. Benchmarks measure derivation, successful binding,
and early rejection. The deterministic E2E supplies an in-memory fetcher that
asserts the exact public request, returns a DID document, and proves URL
mapping, port invocation, bounded parsing, ID correlation, and redacted errors
through the production API.

## Rejected or deferred candidates

Concrete HTTP runtimes, broad SSI facades, permissive URL parsing, redirects,
registration operations, persistence, bindings, consumer migration, and
live-network core tests are rejected or deferred under ADR 0190.

## Open questions and blockers

No implementation blocker remains. The pinned draft permits broader host/path
interpretation than this first profile. Contrary committed-consumer evidence
stops the affected behavior for a new profile decision.

## Evidence commands

Planning gates are `scripts/factory research-ready add-portable-did-web-core`,
`scripts/factory constraints-ready add-portable-did-web-core`, strict validation,
and exact #559 preflight. Implementation requires focused/workspace tests,
deterministic fetch-port E2E, fuzzing, benchmarks, target/dependency receipts,
exact-diff Claude or Codex review, protected CI, and terminal metrics.
All implementation commands and checks are unrun at planning time and cannot
be claimed by the preflight receipt. Rollback and release posture remain bound
to the additive unreleased package and its evidence.
