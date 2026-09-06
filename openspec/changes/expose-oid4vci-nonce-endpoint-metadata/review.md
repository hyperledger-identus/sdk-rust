# Semantic contract review

## Scope reviewed

- Issue #133 and exact base
  `f9771a16707889bf516d1a23a25603ed95ed207b`.
- OpenID4VCI 1.0 Final sections 7 and 12.2.4.
- Existing issuer metadata limits, strict scanner, HTTPS endpoint validator,
  static errors, redaction, crate boundary and target policy.
- Read-only Oxid and Lace evidence and license posture listed in the issue and
  design.

## Findings

1. **Resolved — omission has protocol meaning but is not an error.** The public
   accessor returns `None` and documents the Final statement that an omitting
   issuer does not require `c_nonce`; it does not invent endpoint fallback.
2. **Resolved — the exact permitted URL shape is preserved.** The existing
   validator requires HTTPS and host while admitting port, path and query. It
   rejects userinfo and fragments, so no weaker second URL policy is added.
3. **Resolved — one shared endpoint budget avoids API breakage.** The existing
   endpoint limit applies independently to each Credential or Nonce Endpoint.
   Documentation changes, but the ten-argument constructor and defaults do not.
4. **Resolved — endpoint discovery is not transport authority.** The value
   proves bounded metadata syntax only; it makes no reachability, trust, DNS,
   redirect, private-network or HTTP-execution claim.
5. **Resolved — diagnostics remain content-free.** New oversize and unsafe
   variants are fieldless, endpoint Debug is redacted, and errors never expose
   the URL, JSON, parser cause or offset.
6. **Resolved — strict extension behavior is preserved.** The known optional
   member participates in the existing node, duplicate, type and size checks;
   all other bounded unknown members remain losslessly retained as raw JSON.
7. **Resolved — private unlicensed evidence is not imported.** Lace is used
   only as an independent behavior reference. No Lace source or fixture is
   copied, transformed, vendored or linked.

## Decision

The proposal, design, capability requirements, program replacement and task
map are semantically complete, objectively testable, reversible and within the
standing mandate. No correctness, security, privacy, compatibility,
provenance, target or product-scope blocker remains before implementation.

# Exact-diff implementation review

Pending implementation.
