# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances to Authorization Code Token Request construction

After #356 delivers, `IDR-023` SHALL reference focused child #358 with
`delivery_status=in_progress`. Issue #358 SHALL consume one correlated success
and construct a bounded transport-neutral unauthenticated public-client Token
Request for the exact selected server. It SHALL NOT execute HTTP, authenticate
a confidential client, parse or bind a Token Response, or claim authorization
or issuance completion.

#### Scenario: response correlation hands off to request construction

- **WHEN** #356 is delivered and #358 is open
- **THEN** the canonical backlog references #358 without marking IDR-023 done

Before #358 integrates, the row SHALL reference one open focused successor
for binding a bounded Token Endpoint success/error HTTP response to the exact
request lineage without adding transport effects.

#### Scenario: request construction remains independently reversible

- **WHEN** #358 is ready to integrate
- **THEN** IDR-023 references its open response-binding successor and remains
  in progress
