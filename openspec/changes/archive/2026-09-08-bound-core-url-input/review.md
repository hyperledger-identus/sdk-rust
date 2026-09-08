# Distinct local review

## Scope reviewed

- Exact diff from `origin/develop@beb0f24` through implementation `d79ef53`.
- `crates/core` public API, all generated validated construction paths, error
  precedence, redaction behavior and dependency boundary.
- Canonical spec and `SDK-LIM-007` atomicity against issues #193 and #168.

## Findings

No blocking finding remains.

1. The first implementation check was correctly positioned before every
   input-proportional syntax operation, but its initial tests used only a valid
   oversized URL. A malformed oversized-input test was added in `d79ef53` to
   prove `TooLong` precedence rather than merely inspect source order.
2. The fixed 8,192-byte limit meets RFC 9110's recommended 8,000-octet HTTP
   interoperability floor without pretending that RFC 3986 defines a universal
   maximum.
3. Every generated validated path converges on `validate_url`; no constructor
   override or macro change creates divergent validity.
4. Adding a variant to the closed public `UrlError` enum and rejecting a former
   input class are acknowledged source/behavior compatibility events. Issue
   #193 authorizes them for the unpublished 0.0.0 active-development surface.
5. Existing in-budget bytes, display, equality, hashing, serialization and
   stable core error code/family remain unchanged.
6. No dependency or native boundary was introduced. The exact normal cone,
   Cargo manifests and lockfile are unchanged.
7. Constraint language narrows only the named `Url` gap. It retains both the
   outer-allocation caveat and repository-wide inherited audit limitation.

## Non-blocking observations

- The hand-rolled `Url` type is intentionally not a complete RFC 3986 parser;
  replacing it with `url` or `fluent-uri` remains a separate consumer-driven
  decision.
- `new_unchecked` remains a crate-private trusted hatch and intentionally does
  not claim the validated input invariant.
- Local x86_64-Linux validation is impossible on this host; hosted CI remains
  mandatory before merge.
- The archive wrapper can report success after OpenSpec's duplicate-requirement
  abort. The archived content and canonical requirement were verified intact;
  the factory defect is isolated for a separate issue and fix.

## Verdict

The implementation is cohesive, has explicit compatibility authority, closes
the named intrinsic `Url` bound without overstating upstream allocation safety,
and passes the complete compatible gate. It is ready for safe archive and an
issue-linked PR to `develop`.
