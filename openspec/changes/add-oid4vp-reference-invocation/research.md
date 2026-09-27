# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The current implementation has bounded generic presentation semantics but no
OID4VP wire crate.
ADR 0156 retains Identus-owned OID4VP public types and permits no production
framework or DCQL dependency yet. Oxid revision
`183664aeca500c25d6d27a22fa402b4d40c649d3` has a product-specific parser for
`openid4vp://authorize?client_id=...&request_uri=...`; it couples localhost
demo policy, session orchestration, Midnight formats, consent, storage and
proof handling and is therefore an oracle rather than extraction source.

## Normative sources

- OpenID4VP 1.0 Final sections 5, 5.4, 5.7 through 5.10 and 14; the current
  errata-incorporating publication was retrieved from
  `https://openid.net/specs/openid-4-verifiable-presentations-1_0.html`.
- RFC 9101 defines Request Objects by reference and their retrieval boundary.
- RFC 9700 supplies the referenced OAuth security guidance.
- ADR 0156 fixes the crate ownership and third-party reuse boundary.

The Final specification defines three transports: encoded request parameters,
a Request Object by value, and a Request Object by reference. This slice
supports only the reference discriminator. The parser does not claim JAR
validation merely because a URI is syntactically safe.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Identus-owned narrow parser | `build` | Small bounded transport discriminator; owns exact public/redaction policy. | A focused published crate proves smaller bounded parity. |
| Reuse private `identus-oid4vci` form/URI modules | `not-adopt` | Private error-coupled helpers would create sideways protocol coupling. | A separately justified generic wire utility has two consumers. |
| `fluent-uri 0.4.1` | `adopt-existing` | Already reviewed, no-default-feature workspace dependency for URI structure. | Dependency policy or parser behavior changes. |
| Oxid adapter | `oracle` | Proves consumer shape but carries product, HTTP-loopback and Midnight policy. | Never copied as the generic core. |
| Full OID4VC frameworks | `oracle` | ADR 0156 found unacceptable coupling/cones for the SDK boundary. | Narrow released core passes all SDK gates. |
| `siros-dcql 0.3.0` | `defer` | This slice does not parse or execute DCQL. | Separate facade/resource issue with a named consumer. |

## Compatibility and dependency evidence

The new `identus-oid4vp` package is additive, version `0.0.0`, unpublished,
and outside release trains. It depends only on `identus-core`, `fluent-uri`,
and `zeroize`, all existing workspace families. It exports no third-party type
and creates no upward, chain, product, runtime, network, native or unsafe edge.

The exact dependency versions and features remain workspace-selected:
`fluent-uri =0.4.1` with default features disabled, `zeroize 1.9` with
`alloc,derive`, and path-local `identus-core`. The direct dependency cone is
these three families; the resolved dependency cone is measured from the locked
Cargo graph during implementation. Rust 1.89.0 is the effective MSRV and Rust
1.98.1 is primary/etalon. Target evidence will cover host tests plus compile-
only WASM, iOS and Android checks without converting them into runtime claims.

Public and wire compatibility is additive and pre-release. The facade boundary
contains only Identus-owned invocation, reference, method, limit and error
types. Existing packages, serialized forms and consumers remain unchanged.

## Security, privacy and maintenance evidence

The complete invocation, decoded names/values, pair count, client identifier,
and Request URI receive independent limits. Strict percent/form decoding
precedes duplicate detection. NUL, non-UTF-8, duplicate decoded names,
userinfo, fragments, unsafe schemes and unsupported transports fail with
static diagnostics. Retained verifier-controlled values are zeroizing and
redacted from `Debug`, `Display`, errors and component metadata.

Unknown bounded outer parameters are ignored as Final requires, except
`transaction_data`, which fails explicitly because this implementation does
not support it. Ignoring does not permit recognized-name shadowing. Retrieval,
DNS, redirects, decompression, TLS, Request Object signature/type/audience,
verifier trust, nonce/replay and response security remain caller/later owned.

License and provenance are closed: authored code is Apache-2.0; no donor source
or fixture is copied; `fluent-uri`, `zeroize` and their exact lock entries stay
under the repository's existing Cargo-deny policy. Supply-chain evidence is
the locked graph, Cargo-deny/license/advisory checks and root dependency guard.
No new build script, proc macro, native library or unsafe exception is accepted.
The maintenance, release and security posture remains source-only and owned by
SDK maintainers; publication requires a separate release-train decision.

## Consumer and provenance evidence

Oxid is the named first consumer oracle and Lace ID Portal remains a named
future consumer in IDR-024. Oxid was inspected read-only at the exact revision
above; its pre-existing dirty status is recorded in verification and must stay
unchanged. No consumer or donor file, fixture, key, credential or request is
copied. Final-spec examples are re-authored clean-room vectors.

## Rejected or deferred candidates

The generic `url` crate, `oauth2`, Affinidi, Impierce and Spruce are not needed
for a transport discriminator. By-value/inline transports, client-prefix
profiles, signed requests, response modes, DC API and DCQL each need focused
contracts rather than speculative fields in this first public type.

Protocol or draft currency is explicit: implementation binds the immutable
OpenID4VP 1.0 Final text while research records the current errata publication.
Errata that change behavior require a focused compatibility review rather than
silent parser drift.

## Open questions and blockers

None. The roadmap and issue #394 name the crate, consumer outcome, supported
transport and explicit non-scope. HTTPS-only reference syntax is deliberately
stricter than Oxid's localhost demo and does not promise downstream adoption.

Rollback removes the unpublished crate and additive records before any release
or downstream adoption. No wire migration, stored-data conversion, registry
operation or consumer revert is required.

## Evidence commands

Exact planning commands are `scripts/factory doctor`, `scripts/factory
validate add-oid4vp-reference-invocation`, `scripts/factory check`,
`scripts/factory research-ready add-oid4vp-reference-invocation`, and
`scripts/factory constraints-ready add-oid4vp-reference-invocation`. Remaining
implementation commands are focused tests/Clippy/rustdoc, error-contract
canaries, workspace tests, factory checks, dependency/inventory checks and Rust
1.98.1 WASM/iOS/Android compile checks. Unrun checks at planning time are all
Rust implementation gates, hosted CI, runtime browser/mobile tests, network
interoperability, official certification and downstream adoption; they remain
implementation, later-slice, or separate-repository evidence.
