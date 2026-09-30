# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

The current implementation is draft PR #509 at exact revision
`7e0cbf76562f15f02761fcc839929afe9a94fde7`, based on the #508 merge revision
`644993c859b87e2da16612fcdf7cf1868cfe5981`. It introduces a closed TOML
registry, deterministic Markdown projection, Python standard-library validator,
and mutation suite for four SDK-TS DID/DID URL mappings.

The first independent review found six validator-completeness gaps and one
remediation fixed all six. Verification of that exact green head then
reproduced six further gaps: parent-directory symlink escape, invalid canonical
path rendering, mapping/field direction contradiction, missing loss evidence
for error mappings, a single-source fixture assumption, and a compatibility
window extending into an unevidenced SDK-TS major. The durable finding summary
is PR #509 comment `5907933432`; issue #510 records full acceptance.

The landed cross-language catalog is now physically present on this base. Its
validator already proves repository-contained fixture paths through component
symlink checks plus resolved containment. The adapter registry can adopt that
local pattern rather than invent a second path policy. Because mappings refer
to catalog evidence instead of owning it, the same vector may validly support
several language mappings; mapping IDs remain unique, while vector references
must resolve exactly once in the catalog.

## Normative sources

- ADR 0170 at repository revision
  `7e0cbf76562f15f02761fcc839929afe9a94fde7` is the architecture authority:
  Rust contracts remain canonical and language adapters are explicit,
  versioned, observable, bounded, and transitional.
- Canonical `language-adapter-mappings` specification at the same revision
  requires explicit loss, stable error identity, Rust-owned bounds, bounded
  lifecycle, and deterministic rendering.
- `scripts/check-cross-language-vectors.py` at revision
  `644993c859b87e2da16612fcdf7cf1868cfe5981` is the local repository-containment
  oracle. It is Apache-2.0 first-party source and has no redistribution issue.
- `docs/conformance/cross-language-vector-catalog.toml` at revision
  `644993c859b87e2da16612fcdf7cf1868cfe5981` is the authoritative local vector
  namespace for this A1 slice.
- SDK-TS remains pinned at Apache-2.0 revision
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, version 8.1.4. No later SDK-TS
  release or revision was reviewed by this slice.

No external protocol or draft interpretation changes. DID Core 1.0 continues
to govern the underlying vectors through #420; this change governs repository
metadata integrity and does not reinterpret DID syntax.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Reuse the cross-language catalog containment pattern | sdk-rust `644993c859b87e2da16612fcdf7cf1868cfe5981` | `adopt` | Already reviewed first-party logic rejects path traversal, symlink components, and resolved escape without a new dependency. | A shared bounded-path helper becomes available without coupling the checkers. |
| Distinct Cargo package and Rust API path fields | draft registry v1 at `7e0cbf76562f15f02761fcc839929afe9a94fde7` | `retain-local` | Prevents duplicated crate names and keeps stable public error codes out of synthesized Rust paths. | A generated schema supplies typed identity variants. |
| Structured per-distinction loss records | draft registry v1 | `retain-local` | Loss and unsupported input are different concepts; both value and error mappings need reviewable consequences and mitigations. | Two independent mappings prove a smaller closed shape is sufficient. |
| Reuse `unsupported` as the only loss record | draft registry v1 | `not-adopt` | Error-class coalescing is lossy without making a value unsupported; the field cannot express that case honestly. | Never, unless unsupported is deliberately redefined with a schema migration. |
| Closed direction compatibility matrix | draft registry v1 | `adopt` | A one-way mapping cannot contain a reverse or bidirectional field without contradicting its public claim. | A future composition model proves an explicit neutral direction is needed. |
| Exact-patch interval `>=8.1.4,<8.1.5` | SDK-TS 8.1.4, revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | `adopt` | One pinned revision proves one release, not the complete 8.x or 9.x line. | Additional pinned releases and differential evidence justify a wider window. |
| Keep `>=8.1.4,<10.0.0` | draft registry v1 | `not-adopt` | It silently claims compatibility through an unreviewed major version. | Only after explicit 9.x evidence and a versioned mapping decision. |
| Dynamically materialize named source fixtures | Python standard library in repository toolchain | `adopt` | Preserves additive registry behavior without copying the complete source tree or hardcoding DID paths. | Fixture count or size makes focused copying materially slower than a bounded snapshot. |
| Resolve vector IDs inside the mapping validator | catalog v1 at `644993c859b87e2da16612fcdf7cf1868cfe5981` | `adopt` | The catalog now exists on the base, so unknown references can fail at their owning contract rather than at a later parent milestone. | Catalog schema version changes and exposes a stable shared parser API. |
| Add a third-party SemVer/path/schema package | not applicable | `not-adopt` | Closed x.y.z intervals, safe repository paths, and TOML parsing are small and already supported by Python 3.11 standard library. | Version grammar expands to full SemVer ranges or shared schema generation is selected. |

## Compatibility and dependency evidence

The public and wire compatibility impact is none: no Rust crate or consumer
surface changes. The mapping schema is not merged or released, so correcting
its v1 field shape requires no migration and avoids freezing a malformed
canonical identity. Generated Markdown remains a review facade, never a source
of truth.

The direct and resolved dependency cone remains unchanged. Validation uses
Python 3.11 `pathlib`, `tomllib`, `re`, and `shutil`; no Rust crate, native
library, package-manager resolution, or network fetch is introduced. Rust MSRV
1.89.0, primary/etalon Rust 1.98.1, features, and supported targets are not
affected. The scripts execute in factory/Nix evidence on Linux and macOS; this
does not create a Windows support claim.

License and provenance evidence is entirely first-party: modified files are
Apache-2.0 SDK-Rust sources at the revisions above, and the SDK-TS source is
retained only as an immutable Apache-2.0 reference. No source or fixture is
copied from a new repository.

Rollback removes the #510 active change and replacement branch while draft
#509 remains blocked. Once the replacement PR lands, later semantic changes use
mapping versioning and its declared deprecation/replacement lifecycle.

## Security, privacy and maintenance evidence

The parent-symlink case is a repository-evidence integrity defect: a crafted
checkout could redirect validation to an outside fake constant and weaken a
declared resource bound. The design must reject traversal, every symlink
component, missing/non-regular files, resolved escape, oversized source files,
and non-literal or ambiguous public constants without traceback. It does not
read secrets or accept runtime network input.

Stable error codes and caller-input redaction remain unchanged. Explicit loss
records improve privacy and compatibility review by preventing a merged legacy
class from silently erasing distinctions. Cross-catalog resolution consumes
only public Apache-2.0 metadata and fixture identifiers.

The direct and resolved dependency cone contains no reachable third-party
unsafe or native code because the change adds no dependency. Supply-chain,
license, release, and maintenance posture are unchanged. The bounded validator
and focused mutation fixtures remain standard-library-only and offline.

## Rejected or deferred candidates

Generated JSON Schema, Pydantic, a shared checker framework, binding code,
SDK-TS changes, and Rust runtime APIs are deferred. They add coupling or scope
without improving this contract. Broad SDK-TS compatibility, Swift/Kotlin
records, consumer E2E, and #492 canary implementation remain separately owned.

## Open questions and blockers

None. The accepted ADR, existing canonical spec, issue #510, exact review
counterexamples, and landed #420 catalog decide every implementation choice.
No human product decision or external source is required.

## Evidence commands

Commands run during planning:

```text
scripts/factory doctor
python3 scripts/check-language-adapter-mappings.py .
python3 scripts/tests/language-adapter-mappings.py
scripts/check-factory.sh .
/nix/var/nix/profiles/default/bin/nix flake check
claude -p "/review https://github.com/hyperledger-identus/sdk-rust/pull/509"
gh api repos/hyperledger-identus/sdk-rust/issues/505/sub_issues
```

The validator, mutation, factory, and local Nix commands passed on the reviewed
#509 head before this planning branch. Implementation commands for #510 are
intentionally unrun at planning time. A fresh focused suite, full factory,
local Nix flake, hosted exact-head fast lane, and one new independent discovery
review remain required after implementation.
