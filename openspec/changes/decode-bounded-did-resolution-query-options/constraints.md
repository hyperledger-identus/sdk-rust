# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/203
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` continue to apply: query transport remains
private to the outer adapter and DID Core stays independent of Axum.
`SDK-SEC-003` applies to attacker-controlled query input and is made concrete
by byte/member/scalar ceilings plus fail-closed parsing. `SDK-DELIVERY-001` is
satisfied by child issue #203 under #10 and this specification-first change.
`SDK-LIM-003`, `SDK-LIM-005`, `SDK-LIM-006`, `SDK-LIM-007` and `SDK-LIM-009`
preserve draft, host-only, unpublished, adoption and certification limits.

## Introduced or changed constraints

- Raw query input is at most 8 KiB, contains at most 32 parameters, and each
  decoded name/value is at most 256/4,096 bytes.
- `accept` is header-only. The query supports exact common field names and
  string-valued extensions without type inference.
- Parsing splits raw structure before strict single-pass percent decoding,
  preserves literal plus and rejects malformed UTF-8/control input.
- Duplicate decoded names, missing separators, invalid typed values and
  simultaneous `versionId`/`versionTime` fail before resolver invocation.
- All query failures reuse the redacted W3C `invalidOptions`/400 response.

No repository-wide Rust, target, dependency, publication or release constraint
changes.

## Introduced or changed limitations

- HTTP query extensions are strings only; structured JSON extension values
  remain available through local/POST-style typed APIs, not GET.
- Method-specific extension semantics are enforced by the resolver, after the
  transport's generic name/value/resource validation.
- Query normalization is local to option projection; the adapter exposes no
  canonical query string and performs no forwarding.
- The component still omits DID URL dereferencing, POST, OpenAPI and deployment
  controls.

## Consumer and product impact

Hosts gain common and method-specific GET resolution options through the
existing router. Resolver implementations receive only validated SDK values.
No current downstream adoption or portable/release support is claimed.

## Activation and rollback

The decision activates only after issue #203's PR passes local and hosted gates
and merges into `develop`. Revert removes the decoder/spec delta and restores
blanket query rejection; no stored representation or migration exists.

## Evidence

Issue #203 records scope and authority. The research record pins the W3C and
repository revisions, candidate dispositions, compatibility facade, exact
resource limits, security behavior, unchanged dependency/target posture,
verification plan, stop conditions and rollback.
