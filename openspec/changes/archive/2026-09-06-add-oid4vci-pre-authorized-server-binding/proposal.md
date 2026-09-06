# Bind an OID4VCI Pre-Authorized Code server

## Why

The existing OID4VCI states validate a Credential Offer, its grants, unsigned
Credential Issuer Metadata, and a partial Authorization Server Metadata core.
They do not yet prove that a particular Authorization Server is both permitted
by the issuer/offer documents and able to receive the offered Pre-Authorized
Code grant. Constructing a Token Request before that agreement would leave
mix-up, unsupported-grant, and absent-endpoint checks to each consumer.

## What changes

- Add one consuming state transition that binds a matched offer/issuer-metadata
  state to caller-selected Authorization Server Metadata.
- Require the offered Pre-Authorized Code grant, effective issuer-metadata
  membership, exact agreement with any grant hint, explicit Pre-Authorized Code
  grant advertisement, and a Token Endpoint.
- Preserve ownership of both validated predecessor states and expose the
  selected metadata without copying bearer-adjacent values.
- Add static redacted failure classes and cross-document positive/negative
  evidence.

## Non-goals

This change does not discover, fetch, rank, or trust servers; construct or send
a Token Request; authenticate a client; accept a Transaction Code; execute a
flow; handle responses, retry, or replay; add Authorization Code behavior;
edit a consumer; publish; release; or promote to `main`.

## Impact

- **Issue:** #121, child of #7 and #20 / `IDR-023`; predecessor #119.
- **Owner:** existing unpublished `identus-oid4vci` crate.
- **Compatibility:** additive experimental API and static errors; no wire output
  or released compatibility commitment.
- **Dependencies:** unchanged; the transition allocates and parses nothing.
- **Rollback:** remove the additive state, transition, errors, tests, ADR, and
  roadmap entry while retaining all five earlier OID4VCI slices.
