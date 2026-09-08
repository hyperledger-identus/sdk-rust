# DID parser parity results

Run date: 2026-09-08
SDK baseline: `develop@9f0c2d7215d64dff6c3eee9844c5e049807fa0b4`
Candidate: `did_url_parser 0.3.0`
Candidate release commit: `cdde0daface0e24fef88c53d46f5bf6129780f02`

Consumer revisions are embedded per corpus row. The Midnight fixtures come
from `midnight-did@42a8e4aca1c6043f7f2b73463fa3aa3cd3d48b06`, the gitlink recorded by
`midnight-identity@427f8571950c42967a18726cbcbefecc19ef8d79`.

## Provenance and maintenance snapshot

- Crates.io artifact SHA-256:
  `b83822347b20ece984662821ac650ac5bcb3602d83ed83ab992b7f8dda388bdb`.
- The five packaged `src/*.rs` hashes equal the pinned commit hashes.
- `v0.3.0` is a lightweight tag pointing to the SSH-signed release commit;
  the local environment had no allowed-signers map to verify signer identity.
- Crate license: MIT OR Apache-2.0. Repository metadata reports Apache-2.0.
- No `rust-version`/MSRV is declared. The repository is unarchived; its latest
  push and crates.io release were 2025-01-06. One open issue requests operation
  without `alloc` and had no update after 2024-05-27 at retrieval time.
- Crates.io reported 190,851 lifetime and 35,873 recent downloads. Download
  count is adoption context, not correctness or maintenance evidence.

## Dependency and source shape

With defaults disabled and only `alloc`, the standalone normal graph contains:

```text
did_url_parser 0.3.0
form_urlencoded 1.2.2
percent-encoding 2.3.2
```

All three names are absent from the SDK production lock at the baseline. The
candidate has no native-code package. Its 1,042 Rust source lines include
mutable setters, form query parsing and relative resolution; the consumed SDK
parser/facade is 455 lines in one file. The isolated comparison harness has its
own 26-package lock because it links both implementations and is not a release
workspace member.

## Behavioral comparison

The committed corpus has 30 attributable cases: 23 agree and 7 differ. Common
W3C, SDK fuzz, NeoPRISM, Midnight, Lace and Oxid values agree. Mismatches are:

| Class | Curated evidence | Candidate behavior |
| --- | ---: | --- |
| DID grammar | 1 | accepts a terminal-colon method-specific identifier |
| percent grammar | 1 | accepts `%+0` as if `+0` were two HEXDIG characters |
| trimmed parse/exact storage | 3 | accepts surrounding controls/space, stores them, and derives offsets from the trimmed view |
| SDK resource limits | 2 | accepts one byte beyond both the 2 KiB DID and 4 KiB DID-URL ceilings |

The deterministic generated comparison evaluates 17,284 cases and reports 95
mismatches:

```text
leading-byte       34
trailing-byte      35
percent-pair       22
terminal-colon      4
```

The 34 boundary bytes are ASCII control bytes plus space. The extra trailing
case is `:`. All 22 percent mismatches are `%+` followed by a hexadecimal digit;
Rust integer parsing accepts the plus sign, but RFC 3986's production does not.

Accepted common cases have identical exact strings and method, method-specific
identifier, path, query and fragment slices. In contrast, a leading-space
candidate value stores the leading space but indexes the trimmed view, exposing
shifted slices. This is an internally inconsistent validated value, not merely
a difference in error categorization.

## Facade, allocation and resource behavior

- SDK `Did::try_from(String)` retained the input pointer; candidate
  `DID::try_from(String)` did not.
- Candidate setters produced `did:INVALID:not/absolute`, which the candidate's
  own parser rejects. The SDK values expose no invariant-breaking mutation.
- Candidate size on the diagnostic host was 56 bytes, versus 32 for SDK `Did`
  and 72 for SDK `DidUrl`. Type size does not offset the allocation/copy and
  semantic incompatibilities.
- Candidate parsing allocates its owned string before syntax validation and
  exposes no pre-allocation limit or borrowed parser result. A private SDK
  wrapper would need the existing precheck plus either retain the candidate
  object or copy into the SDK representation.
- Candidate relative joining works for the exercised RFC-shaped cases but
  reaches one `unsafe { from_utf8_unchecked(...) }` source path. The SDK parser
  has no relative-join consumer surface, so this code is cost rather than reuse.

One optional release-mode host diagnostic over 500,000 iterations measured:

| Input | SDK | Candidate |
| --- | ---: | ---: |
| valid DID URL | 59.45 ms | 58.66 ms |
| invalid DID URL | 27.38 ms | 42.42 ms |

These timings are machine-specific and are neither promises nor primary
decision criteria. The candidate's small valid-input difference cannot repair
the accepted-language and facade failures.

## Compiler, targets, lint and tests

- Rust/cargo: 1.98.1.
- Candidate `cargo test`: 19 passed. Its property test runs 1,024 positive DID
  cases; source TODOs explicitly omit path/query/fragment properties.
- Candidate `--no-default-features --features alloc`: passed.
- The same candidate surface passed `wasm32-unknown-unknown`,
  `aarch64-linux-android` and `aarch64-apple-ios` compile checks.
- Candidate strict library Clippy with `-D warnings` failed on three
  `mismatched_lifetime_syntaxes` findings introduced by the Rust 1.98 lint set.
- The repository harness itself passes format and strict `--no-deps` Clippy.
- Existing SDK evidence records one million fuzz units for each DID lexical
  target without invariant failure; this decision adds no production code.

## Supply-chain result

A minimal consumer lock containing only the candidate's normal graph passed
the current Nix-pinned `cargo audit --deny warnings` after loading 1,242
advisories. Auditing the candidate's published development lock failed because
its old `proptest 0.10.1` graph reaches `rand 0.7.3`, flagged by
RUSTSEC-2026-0097. That finding is development-only but means the upstream
source-test lock is not currently warning-clean.

## Decision

Retain the current parser. The candidate fails independent grammar,
exact-storage, resource, allocation, immutable-facade and unsafe-reach stop
conditions. Wrapping it would not remove the existing mechanics. ADR 0086
records the reconsideration trigger for a corrected future release.

## Exact replay commands

From the repository root:

```sh
cargo run --manifest-path docs/research/rust-library-reuse/did-url-parser-parity/Cargo.toml --locked --release
DID_PARSER_BENCH=1 cargo run --manifest-path docs/research/rust-library-reuse/did-url-parser-parity/Cargo.toml --locked --release
cargo clippy --manifest-path docs/research/rust-library-reuse/did-url-parser-parity/Cargo.toml --locked --no-deps -- -D warnings
```

Candidate source checks used the unpacked crates.io artifact and the exact
release checkout. Sanitizers, Miri, Windows and a historical compiler matrix
were intentionally unrun: no production adoption survives the semantic stop
conditions, and Rust 1.98.1 is the active SDK etalon.
