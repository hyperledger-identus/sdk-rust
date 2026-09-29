# Design

## Private ownership boundary

`decode_resolution_options` remains the adapter coordinator. It derives the
optional accepted representation, handles absent/empty query input, and applies
the raw query byte ceiling. A private `ResolutionQueryFields` owns the decoded
name set, optional common fields, and ordered extension map while consuming
bounded query parameters in source order.

The owner finalizes the mutually exclusive version selection and calls the same
`ResolutionOptions::new`. Existing percent decoding, boolean parsing, typed
version constructors, representation negotiation, handler, resolver port, and
response projection retain their responsibilities.

## Exact phase order

The design preserves this order:

1. derive `accept` from the negotiated representation;
2. construct empty options for absent/empty input;
3. check raw query bytes;
4. check parameter count before raw structure;
5. split one name/value pair, decode name, then decode value;
6. check empty/control name, control value, then decoded-name duplication;
7. reject query `accept` or convert/store the selected field;
8. reject simultaneous version ID and time; and
9. call the same validated options constructor.

No property is normalized or reordered, literal plus remains literal, unknown
options remain ordered string extensions, and every failure remains before
resolver invocation.

## Characterization boundary

Before production movement, a compact matrix binds first errors across raw
size/count, missing `=`, name/value decode, empty/control/duplicate checks,
known-option parsing, version conflict, and final construction. A counting
resolver or existing equivalent must prove zero calls on every rejection.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched signal
must disappear without an equivalent owner/helper or module signal,
forwarding-only chain, generated code, moved tests, waiver, new allocation
class, or weaker threshold. Canonical evidence is rebound to the protected
implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are name/value decode inversion, duplicate-before-control drift,
early/late known-option parsing, version-conflict drift, extension reordering,
query-controlled accept, extra allocation, resolver invocation on rejection,
and diagnostic disclosure. The matrix, existing adapter corpus, allocation and
redaction review, and exact diff make those visible. Rollback inlines the
private owner without public, resolver, adapter, or stored-data migration.
