# Exact-diff architecture, security and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@cea35dced8a99fa7fb2fc37c238c64189e0bce87
Implementation head: 0b1f1be3248a345114128e7abd78e4a64f021cee
Reviewed head: 0b1f1be3248a345114128e7abd78e4a64f021cee
Specification commit: 330e95d022bba50762cf1749bcc5d465d61be913
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #356, ADR 0143,
OID4VCI 1.0 Final sections 5.2 and 5.3, RFC 6749 sections 3.1, 4.1.2 and
4.1.2.1 plus Appendices A and B, RFC 9207 sections 2 and 3, RFC 9700 sections
2.1 and 2.1.1, the immutable preimplementation receipt, parser allocation and
resource bounds, static diagnostics, public compatibility and all focused and
workspace verification.

## Findings

1. **Correlation order and ownership — accepted.** The transition consumes one
   request and does not construct either protocol outcome until the complete
   bounded query has parsed and exact retained state has matched. Success keeps
   the request lineage for later exchange; a correlated error drops request
   secrets.
2. **Issuer identification — accepted.** Metadata stores the exact optional
   RFC 9207 support flag with an effective false default. Advertised support
   requires the returned issuer to equal the selected Authorization Server,
   including delegated-server cases. Omitted/false support forbids `iss`, and
   the closed evidence name does not claim mix-up protection.
3. **Wire and extension behavior — accepted.** The boundary accepts only an
   extracted query. Strict bounded form decoding precedes decoded-name
   duplicate detection; unique bounded extensions are validated and discarded.
   Full callback URI, fragment, form-post and JARM shapes are not accepted.
4. **Branch and grammar behavior — accepted.** Success and OAuth error branches
   are exclusive. Code uses non-empty RFC 6749 VSCHAR; error and optional
   description use non-empty NQSCHAR; the optional URI uses the OAuth character
   class plus `fluent_uri::UriRef`. Standard errors have a closed classification
   and extensions remain exact and untrusted.
5. **Resource safety and privacy — accepted.** Positive limits independently
   cover encoded bytes, count, decoded names, generic values and every retained
   role. Retained remote values are zeroizing; code and developer text require
   explicit accessors; Debug and all public errors are redacted and static.
6. **Cohesion, compatibility and coupling — accepted.** Shared OAuth grammar is
   private, token-error regressions pass, and the success lineage is boxed
   internally to avoid an oversized public outcome without exposing allocation
   mechanics. Seventeen errors append after the immutable prefix. No dependency,
   feature, lockfile, unsafe/native, storage, release or existing wire contract
   changes.

## Residual limitations

- Callback routing and transport authenticity remain adapter obligations.
- `NotAdvertised` records absence of RFC 9207 evidence and is not mix-up
  protection.
- Code exchange, HTTP/TLS, client authentication, DPoP, PAR/JAR/JARM,
  persistence, retries, trust and product policy are not provided.
- Bounded authorization-code token-request construction remains issue #358.

## Review decision

The implementation is a cohesive one-shot correlation boundary with exact
selected-server evidence, bounded hostile-input handling, redacted ownership
and no unjustified transport or trust authority. No unresolved correctness,
security, privacy, compatibility, architecture or delivery finding remains.
