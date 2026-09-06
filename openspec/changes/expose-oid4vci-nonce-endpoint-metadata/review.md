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

## Candidate reviewed

- Base: `f9771a16707889bf516d1a23a25603ed95ed207b`.
- Specification commit: `9eb7b503477c0ae3dab10443600ffa40b1f0590e`.
- Production implementation commit:
  `147d437e3da63e233b991d867a20f3a41c4b8634`.
- Exact diff:
  `origin/develop...147d437e3da63e233b991d867a20f3a41c4b8634`.

## Review findings

1. **No blocker — the optional member stays inside the established parser.**
   `nonce_endpoint` uses the same decoded-name uniqueness, aggregate-node,
   non-empty string and raw-document bounds as every existing known metadata
   member; unknown extensions remain retained in the original bounded JSON.
2. **No blocker — the shared byte policy is independent.** The parser applies
   `max_credential_endpoint_bytes` separately to each endpoint string. An
   accepted Credential Endpoint consumes no part of the Nonce Endpoint budget,
   and the public constructor and default values are unchanged.
3. **No blocker — URL semantics match the reviewed contract.** Both endpoint
   types reuse the HTTPS-with-host validator; ports, paths and queries remain
   exact while userinfo and fragments fail closed.
4. **No blocker — absence is explicit and transport-free.** The metadata owns
   `Option<NonceEndpoint>` and exposes only a borrow. No fallback, request,
   network, trust, nonce lifecycle, proof or credential-flow behavior enters
   the type state.
5. **No blocker — remote content is absent from diagnostics.** Endpoint Debug
   is redacted, metadata Debug exposes only an advertised boolean, and both new
   fieldless errors bridge to static tested codes and messages.
6. **No blocker — compatibility boundaries are unchanged.** No manifest,
   lockfile, dependency, feature, unsafe, FFI, target, chain, consumer or
   product file changes. The runtime cone remains `identus-core`, `serde_json`,
   `uriparse` and `zeroize`.

## Decision

The exact production diff is minimal, contract-complete and ready for the
recorded reproducibility and archive gates. No unresolved local finding
remains.
