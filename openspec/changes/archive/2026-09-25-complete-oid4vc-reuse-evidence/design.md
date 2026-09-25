# Context

PR #392 merged a sound architectural decision but its source matrix was not a
complete acceptance mapping for issue #391. This follow-up makes the missing
evidence reviewable without reopening the production decision.

# Goals

- Map every requested OID4VC seam and candidate in one table.
- Add independent positive and negative DCQL vectors.
- Quantify dependency, source, compile, memory, boundary, and rollback costs.
- State exactly what cannot be measured yet.

# Decisions

## Keep one canonical results report

`docs/research/siros-dcql-spike/results.md` owns the matrix, measurements,
interpretation, and acceptance checklist. Existing portfolio ledgers link to
it rather than duplicating figures.

## Add vectors to the isolated fixture

Clean-room tests exercise duplicate identifiers, empty requests, required
sets, claim values, paths, holder binding, and combination limits through
public candidate APIs. They do not enter root tests or production graphs.

## Treat measurements as bounded diagnostics

The report records exact observed source lines, incremental package names,
cold-check wall time and process RSS. It describes candidate allocation shapes
from public code paths and APIs but does not add unsafe instrumentation.

# Rollback

Remove the report and additive fixture tests. ADR 0156 and all production code
remain valid.
