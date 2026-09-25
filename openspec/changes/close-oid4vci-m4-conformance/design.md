# Design

## One canonical matrix

Store the coverage inventory as RFC 4180-compatible CSV under
`docs/conformance`. Each row has a stable capability ID, Final section,
wallet-core relevance, one closed status, public surface paths, canonical spec
paths, test paths, limitation text, provenance class and optional focused issue.
Semicolon-delimited path cells keep the document readable without introducing a
new parser dependency.

The closed statuses are `implemented`, `partial`, `unsupported`, and `missing`.
They describe the bounded SDK surface, not standards compliance grades.

## Offline validator

Add a standard-library Python validator that:

- rejects malformed headers, duplicate capability IDs and unknown enum values;
- requires coverage for Final sections 4 through 12, including explicit
  unsupported rows for sections 10 and 11;
- rejects absolute, escaping, missing or non-regular evidence paths;
- requires implementation, canonical spec and executable test evidence for
  `implemented` rows;
- requires a limitation for every `partial`, `unsupported` or `missing` row;
- requires one focused issue for a `missing` required wallet-core row;
- permits consumer provenance only as `reference-only` with immutable source
  revisions and license disposition in the companion report;
- prints a deterministic status summary for CI and reviewers.

Unit tests operate on temporary repositories and cover success plus each
fail-closed class. `scripts/check-factory.sh` runs the validator so matrix drift
is part of the required fast lane.

## Human report

The Markdown report explains status semantics, summarizes sections, lists
residual gaps and owners, records exact consumer provenance/license decisions,
reconciles issue #7 tasks and recommends close or continue for M4. It does not
duplicate every path from the CSV.

## Fixture disposition

Do not copy the current consumer fixture set. Record the immutable Portal and
Oxid revisions/hashes as reference-only evidence. The absence of an explicit
license at the Portal source revision and the product-specific
`midnight_cbor_phase1` format are independent blockers to generic import.

## Rejected alternatives

A JSON schema would add tooling without improving a flat inventory. Markdown
tables alone are too easy to drift. Rust tests cannot express unsupported
scope or legal provenance. Automatically fetching consumers would break
offline reproducibility and repository isolation.
