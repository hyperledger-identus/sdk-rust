# Pre-implementation review

- **Issues:** #271 umbrella; #277 OID4VCI delivery
- **Exact base:** `6217384f85ff72003a7b94482bf7879f4587be02`
- **Scope:** ADR/OpenSpec semantics and staged exact-base golden candidate
- **Result:** PASS
- **Blocking findings:** none in the proposed contract; the golden/receipt
  workflow gate remains intentionally outstanding

## Semantic and architecture review

The private code/kind/message record is irreducible for current OID4VCI
diagnostics: four variants differ in kind while capability and local/public
messages are invariant. The six 12/21/39/34/36/29 groups are contiguous,
exhaustive, total 171, and follow existing protocol responsibilities. No group
contains more than 39 records.

The plan correctly treats this as review-locality maintenance. The current and
selected designs each have 171 behavioral decisions, one mapping site, and no
wildcard default. ADR 0119 makes no false deduplication or compression claim.
It reuses the successful private pattern without introducing a shared type,
crate, trait, public generator, or cross-domain release edge.

## Golden and compatibility review

Source inventory found 171 enum variants, 171 public constants, and 171 ordered
match arms. Kind totals are 167 `InvalidInput` and four `Unsupported`, with the
four exact unsupported variants recorded in research. All messages are static
and identical across local/public surfaces; both source results are empty.

The staged candidate has the existing 11-column schema, 171 ordered rows, 175
LF lines, 59,380 bytes, and SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
The exact-base OID4VCI tree is byte-identical to the source used to construct
the candidate except for its provenance revision substitution. The candidate
contains public static diagnostics only, not offers, tokens, credentials,
nonces, identifiers, URIs, causes, or secret data.

The plan preserves enum order/derives/non-exhaustive marker, all public
constant paths, `CAPABILITY`, const bridge, `From`, `Display`, `Error`, and
source behavior. It explicitly excludes typed wire models, Serde, protocol
logic, dependencies, features, lockfile, canonical specifications, issues #7
and #168, FFI/bindings, and consumers.

## Factory, security, and target review

The fourth binding extends the current hardened data-driven validator and
Git-receipt path rather than copying security-sensitive logic. Static redacted
records cannot retain runtime or secret input. All current WASM/Android/iOS Nix
target derivations already include `identus-oid4vci`; their eventual results
remain compile-only evidence.

The ADR/OpenSpec contract is semantically suitable for the planning-only
commit. Before any production edit, the exact candidate must be added to the
active change, independently cross-checked in repository state, the relevant
factory research/constraints/OpenSpec checks must pass, and the durable
preimplementation receipt must be committed. This outstanding sequence is an
expected workflow gate rather than an architecture blocker.
