# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation has two active consumers for generic RFC 3986
mechanics. `identus-did::Uri` owns a bounded absolute-URI parser including
schemes, authority/userinfo/host/port, paths, queries, fragments, percent
escapes, IPv6 and IPvFuture. `identus-oid4vci` calls `uriparse 0.6.4` at four
absolute/reference seams and wraps it with HTTPS, host, userinfo, query,
fragment, visible-character, size and redaction policy. Because `uriparse`
rejects IPvFuture, OID4VCI replaces that host with `[::1]` in a zeroizing
temporary string and reparses the mutated value.

`identus-core::Url` is a separate inherited hierarchical value with no users
outside its own tests. Changing it would add a foundation dependency and alter
an under-specified acceptance boundary without current consumer payoff, so it
is excluded.

## Normative sources

- [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986.html) defines URI,
  URI-reference, scheme, authority, IPvFuture and component grammar.
- [`fluent-uri 0.4.1` documentation](https://docs.rs/fluent-uri/0.4.1/fluent_uri/)
  defines distinct borrowed `Uri` and `UriRef` parsers and its feature surface.
- [Exact release source](https://github.com/yescallop/fluent-uri-rs/tree/d9a6a20614f34b00476837eb8904fb01ca3e54df)
  is the VCS revision recorded by the published artifact.
- Canonical `did-core`, OID4VCI Credential Offer, Token Error Response and
  Token Response specifications define SDK resource and protocol policy.

Protocol or draft currency is stable for this seam: RFC 3986 is the intended
generic grammar and OID4VCI/OAuth layers retain their current final-profile
policy. IRI parsing remains explicitly out of scope.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `fluent-uri` | 0.4.1 / `d9a6a206` | `adopt` | Strict URI/URI-reference grammar, IPvFuture, borrowed parsing, low coupling, two active SDK consumer crates and meaningful local-code/runtime-dependency removal. | Reconsider on parity failure, target failure, advisory, unbounded regression or unsafe-cone rejection. |
| `uriparse` | 0.6.4 | `retain-local` as dev oracle only | Existing NeoPRISM-aligned differential evidence is useful, but runtime IPvFuture mismatch requires a workaround and it has a historical panic on malformed scheme input. | Remove the dev oracle only when independent RFC vectors and fuzz evidence fully replace its useful comparison value. |
| Local DID generic URI parser | `develop@1d627801` | `not-adopt` as retained production mechanics | Correct and bounded, but duplicates a closed standard and costs roughly 180 lines plus maintenance after a compatible engine is available. | Restore only if the candidate fails conformance, targets, supply-chain review or sustainable maintenance. |
| `identus-core::Url` migration | current local value | `conditional-adopt` | No external consumer and a distinct hierarchical contract make migration payoff unproved. | A named consumer requires full RFC URL grammar and a separate issue specifies its bounds and compatibility. |

## Compatibility and dependency evidence

The exact version and features proposed are `fluent-uri 0.4.1` with default
features disabled and no optional features for borrowed parsing. It declares
MSRV Rust 1.68, below the effective Rust 1.98.1 etalon. The preliminary
standalone direct and resolved dependency cone counted eight normal/build
packages: `fluent-uri`, `borrow-or-share`, `ref-cast`, `ref-cast-impl`,
`proc-macro2`, `quote`, `syn` and `unicode-ident`. Integration must record the
exact locked graph and incremental removal of runtime `uriparse`, `fnv` and
`lazy_static` from OID4VCI.

Target evidence before integration is limited to the parent portfolio probes,
which compiled the candidate on host, WASM, iOS and Android. Those probes are
dated inputs, not acceptance. This change must rerun the repository Rust 1.98,
no-default-feature, WASM, Android, iOS and Nix gates against the integrated
lockfile.

Public and wire compatibility must be exact: Identus-owned strings, serde,
errors and exact spelling remain unchanged. The facade boundary discards
candidate views and errors immediately after validation. Rollback is a single
PR revert restoring the local parser and `uriparse`; no stored value migration
exists.

## Security, privacy and maintenance evidence

License and provenance: `fluent-uri 0.4.1` is MIT; the crates.io checksum is
`bc74ac4d8359ae70623506d512209619e5cf8f347124910440dbc221714b328e`.
The package records revision
`d9a6a20614f34b00476837eb8904fb01ca3e54df`, an unsigned lightweight release
commit. Packaged `src/lib.rs` SHA-256
`91463a1f23b0d1a7a1d56e6d643c4a6e7b7194c6b8244ea500ec48d054675cad`
matches that revision. The current crates.io release remains 0.4.1 on the
retrieval date; the repository has later commits, so floating source is not
used.

Unsafe and native-code evidence: the candidate and `borrow-or-share` forbid
unsafe code and no native code or FFI is expected. `ref-cast` and generated
derive implementations contain unsafe reference casts. Integration must
attribute exact reachable sites under immutable borrowed parsing and run the
repository dependency/security tools; the upstream `forbid` declaration alone
is not a transitive safety proof.

Supply-chain evidence currently includes exact artifact/source comparison,
release metadata and manual source inspection. A fresh lockfile-based advisory
and license pass is an implementation gate. Maintenance, release and security
posture is active and narrow, but the release commit/tag is unsigned; checksum
pinning and repository review remain required.

## Rejected or deferred candidates

WHATWG `url` remains rejected for this generic seam because it normalizes and
applies URL semantics rather than preserving exact RFC URI spelling.
`did_url_parser` is a separate DID/DID-URL spike under #159 and cannot replace
generic RFC URI references. Moving the parser into `identus-core` or changing
`identus-core::Url` is deferred for lack of a named consumer and because it
would broaden the foundation decision. IRI features, normalization and
resolution remain disabled.

## Open questions and blockers

No research blocker prevents the bounded implementation. Implementation must
still prove: exact acceptance parity for all pinned DID cases; unchanged
OID4VCI HTTPS/URI-reference policy; whether no optional candidate feature is
sufficient; exact resolved cone/advisories/licenses; and all supported target
builds. Any unexplained widening, negative-input weakening, public type leak,
target failure or unapproved unsafe reachability is a stop condition, not a
follow-up.

## Evidence commands

Exact commands and unrun checks are separated:

- `rg` and `cargo tree` located every current parser call, consumer and normal
  `uriparse` edge;
- `cargo search fluent-uri --limit 3` and `cargo info fluent-uri@0.4.1`
  confirmed current release metadata and features on 2026-09-08;
- GitHub API calls resolved the `v0.4.1` lightweight tag, exact commit,
  unsigned verification state, later source drift and release chronology;
- `shasum -a 256` compared the cached crates.io archive/package source with
  the exact release source; direct unsafe searches inspected the candidate and
  transitive packages;
- Cargo integration, differential tests, `cargo deny`, supported targets,
  factory and Nix checks are unrun implementation tasks and are not represented
  as passed by this research record.
