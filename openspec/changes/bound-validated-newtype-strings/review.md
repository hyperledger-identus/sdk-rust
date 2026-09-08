# Distinct review

Review date: 2026-09-09
Reviewer role: security, compatibility and architecture review
Base revision: `8f2bfc3b888ec56656aad71a629b380a1c1d7c25`
Scope: issue #207 exact diff plus its specification and ADR

## Outcome

Approved with no unresolved findings.

## Security review

- Generated borrowed string parsing calls the validator on the original `&str`
  before the only `to_owned`; a pointer-identity test fails under the former
  order and passes under the new order.
- `DidMethod` checks empty, then its byte limit, then ASCII bytes. Oversized
  input cannot trigger an unbounded character traversal and length has explicit
  precedence over a later grammar error.
- Empty, grammar-invalid and oversized validator paths use fixed details.
  Focused tests cover local `Display`/`Debug`, serde error `Display`/`Debug` and
  the stable public bridge without rejection canaries.
- No unsafe, secret, FFI, native, allocation-counting hook or dependency was
  introduced. Outer serde/transport allocation remains disclosed.

## Compatibility review

- Accepted input, serialization, owned `TryFrom<String>`, registry and DID
  Registration behavior are unchanged.
- The public ceiling is derived from the existing DID ceiling and an exact
  2,048-byte DID with a 2,042-byte method passes Registration construction.
- Requiring validated `String` validators to accept `&str` is an intentional
  source break for hypothetical `fn(&String)` consumers. Repository search
  proves both current production users already accept `&str`; ADR 0093 records
  the pre-release migration.

## Architecture and dependency review

- The change stays in generic `identus-derive` and `identus-did`; no Midnight,
  Cardano, Oxid, transport or method policy moves upstream.
- The public constant belongs with the type and reuses `MAX_DID_BYTES` as its
  single budget authority.
- No library can replace the code-generation call order or the SDK budget
  decision; retaining the existing closed ASCII validator has the smallest
  cone and highest cohesion.

## Residual limitations

`SDK-LIM-007` remains effective because deserializers and transports can
allocate before invoking SDK validation, and other inherited boundaries remain
under audit in #168. No completion claim is made for that parent issue.
