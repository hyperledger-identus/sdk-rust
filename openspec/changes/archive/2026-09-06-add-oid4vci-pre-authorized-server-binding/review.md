# Pre-implementation architecture, API, standards, and security review

- **Date:** 2026-09-07
- **Issue:** #121 under #7 / #20 / `IDR-023`
- **Develop base:** `04f5dc8bfe7df4f8089751af19e18fcf2f9410ae`
- **Result:** contract was implementable with no unresolved blocker

## Findings

1. Server choice is application policy. The SDK may validate one supplied
   candidate but must not infer priority from metadata order or silently fall
   back to another server.
2. The selected server must be in the issuer metadata's effective set. The
   omitted-list default is the Credential Issuer itself and must remain
   distinguishable from an advertised list.
3. The grant-level `authorization_server` hint constrains only the named grant.
   The Pre-Authorized transition must compare its own hint exactly and not use
   an Authorization Code hint as a substitute.
4. RFC 8414 defines omitted `grant_types_supported` as `authorization_code`
   plus `implicit`; omission is therefore negative evidence for this specific
   Pre-Authorized Code transition, not an open-ended capability claim.
5. The Token Endpoint is conditionally necessary for the selected exchange.
   Presence is required, but the partial metadata core cannot prove endpoint
   reachability, trust, client authentication support, or complete RFC 8414
   conformance.
6. The anonymous-access flag describes client identification/authentication
   behavior and must remain evidence rather than a binding allow/deny switch.
7. Moving the two predecessor states into one wrapper preserves bearer-adjacent
   ownership. Static fieldless errors and data-free Debug prevent disclosure.
8. All collections were bounded by predecessor parsers. Exact membership scans
   are linear over at most 16 servers and 32 grants and allocate no input data.
9. No HTTP, runtime, crypto, DID, storage, chain, product, manifest, feature, or
   dependency change is justified.
10. Consumer repositories are evidence-only. No production code or fixture is
    copied and downstream adoption remains separate.

Verdict: READY to implement after ADR 0046 and strict OpenSpec validation.

# Post-implementation exact-diff review

- **Date:** 2026-09-07
- **Reviewed production head:** `03bae391505cf5aa7a76316212d15ee7fee88c9b`
- **Exact diff:** `develop@04f5dc8b...03bae391`
- **Method:** fresh architecture/API/standards/security/resource review after
  focused, workspace, and full Nix gates
- **Result:** no unresolved finding

## Exact-diff findings

1. `try_with_pre_authorized_server` is consuming and returns the only public
   constructor path for `CredentialOfferWithPreAuthorizedServer`; every stated
   invariant is checked before construction.
2. Effective server membership correctly uses the exact issuer default only
   when the issuer metadata list was omitted. An advertised list is scanned
   without normalization, substitution, ordering policy, or fallback.
3. Only the Pre-Authorized Code grant's own optional hint constrains this
   transition. The prior metadata stage already validates that any hint is
   permitted only with multiple exact advertised servers.
4. Grant capability checks use the metadata core's effective values. An
   omitted list yields only the RFC defaults and fails; an explicit exact
   Pre-Authorized Code identifier succeeds independent of list position.
5. Token Endpoint presence is checked only after identifier and grant
   agreement. No endpoint string is copied, transformed, resolved, probed, or
   represented as trusted.
6. The success state exposes both owned predecessor states by reference. Its
   Debug surface is data-free; five new errors are fieldless and map to tested
   static codes/messages with no canary disclosure.
7. Nine new tests cover the effective single-server default, two multi-server
   paths, exact hints, every failure class, preserved state/accessors,
   anonymous-flag neutrality, static codes, and diagnostic canaries.
8. The production transition parses and allocates nothing. Its only loops are
   over previously bounded server and grant collections, and the normal
   dependency cone is unchanged.
9. ADR 0046, blueprint, canonical backlog, and OpenSpec agree that this is a
   server-binding proof, not Token Request construction or protocol-engine
   completion.
10. Focused/workspace Cargo and all 27 compatible Nix checks pass, including
    Rust 1.85, WASM, Android, iOS, strict lints/docs, supply-chain policy, and
    the 479-test principal suite. Consumer receipts match preflight exactly.

## Archive-stage correction

The guarded archive appended extra blank lines at the two canonical specs' ends
of file. They were removed before the archive commit; this was a
text-hygiene-only correction with no requirement or implementation change.

Verdict: READY for specification synchronization and pull-request review.
