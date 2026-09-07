## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and advance its
issue reference from completed child #135 to active child #137. Issue #137
SHALL deliver only a bounded, request-bound OID4VCI Final Credential Nonce HTTP
response transition: positive field/body limits, 2xx status validation, exact
case-insensitive `application/json` media-type classification, RFC-structured
bare `no-store` recognition, reuse of the strict response-body parser, and
fieldless redaction-safe errors without retaining transport inputs.

The slice SHALL NOT claim the full OID4VCI engine, metadata discovery or
retrieval, signed metadata, complete RFC 8414 conformance, trust, HTTP
execution, DNS/TLS/redirect/private-network policy, generic header collection,
compression/framing validation, DPoP handling, Issuer nonce generation or
unpredictability, actual response provenance, nonce freshness/expiry or replay
safety, proof construction/verification, Credential Requests/Responses,
consumer adoption, publication or release.

#### Scenario: ledger points at the active bounded child

- **WHEN** issue #137 is implemented
- **THEN** `IDR-023` references #137 with `delivery_status=in_progress`
- **AND** #7 and #20 remain the open component/program parents

#### Scenario: response envelope validation is not protocol-engine completion

- **WHEN** Final Nonce HTTP response status/media/cache/body gates pass
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered
