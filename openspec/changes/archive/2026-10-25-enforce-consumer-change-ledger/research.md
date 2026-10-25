# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

At base `fb1c7011897ba766f82ea5957feaeb95df179418`, the canonical
`identus-platform-change-ledger.toml` is an intentionally empty schema-v1
vocabulary plus a comment listing expected fields. It has no parser, validator,
renderer, mutation suite, lifecycle enforcement, or factory disposition. That
is honest for the current repository state but cannot govern a migration.

The three predecessor contracts are implemented and locally validated:

- #420 owns catalog `did.syntax.v1` and 22 stable DID/DID URL vector IDs;
- #505 owns four active TypeScript DID value/error mapping IDs rooted in
  canonical `identus-did` behavior; and
- #501 owns active declaration `did.syntax.quality.v1` with explicit property,
  fuzz, benchmark, and differential dispositions.

The current factory validates each registry independently. It does not yet
prove that a consumer-visible change cites compatible records or that a claim
of behavior-neutral work has a durable disposition.

## Normative sources

- ADR 0164 defines the six change classes, release evidence, deprecation
  phases, and legacy-bug policy.
- ADR 0173 fixes A1 ownership and orders #422 after stable #420 and #505 IDs;
  issue #422 additionally requires the #501 declaration reference.
- The canonical `cross-language-compatibility-foundation` and
  `identus-platform-core-migration` specifications require every
  consumer-visible change to be ledgered before merge.
- `docs/architecture/a1-compatibility-contracts.md` requires schema v2,
  offline validation, deterministic rendering, and an honest empty state.
- The three machine registries at the base revision are the only local
  cross-record authorities. Their source, adapter, and quality payloads remain
  independently owned.

The exact repository source is
https://github.com/hyperledger-identus/sdk-rust/tree/fb1c7011897ba766f82ea5957feaeb95df179418.
All assessed repository material is Apache-2.0. No consumer repository was
read or changed for this planning slice.

The current implementation and consumer evidence were assessed at that exact
revision. The exact version and features are ledger schema v1, vector catalog
v1.0.0 with packet `did.syntax.v1`, adapter registry v1.0.0, quality registry
v1.0.0, and the repository-local factory feature set. License and provenance
are Apache-2.0 repository history plus the immutable source records owned by
the three referenced registries.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Closed TOML schema-v2 ledger | `adopt` | Matches the three delivered A1 registries, remains diffable, and supports strict offline validation. | Multiple repositories need to author the same records through a published interchange API. |
| Python standard-library validator and renderer | `adopt` | Reuses the repository factory pattern without adding a runtime or Cargo dependency. | Shared compiled tooling demonstrably reduces maintenance across repositories. |
| Stable ID references into all three A1 registries | `adopt` | Preserves single ownership and makes dangling or capability-mismatched evidence fail closed. | A registry introduces a versioned export boundary that replaces direct local resolution. |
| Explicit compatibility-impact file per qualifying OpenSpec change | `adopt` | Gives the factory a closed yes/no decision without parsing free-form prose or guessing from a diff. | OpenSpec gains an equivalent typed extension with preserved exact semantics. |
| Synthetic fixture outside the canonical ledger | `adopt` | Exercises the full DID graph while keeping governance-only work from masquerading as a migration. | A real consumer migration becomes the first legitimate canonical record. |
| Infer compatibility class from file paths or SemVer | `not-adopt` | Diffs cannot reliably distinguish public, wire, error, persistence, ABI, target, runtime, behavioral, or security impact. | Never without an explicit reviewed record. |
| Duplicate vectors, mappings, quality commands, or freshness conclusions | `not-adopt` | Copies drift and collapse independent ownership; quality declaration existence is not current release evidence. | Never; reference versioned owners instead. |
| Fetch GitHub or consumer repositories during validation | `not-adopt` | Makes required CI nondeterministic and crosses the upstream/downstream isolation boundary. | A separately governed online audit is added outside required offline validation. |
| Treat TypeScript DTO/error shapes as canonical | `not-adopt` | Conflicts with ADRs 0169/0170 and the delivered #505 direction. | A future architecture decision changes canonical ownership. |

## Compatibility and dependency evidence

The implementation is repository metadata and standard-library Python. It adds
no Cargo package, Rust API, production dependency, wire shape, persisted data,
MSRV, target, unsafe code, or runtime behavior. The exact registries remain the
owners of their IDs and payloads.

The direct and resolved dependency cone is unchanged for every Cargo package;
the checker depends only on the pinned Python standard library and local text
files. Public and wire compatibility are unchanged. The facade boundary is the
non-executing factory command plus checked-in TOML/Markdown evidence. Protocol
or draft currency is not applicable to the ledger metadata itself; referenced
DID semantics remain bound to the final DID Core source selected by #420.

Every active change record will name at least one compatibility dimension and
one affected consumer. Referenced vector and quality IDs must exist, be active,
and share the record capability. A language-adapter migration also requires
mapping IDs; a Rust-only record uses an explicit reviewed `not-applicable`
mapping disposition and rationale rather than an unexplained empty list. Each
referenced mapping must share at least one cited vector, and every cited vector
must be covered by at least one cited quality declaration. This prevents
decorative evidence lists while allowing a focused subset of a larger packet.

The compatibility-impact declaration is explicit rather than inferred. A
`consumer-visible` disposition requires one or more canonical ledger IDs. A
`behavior-neutral` disposition requires zero ledger IDs, a bounded scope from
the closed governance/documentation/test-only/internal-refactor/tooling set,
and a substantive rationale. Unknown or omitted dispositions fail readiness.
Semantic review remains responsible for detecting a dishonest classification.

## Security, privacy and maintenance evidence

The validator treats every registry, impact record, selector, path, and prose
field as untrusted local input. It will reject unknown fields, duplicates,
control characters, unsafe or symlinked paths, excessive file/collection/text
sizes, malformed identifiers and versions, dangling or capability-mismatched
references, invalid lifecycle chains, incoherent class/release-note pairs,
and incomplete breaking/deprecation/removal evidence. Stored commands and
selectors are data only and are never executed.

Security changes may be ledgered, but historical unsafe behavior cannot use a
temporary compatibility simulation. It must select `reject-as-unsafe`, fail
closed, document the break, and provide rollback that does not restore the
unsafe behavior. Record text and rendered output contain public repository
metadata only; secrets, raw inputs, credentials, prompts, personal data, and
production identifiers are prohibited.

Canonical records are append-preserving evidence. Corrections use explicit
`supersedes`/`replaced_by` links and a registry version increment; removal does
not erase history. Deterministic rendering is derived output and normal checks
fail on drift.

Unsafe and native-code evidence is unchanged because the implementation adds
no Rust, extension module, executable dependency, or native library.
Supply-chain evidence is limited to repository-owned Python, TOML, Markdown,
and the already pinned Nix/Python environment; no package is added. The
maintenance, release and security posture remains unchanged: metadata may
block an invalid candidate, but cannot authorize publication or promotion.

## Rejected or deferred candidates

Automatic release-note publication, consumer PR creation, deprecation timers,
SemVer selection, current slow-lane evaluation, runtime telemetry collection,
and release authorization are deferred to their owning changes. A1 closeout
#504 will validate the combined graph after #422 merges. SDK-TS adoption #492
will supply the first real consumer migration record if its implementation
changes a supported consumer surface.

## Open questions and blockers

None. The existing ADRs decide classification and lifecycle policy; the three
delivered registries provide stable local IDs; and the empty-ledger versus
synthetic-fixture separation resolves the only apparent acceptance tension.

## Evidence commands

Planning evidence uses `scripts/factory research-ready`,
`constraints-ready`, strict OpenSpec validation, and a durable preflight
receipt. Focused checker, renderer-drift, mutation, impact-disposition, and
combined factory commands are intentionally implementation work and are not
claimed by this planning packet. Those implementation commands are unrun at
planning time and will be reported exactly after their files exist.
