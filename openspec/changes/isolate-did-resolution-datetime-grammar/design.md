# Design

## Private ownership boundary

`validate_datetime` remains the private entry point used by both public
constructors. A private, field-only parsed datetime value owns `month`, `day`,
`hour`, `minute`, `second`, and the year modulo 400. Its lexical constructor
borrows the original bytes, proves bounds/ASCII/terminal `Z`, proves year and
tail shape, and projects the fixed numeric fields without allocation.

The parsed value exposes one consuming or borrowed calendar-validation method
that derives leap state, month length, and valid normal/end-of-day time. A
single local invalid-result constructor keeps every branch mapped to the exact
existing static error. There is no public helper, generic parser framework,
predicate graph, or helper per conditional.

## Exact validation contract

The design preserves this order and behavior:

1. complete byte length, ASCII, and terminal `Z` guard;
2. optional negative sign and first year delimiter discovery;
3. minimum four digits and extended-year leading-zero rejection;
4. exact separator/digit tail recognition;
5. bounded numeric projection and year-modulo-400 fold;
6. Gregorian month/day validation;
7. normal time or exact `24:00:00`; and
8. identical success with the caller's exact original string/allocation.

All failures remain indistinguishable at the public error boundary. The design
does not parse, allocate, normalize, log, or retain failed input.

## Characterization boundary

Before production movement, a compact table-driven matrix binds every tail
separator position, representative non-digit positions, sign/year boundaries,
month/day/leap-century boundaries, normal-time limits, exact end-of-day, and
constructor/Serde equivalence. Existing deterministic matrices continue to
bind large extended years, the public byte ceiling, and exact spelling.

## Compatibility and code-health ratchet

The normalized public API, manifests, and wire behavior remain identical. The
touched validator signal must disappear without producing an equivalent large
parser/method, forwarding-only helper chain, generated code, moved test,
waiver, allocation, or weaker threshold. Canonical evidence is rebound to the
protected implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are accepting a malformed tail, changing negative/extended-year
spelling, changing century leap behavior, accepting `24` with nonzero minute
or second, changing error projection, or allocating on invalid input. The
characterization matrix, existing property tests, public/source diff, and
exact-diff review make those visible. Rollback restores the single validator
body without consumer or data migration.
