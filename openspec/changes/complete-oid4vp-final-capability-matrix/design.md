# Design

## Closed matrix

Store RFC 4180-compatible CSV under `docs/conformance`. Stable rows map one
Final capability group to its section, wallet-core relevance, status,
implementation paths, canonical spec paths, executable test paths, exact
limitation, provenance, and focused owner.

The statuses describe the bounded SDK profile. `implemented` is not a
certification grade. `partial` requires a real executable subset plus an exact
limitation and, for required work, a focused owner. `unsupported` states an
explicit current profile decision. `not-applicable` is reserved for separate
protocol/profile ownership.

## Offline validator

The standard-library checker rejects malformed schemas, duplicate IDs,
unknown enums, missing section groups, unsafe/missing evidence paths,
incomplete implemented/partial rows, missing required owners, and an IDR-024
delivered state while gaps remain. It prints a deterministic summary.

Unit tests create temporary repositories and exercise success plus each
fail-closed class. `scripts/check-factory.sh` and the factory fixture include
the new artifacts.

## Reference harness and adapters

The Rust integration test remains the executable protocol harness. It runs
the versioned clean-room packets through public crate APIs. Two test-only
serde input structures model representative camelCase TypeScript and Swift
adapter boundaries, translate immediately into SDK-native inputs, and assert
the same response lineage and error categories. These structures are not
exported and claim no donor compatibility.

## Knowledge report

The report explains the matrix semantics, current verdict, section findings,
child issue hierarchy, adapter boundary, assurance limits, and the exact
condition for IDR-024 completion. README and bootstrap inventory link to it
rather than duplicating the matrix.

## Rejected alternatives

JSON Schema adds tooling for a flat inventory. Markdown-only evidence is not
fail-closed. A live verifier service would introduce network nondeterminism.
One giant implementation PR would erase the independent security and protocol
boundaries captured by #544 through #547.
