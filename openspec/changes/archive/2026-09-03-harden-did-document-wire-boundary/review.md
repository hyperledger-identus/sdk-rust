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
dependency-free. Differential execution confirms that `uriparse` 0.6.4 rejects
the RFC 3986 `IPvFuture` production; the SDK retains its standards-conforming
acceptance. The oracle also panics for the minimized malformed `1bad:value`;
the SDK rejects it without unwinding. Both behaviors are pinned alongside the
SDK's stricter 4,096-byte limit. #35 retains sanitizer cargo-fuzz; #41 consumes
the private scanner later. No consumer edit, release, publication, FFI, new
unsafe code, or public generic JSON API is justified.

# Post-implementation semantic, security, and API review

- **Date:** 2026-09-03
- **Reviewed implementation head:**
  `41a85cbe8522a98d9e4cda632012e9a8e2b30567`
- **Result:** no unresolved blocker; suitable for exact-head hosted review

The complete develop-base-to-implementation diff was reviewed after the
focused and full local gates completed.

1. **Wire ordering:** `from_json_slice` applies the existing 256 KiB ceiling,
   streams the complete JSON value through the duplicate/resource scanner,
   and only then performs typed deserialization. No last-value-wins map can
   precede duplicate detection on the documented raw entry points.
2. **Duplicate semantics:** comparison uses decoded `String` names inside one
   open object. Escaped-equivalent names collide, while reuse in sibling or
   nested objects remains valid. Known DID fields and arbitrary extension,
   context, verification, JWK, service and endpoint maps share this rule.
3. **Resource behavior:** traversal stops beyond 64 open containers, 16,384
   values, 128 members in one object, or 128 KiB of decoded names retained by
   simultaneously open objects. The scanner holds one ordered name set per
   open object and releases its accounted bytes when that object closes.
4. **Error hygiene:** scanner failures carry only static categories. The new
   local duplicate reason and every public rendering preserve
   `did.invalid_document` and omit caller-controlled names, values, offsets and
   document bytes; an explicit secret-marker regression proves the boundary.
5. **URI conformance:** 1,000 deterministic RFC 3986 component combinations
   agree with NeoPRISM's exact `uriparse` 0.6.4 oracle. The SDK's stricter byte
   cap, standards-conforming IPvFuture acceptance and safe rejection of the
   oracle's minimized panic input are explicit, pinned differences.
6. **Compatibility and dependencies:** unique-name raw documents and native
   construction retain their semantic behavior. The public change is additive
   except for the intentional ambiguous-JSON rejection. `uriparse` is dev-only
   and absent from the normal `identus-did` dependency cone.
7. **Portability and ownership:** Rust 1.85, WASM, Android ARM64 and iOS ARM64
   Nix lanes pass. No unsafe code, chain-specific policy, donor source, FFI,
   publication or consumer edit entered the slice.

## Corrections made during implementation and review

- Minimized and classified an oracle panic for `1bad:value` instead of
  treating the oracle as authoritative for malformed inputs.
- Preserved RFC 3986 IPvFuture acceptance after confirming that the oracle
  rejects the production.
- Added an explicit caller-controlled duplicate-name redaction regression.
- Kept the scanner crate-private so #41 can reuse it internally without
  prematurely committing a generic public JSON API.
