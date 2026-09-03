## Context

The current DID document model validates typed identifiers, collection sizes,
extension trees, public key material, and semantic collisions. Its bounded raw
entry point nevertheless calls `serde_json::from_slice` directly. RFC 8259
permits parsers to handle duplicate object names differently; `serde_json`
materializes maps with one surviving value, so the SDK cannot reject the
original ambiguity after deserialization.

W3C DID Core 1.0 uses RFC 3986 URI values throughout documents. The SDK owns a
small dependency-free URI recognizer; NeoPRISM instead uses `uriparse` 0.6.4.
Independent differential evidence can harden the SDK parser without adding a
runtime dependency or inheriting NeoPRISM's permissive public records.

The exact base is `87ecd9cb970da0e8b3938c62b1eddc8ce0b73995`.
NeoPRISM `d6ad1ec`, midnight-identity `427f857`, Lace ID Portal
`804de0a`, Oxid `bfe3b48`, and Apollo `ccee22b` are read-only evidence.
No donor code or fixture is copied.

## Goals / Non-Goals

**Goals:**

- Detect duplicate names recursively before a JSON map can discard one.
- Bound the scanner independently of later semantic validation.
- Keep all errors stable and free of caller-controlled text.
- Reuse the scanner in future DID resolution-result hardening.
- Establish deterministic generative and independent URI conformance evidence.
- Preserve unique-name wire and native behavior on Rust 1.85, native, mobile,
  and WASM targets.

**Non-Goals:**

- JSON-LD processing, URI normalization/equivalence, scheme or SSRF policy.
- DID-method, resolver, chain, storage, signing, trust, or product behavior.
- Sanitizer cargo-fuzz infrastructure (#35) or resolution-envelope adoption
  and standards drift (#41).
- A public generic JSON parser, new crate, runtime dependency, FFI, release,
  publication, or downstream edit.

## Decisions

### Decision 1: scan raw JSON before typed deserialization

A crate-private module uses a recursive `serde::de::Visitor` with
`IgnoredAny`-equivalent scalar handling and a `BTreeSet` local to each object.
Each name is decoded by the JSON deserializer and checked before its value is
visited. Therefore textual and escaped spellings that decode to the same name
collide. The visitor discards values instead of constructing a generic JSON
tree; typed deserialization runs only after the preflight succeeds.

This intentionally traverses valid input twice. A one-pass custom DID document
deserializer would duplicate every serde wire structure, couple the security
primitive to one model, and be difficult for #41 to reuse. Deserializing first
to a duplicate-aware `Value` would allocate both a complete generic tree and
the typed model. The streaming preflight has a smaller and auditable memory
shape.

### Decision 2: distinguish duplicate, resource, and malformed failures

The scanner returns a private closed reason: duplicate name, too deep, too many
nodes, too many members, too many live key bytes, or malformed JSON. The DID
document entry point maps duplicate names to a new
`DocumentError::DuplicateJsonProperty`; scanner resource failures to the
existing redacted document resource reasons where semantics match; and syntax
or trailing data to `MalformedJson`. Neither private nor public error contains
the rejected name, value, offset, or document bytes.

Direct generic serde remains supported for semantic JSON values, but the raw
lexical guarantee is deliberately attached to `from_json_slice` and
`from_json_str`. A native map or materialized `serde_json::Value` cannot
represent a discarded duplicate and therefore cannot claim lexical parity.
All representable semantic invariants continue to converge on the existing
`DidDocument::validate` path.

### Decision 3: use explicit scanner limits under the raw envelope

The 256 KiB raw byte check remains first. Scanner work is additionally limited
to 64 nested containers, 16,384 visited JSON values, 128 members in one object,
and 128 KiB of simultaneously retained decoded object names. These ceilings
cover the existing semantic maximum of 32 extension levels, 4,096 extension
nodes, 64 extension properties, and the fixed document structure while
preventing the preflight itself from becoming an availability primitive.

The scanner is parameterized by a private limits value so #41 can reuse the
mechanism with the resolution envelope's own byte ceiling. It has no logging,
I/O, global state, unsafe code, or recursion beyond the explicit depth limit.

### Decision 4: retain the SDK URI parser and add a dev-only oracle

The production `Uri` remains dependency-free, allocation-after-validation,
and exact-spelling preserving. `uriparse = 0.6.4` is pinned as a workspace
development dependency solely for differential tests because it is the exact
version in NeoPRISM's lockfile. Tests compare accept/reject decisions across
RFC 3986 component classes and deterministic generated cases.

Profile differences are classified rather than silently adjusted: the SDK's
4,096-byte availability cap is stricter than RFC 3986 and the oracle; the SDK
accepts the RFC 3986 `IPvFuture` production that `uriparse` 0.6.4 rejects; the
oracle panics on the minimized malformed `1bad:value` rather than returning a
rejection; the SDK safely rejects it; the SDK does not normalize spelling; and
only acceptance of the RFC 3986 `URI` production is compared. Method, IRI,
scheme, dereferencing, and equivalence rules are not inferred from the oracle.

### Decision 5: make generative evidence deterministic in PR CI

Ordinary Rust integration tests use a fixed, documented deterministic
generator to produce URI component combinations, bounded extension trees,
document cardinalities, and duplicate-name mutations. Failures are reduced to
checked-in regression cases. This provides reproducible property evidence on
MSRV/mobile/WASM-compatible code without a nightly fuzz tool in the production
or PR dependency cone. Sanitizer/libFuzzer coverage for DID/DID URL lexical
types remains in #35.

An ignored release diagnostic reports hardened scan-and-parse throughput.
Resource/allocation evidence consists of the raw byte, node, depth, member,
and live-key ceilings plus the streaming/no-tree design; machine-specific
allocation counts and throughput thresholds are not CI pass criteria.

## Risks / Trade-offs

- **[Risk] valid unique-name input is parsed twice** → keep the scanner
  allocation-light, measure release throughput, and retain no hardware-specific
  threshold.
- **[Risk] scanner recursion becomes a stack risk** → reject container depth
  above 64 before descending further; fuzz/property tests exercise the edge.
- **[Risk] scanner and typed parser disagree on syntax** → both use the same
  pinned `serde_json` parser; preflight requires end-of-input and tests cover
  malformed/trailing forms.
- **[Risk] users assume generic `Deserialize` can detect already-collapsed
  duplicates** → document raw entry points as the security boundary and avoid
  making an impossible native-map guarantee.
- **[Risk] a differential oracle becomes production authority** → keep
  `uriparse` dev-only and record RFC 3986 plus explicit SDK policy as normative.
- **[Risk] stricter rejection breaks a pre-release caller relying on duplicate
  collapse** → record the intentional wire tightening; rollback is a focused
  revert and no released/stored format is migrated.

## Migration Plan

1. Land this issue-linked specification, ADR, threat review, and provenance
   record as a signed+DCO commit.
2. Add the scanner, redacted error mapping, integration/property/differential
   tests, and exact dev-only oracle pin.
3. Run focused coverage/performance, complete a distinct semantic/security/API
   review, and rerun the full workspace and Nix matrix.
4. Produce the receipt, sync `did-core`, archive the change, open a ready PR to
   `develop`, and merge only after all hosted gates are green.
5. Reuse the internal scanner from #41 and perform downstream adoption only in
   separately authorized repositories.

Rollback reverts this one PR. No package, consumer tree, persistent data, or
chain state changes in this slice.

## Open Questions

None. Standards precedence, scanner scope and limits, error behavior, oracle
role, compatibility tightening, and #35/#41 boundaries are resolved above.
