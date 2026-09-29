# Design

## Private ownership boundary

`AuthorizationRequest::try_into_authorization_response` remains the public
consuming entry point. A private query decoder owns the borrowed query, limits,
decoded-name set, and retained `ResponseFields`. It performs complete structural
and resource validation before returning fields.

A private correlator then owns the consumed request, parsed fields, response
length, expected state/issuer data, and advertised issuer rule. It exposes
semantic phases for exact transaction correlation and exclusive outcome
construction. Small helpers are acceptable only when they own one complete
invariant; there is no helper per conditional or field arm.

## Exact validation order

The design preserves this order:

1. reject empty/full-callback shapes and encoded query overflow;
2. scan bounded parameters in input order;
3. strict-form-decode bounded names and reject empty/decoded duplicates;
4. choose the known-role or generic value ceiling, decode once, and retain or
   discard the value;
5. require exact returned state;
6. apply the selected server's effective RFC 9207 issuer rule;
7. require exactly one success/error branch and forbid developer fields on
   success; and
8. validate selected-branch field grammar and construct the same redacted
   owned outcome.

Neither owner can alter grammar predicates, normalize values, inspect unknown
values after bounded decoding, expose request or field state publicly, invoke
transport, or broaden trust evidence.

## Characterization boundary

Before production movement, a compact matrix binds malformed/duplicate query
before correlation, state before issuer, issuer before branch shape, branch
shape before selected grammar, and success/error-specific grammar ordering.
Existing tests continue to bind each positive outcome, limit, error code,
issuer evidence, redaction surface, and request lineage.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. Both touched signals
must disappear without equivalent decoder/correlator signals, forwarding-only
chains, generated code, moved tests, waivers, new allocation, or weaker
thresholds. Canonical evidence is rebound to the protected implementation
squash in a second issue-linked PR.

## Risks and rollback

Primary risks are query/correlation priority drift, state or issuer authority
weakening, branch ambiguity, different role ceilings, unknown-field behavior,
extra zeroizing copies, and diagnostic disclosure. The matrix, existing corpus,
allocation/redaction review, and exact diff make those visible. Rollback inlines
the two private owners without consumer or stored-data migration.
