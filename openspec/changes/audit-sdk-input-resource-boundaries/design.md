## Context

Resource ownership crosses typed values, primitive operations, protocol
semantics, transports and bindings. A single boolean “bounded” flag would hide
whether the SDK checks before allocation, retains input, or delegates work to a
caller-owned adapter.

## Goals and non-goals

The design must make every implemented runtime package visible, prove every
distinct input family has an ownership disposition, fix avoidable inherited
BIP-39 work, and retain exact residual limitations. It does not invent budgets
for payloads whose protocol/adapter owner is outside the crate.

## Decisions

### One governed inventory, grouped by input family

`docs/architecture/sdk-input-resource-boundaries.toml` is normative. Package
coverage is derived from the existing bootstrap inventory rather than copied as
another package classification. Boundary rows may group public items only when
they share the same owner, limit mechanism, retained shape and evidence.

Allowed dispositions are:

- `sdk-enforced`: typed SDK code enforces declared limits;
- `fixed-or-no-input`: fixed-size/numeric/static values or no caller input;
- `caller-budgeted-work`: borrowed input or a generic port whose byte/CPU/time
  budget belongs to its caller/adapter;
- `outer-preallocation`: SDK semantic limits exist, but another runtime may
  allocate or scan before the typed check.

Rows can overlap a package because an HTTP or binding surface may be both
semantically bounded and subject to outer preallocation. Each row declares one
primary disposition and describes related residuals explicitly.

### Fail closed on inventory structure, not inferred source semantics

The checker verifies exact implemented-package coverage, known dispositions,
unique stable IDs, non-empty ownership/impact/review fields, limit presence for
SDK-enforced rows, repository-local evidence existence, file/count/size bounds,
and absence of placeholder or verification-only claims. It does not pretend a
regex can prove every Rust API. Semantic completeness remains a security and
architecture review responsibility; issue #275's future `syn` scanner remains
separate code-health work.

### Bound BIP-39 before expensive or allocating work

The helper checks entropy length before dependency entry; mnemonic count and
each word byte length before `join`; and passphrase bytes before NFKD or PBKDF2.
It reuses `Error::MnemonicInvalid`, preventing secret-bearing diagnostics and a
new public error surface. The 4,096-byte passphrase ceiling aligns with existing
crypto text input policy. The English word ceiling is derived from the adopted
wordlist and locked by a test.

## Risks and mitigations

- Aggregated rows can obscure a future API: package/source review triggers and
  factory coverage prevent silent package omission; reviewers must update the
  row when an input family changes.
- An inventory can drift from code: exact evidence paths, mutation tests, and
  focused boundary tests turn structural drift into a failing gate.
- The passphrase ceiling rejects a previously accepted extreme input: the API
  is experimental/unpublished, the bound is explicit, and valid test vectors
  remain exact.
- Consumers may mistake typed checks for transport safety: the residual
  limitation and every outer-boundary row state preallocation ownership.

## Verification

Focused tests cover exact/one-over mnemonic word count, word bytes, passphrase
bytes, entropy lengths, redaction, and KMP parity. Inventory mutations exercise
every structural invariant. Factory/OpenSpec, format, strict Clippy, all tests,
builds and compatible Nix validate integration.
