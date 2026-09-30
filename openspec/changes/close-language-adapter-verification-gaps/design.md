# Design

## Replacement correction boundary

This change is based on #509 head
`7e0cbf76562f15f02761fcc839929afe9a94fde7` because it corrects the unmerged
mapping contract. It will be reviewed as a separate issue-linked replacement PR,
not pushed directly into #509. The complete corrected branch opens a new PR to
protected `develop` and supersedes draft #509. This gives the replacement its
own exact-head CI and discovery/remediation budget while preserving the normal
base-branch policy. #509 is closed, not merged, only after the replacement is
ready.

## Canonical identity shape

Keep `canonical_crate` as the Cargo package identity. Replace the ambiguous
`canonical_module` plus `canonical_symbol` pair with one exact
`canonical_rust_path`, for example `identus_did::Did` or
`identus_did::Error::to_identus_error`. Error mappings retain their public
stable code in the closed error record (`rust_code`); the code is not a Rust
symbol.

The renderer emits separate Cargo package, Rust API path, and—only for error
records—stable-code columns. It never prepends a Cargo package to a Rust path
and never formats a stable error code as source syntax.

Because v1 is unmerged and unpublished, correcting these fields is a schema
fix rather than a compatibility migration. `schema_version` remains 1; no
consumer exists that could rely on the defective representation.

## Repository-contained canonical bounds

The root is resolved once. A canonical bound path must be non-empty,
repository-relative, contain no `..`, and remain under the root lexically. Walk
each path component from the root and reject any symlink before reading. Then
resolve the candidate strictly and require it to remain under the resolved
root, be a regular file, and stay below the repository's existing 2 MiB source
evidence ceiling.

The named bound remains an uppercase identifier and must resolve exactly once
to a literal public Rust `usize` constant. Missing, ambiguous, computed,
symlinked, outside-root, oversized, invalid UTF-8, and unreadable sources append
static diagnostics and return failure without traceback. The adapter byte
ceiling may equal or be less than the resolved Rust value, never greater.

## Direction matrix

The enclosing mapping direction defines the allowed field directions:

| Mapping direction | Allowed field directions |
| --- | --- |
| `bidirectional` | `both`, `rust-to-language`, `language-to-rust` |
| `rust-to-language` | `rust-to-language` |
| `language-to-rust` | `language-to-rust` |

One-way derived or accepted fields remain expressible inside a bidirectional
mapping. A one-way mapping cannot imply reverse conversion through `both` or an
opposite field. Error records contain no field mappings and inherit the
enclosing direction.

## Loss versus unsupported behavior

Add a closed `losses` array to every mapping. Each record contains a stable ID,
the lost distinction, its compatibility consequence, and the required
mitigation. Lossless mappings require an empty array; lossy mappings require at
least one record with unique IDs.

`unsupported` remains a closed value-boundary record. A lossy value mapping
may use matching IDs to connect a lost round-trip distinction to deterministic
rejection, but error-class coalescing needs only a loss record because the
value still surfaces through a stable canonical code. Every unsupported ID must
name a declared loss; not every loss must be unsupported.

The DID URL record declares its raw-query and absent-versus-empty losses. The
two error mappings each declare that the legacy `InvalidDIDString` class
coalesces canonical failure identities, with stable Rust code preservation as
the mitigation.

## Evidence-bounded version window

Schema v1 carries one `language_version` and one immutable
`language_revision`. Therefore its supported interval is exactly that patch:
lower bound equals the pinned version and exclusive upper bound increments its
patch by one. The four seed records change from `>=8.1.4,<10.0.0` to
`>=8.1.4,<8.1.5`. Semantic-version components use canonical decimal spelling;
leading-zero aliases cannot enter the rendered compatibility contract.

A wider interval needs a future schema capable of naming additional pinned
revisions plus differential evidence. Version negotiation and deprecation
remain unchanged.

## Cross-catalog resolution

Load the canonical catalog with bounded standard-library TOML parsing. Count
its vector IDs and retain each vector's capability and targets. Every mapping
reference must resolve exactly once, match the mapping capability, and include
the mapping language target. Value mappings consume `success` vectors; error
mappings consume vectors whose expected outcome is one of their declared stable
Rust codes. Duplicate vector references across mappings are allowed because
mappings consume shared evidence; duplicate mapping IDs remain invalid.

This removes the stale deferral that predated #508. The catalog's own checker
continues to own packet, provenance, selector, and supersession integrity.

## Mutation fixtures

The positive fixture parser discovers every unique `canonical_bound_path` in
the registry and copies each safe source to the temporary root while preserving
its relative path. It also copies the canonical vector catalog. Invalid-path
mutations do not need materialization and must fail through validator
diagnostics.

Negative evidence covers a parent-directory symlink escape, resolved
outside-root path, wrong/duplicated/non-literal constant, contradictory field
directions, lossy mapping without loss records, unsupported ID without a loss,
unknown/ambiguous/wrong-capability/wrong-target vector, broad version window,
and malformed tables without traceback. Positive evidence retains the
additive fifth-language record using existing shared vector IDs and adds a
second canonical source path so the fixture is not DID-file-specific.

## Stop boundary

The implementation ends at registry metadata, deterministic docs, validator,
mutation suite, OpenSpec synchronization, and factory evidence. It does not
generate bindings, change Rust APIs, edit consumers, or begin #492.
