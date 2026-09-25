# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances through correlated Authorization Responses

After #354 delivers, `IDR-023` SHALL reference focused child #356 with
`delivery_status=in_progress`. Issue #356 SHALL parse one bounded query-mode
Authorization Response, correlate exact state and selected-server RFC 9207
issuer behavior, and produce a redacted success or OAuth error outcome. It
SHALL NOT exchange the code or perform callback/browser/HTTP behavior.

#### Scenario: request construction advances to response correlation

- **WHEN** #354 is delivered and #356 is open
- **THEN** the canonical backlog references #356 without marking IDR-023 done

### Requirement: Authorization Response delivery names a code-exchange successor

Before #356 integrates, the row SHALL reference one open focused successor
that constructs the Authorization Code Token Request from the correlated
success lineage without executing HTTP.

#### Scenario: response correlation remains independently reversible

- **WHEN** #356 is ready to integrate
- **THEN** IDR-023 references its open code-exchange-request successor and
  remains in progress
