# Pre-implementation semantic and security review

- **Date:** 2026-09-03
- **Issue:** #38, child of #5 / `IDR-005`
- **Develop base:** `87ecd9cb970da0e8b3938c62b1eddc8ce0b73995`
- **Result:** contract is implementable with no unresolved blocker

## Standards and compatibility findings

W3C DID Core 1.0 makes verification identifiers, relationships, aliases,
contexts, and service values URI/open-map surfaces. RFC 8259 only says object
names SHOULD be unique but explicitly warns that duplicate handling is
unpredictable. A security boundary must therefore select deterministic reject,
not inherit a map library's last-value behavior. This is an intentional
pre-release wire tightening and preserves the stable public error code.

RFC 3986 distinguishes `URI`, which permits a fragment, from `absolute-URI`,
which does not. DID document resource identifiers need the `URI` production.
The issue's phrase “absolute URI” means scheme-bearing rather than the narrower
ABNF rule. The spec and differential plan make that interpretation explicit.

## Security and resource findings

A duplicate scanner that first constructs `serde_json::Value` would already
have lost duplicates. A duplicate-aware generic tree would retain correctness
but double peak memory. A streaming visitor followed by ordinary typed parsing
uses two CPU passes while retaining only object-local decoded names. Explicit
byte, depth, node, member, and live-key bounds make that trade-off acceptable
and independently testable.

Decoded JSON strings, rather than source spelling, define name equality. Error
variants and messages must never carry the duplicate name or document bytes.
The scanner must require complete input so it cannot bless a unique prefix and
leave trailing syntax to a differently classified parser path.

## Provenance and boundary findings

NeoPRISM's URI parser and `uriparse` lock provide independent behavior, while
its document records are too permissive to port. Midnight, Lace, and Oxid show
extension, normalization, collision, and holder-bound concerns, but their
method/product rules stay downstream. Apollo has no DID parser component. No
donor code or fixture is required; exact paths and revisions are recorded in
issue #38 and the ADR.

`uriparse` is acceptable only as a dev dependency pinned to NeoPRISM's exact
0.6.4. The production parser remains small, offline, chain-neutral, and
dependency-free. #35 retains sanitizer cargo-fuzz; #41 consumes the private
scanner later. No consumer edit, release, publication, FFI, new unsafe code, or
public generic JSON API is justified.

# Post-implementation semantic, security, and API review

Pending implementation and exact-head verification.
