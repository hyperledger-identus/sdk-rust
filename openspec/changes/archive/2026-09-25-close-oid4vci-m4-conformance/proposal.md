# Close OID4VCI M4 with traceable conformance evidence

## Why

Thirty-seven bounded OID4VCI deliveries now cover the wallet-side issuance
core, but issue #7 still describes a much earlier shape and there is no single
auditable mapping from the Final specification to public APIs, canonical
contracts, tests, explicit limitations and follow-up owners. Without that
mapping, closing M4 would confuse implemented structural protocol behavior with
complete OID4VCI conformance or consumer interoperability.

## What changes

- add one machine-readable OpenID4VCI 1.0 Final wallet-core coverage matrix;
- validate its schema, section coverage, repository paths, status evidence and
  gap ownership in the factory gate;
- publish a human-readable M4 report generated from the same facts;
- classify the Oxid/Lace ID Portal vectors as importable, reference-only or
  unsuitable using immutable provenance and license evidence;
- reconcile issue #7 and every M4 child without inventing protocol behavior;
- open focused issues for evidence-backed gaps and make a truthful close or
  continue recommendation for M4.

## Non-goals

No new OID4VCI wire or state behavior, HTTP client, browser integration,
credential format implementation, notification or encryption flow, trust,
storage, product policy, consumer mutation, publication, certification or
release claim.

## Capabilities

### Added capabilities

- `oid4vci-final-conformance`: maintain a closed, machine-checked mapping from
  Final sections to SDK evidence, limitations and focused gap owners.

### Modified capabilities

- `ssi-upstream-program`: make IDR-023/M4 completion depend on the traceable
  matrix and honest reconciliation rather than the age of the parent issue.
