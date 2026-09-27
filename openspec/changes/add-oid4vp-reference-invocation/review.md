# Exact-diff architecture and security review

Review date: 2026-09-28
Issue: #394
Planning receipt commit: `f80f39946f7496d008e53a2832a1f6fb77ed8025`
Initial implementation commit: `84f2bf56b8b495a88ce382385d0c999a5725b228`
Corrective specification commit: `d03ab25dc373cce5d32db67def261b9168ca175b`
Reviewed implementation commit: `e66c8b2eede13a012e665d4de86a8384d286a17b`

## Resolved finding

### A-1 — Oxid route was mistaken for the Final static endpoint

Severity: blocking before correction

The initial parser accepted only `openid4vp://authorize`, copied as a shape
from the read-only Oxid design oracle. OpenID4VP 1.0 sections 9 and 13.1.2
instead define the static authorization endpoint as `openid4vp:`. Shipping the
initial shape would have encoded downstream product routing as generic Final
syntax.

Resolution: the specification, ADR, issue and blueprint now distinguish the
two shapes. The parser accepts `openid4vp:?…`, rejects any authority or path,
and has a negative regression vector for `openid4vp://authorize`. Oxid remains
unchanged and owns any product-route adaptation.

## Final architecture review

- The crate owns one cohesive wire ingress; generic presentation semantics
  remain in `identus-presentations` and product/session policy remains outside
  the SDK.
- The internal runtime dependency cone is exactly `identus-core`; the other
  direct families are the existing neutral `fluent-uri` parser and `zeroize`.
- No HTTP runtime, JSON/DCQL engine, chain, product, SIROS or consumer edge was
  introduced. No third-party type enters the public API.
- Source modules remain separated by responsibility: invocation parsing,
  resource policy, static errors and facade. No new architecture or
  decomposition blocker remains.

## Final security review

- Complete input, pair count, decoded names/values and retained fields have
  independent positive limits. Duplicate checks occur after strict decoding.
- The retained Request URI is HTTPS syntax only and confers no reachability,
  transport, signature, provenance or trust authority.
- Unknown values are bounded and dropped; recognized names cannot be shadowed.
  By-value/inline transport and unsupported transaction data fail closed.
- Retained verifier-controlled values are zeroizing. Public errors and all
  debug/display surfaces were checked with canaries and contain no input.
- Production source contains no `unsafe`, panic, unwrap, transport, filesystem
  or environment access. The workspace-level unsafe prohibition applies.

Final disposition: no unresolved architecture or security finding.
