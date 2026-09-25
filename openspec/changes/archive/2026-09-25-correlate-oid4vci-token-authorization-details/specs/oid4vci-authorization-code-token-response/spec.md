# oid4vci-authorization-code-token-response Specification

## ADDED Requirements

### Requirement: Successful binding has one typed consuming correlation path

`RequestBoundAuthorizationCodeTokenResponse` SHALL expose a consuming
Authorization Details correlation transition under explicit positive limits.
The transition SHALL move the exact #360 lineage and Token Response core; it
SHALL NOT accept replacement metadata, configuration, response JSON or token
values. The OAuth error branch SHALL NOT expose this transition.

#### Scenario: only success can correlate

- **WHEN** callers hold the exclusive success or error outcome branch
- **THEN** only the success type can enter credential Authorization Details
  correlation

#### Scenario: failed correlation consumes the success

- **WHEN** response-local or lineage correlation fails
- **THEN** no reusable request-bound success or Token Response core is returned
