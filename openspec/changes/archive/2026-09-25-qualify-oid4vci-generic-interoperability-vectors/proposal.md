# Qualify generic OID4VCI interoperability vectors

## Why

The OpenID4VCI Final wallet-core matrix has one remaining missing row. The
historical Oxid/Portal fixture exchange cannot be copied safely: the exact
Portal revisions contain no explicit repository license, several fixtures are
issuer-side rather than wallet-core inputs, and the profile names a
Midnight-specific credential representation. Leaving those files
reference-only is correct, but the SDK still needs executable, chain-neutral
evidence that its public APIs express the generic wallet-side contract and
reject the known legacy wire shapes.

## What changes

- add an Apache-2.0, repository-authored OpenID4VCI Final wallet-core vector
  set containing visibly synthetic positive and negative inputs;
- add a closed provenance manifest with per-file SHA-256, normative mapping,
  expected result, authorship and immutable consumer-reference dispositions;
- execute every retained vector through `identus-oid4vci` public APIs and fail
  on fixture or manifest drift;
- publish a consumer compatibility review that maps the pinned Oxid and Portal
  evidence to accepted, rejected or downstream-only SDK behavior without
  copying donor bytes or claiming live interoperability;
- update the conformance matrix, M4 report, blueprint and IDR-023 ledger only
  after the executable evidence passes.

## Non-goals

No consumer repository mutation, HTTP execution, issuer implementation,
Midnight format semantics, credential verification, trust policy, storage,
official OpenID certification, product support promise, publication or
release. The suite is not a production Oxid-to-Portal end-to-end test and is
not a human team approval record.

## Capabilities

### Modified capabilities

- `oid4vci-final-conformance`: replace the remaining missing cross-consumer row
  with clean-room executable vectors, immutable provenance and a bounded
  consumer-design assessment.
- `ssi-upstream-program`: permit IDR-023/M4 functional completion when the
  reviewed matrix has no missing required row, while retaining every partial,
  unsupported and downstream limitation.
