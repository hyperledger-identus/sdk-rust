# Exact-diff architecture, security and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@1b211e9246808e3883fa5fa3ac01a40b36e89076
Implementation head: c876ae7f6be728877b987554cc8a25efb03ff1a2
Reviewed head: c876ae7f6be728877b987554cc8a25efb03ff1a2
Specification commit: 63396de4f84fdc9795629dc9c01a3aeb1b42ac95
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #354, ADR 0142,
OID4VCI 1.0 Final sections 5.1, 5.1.1 and 5.1.3, RFC 6749 sections 3.1 and
4.1.1 plus Appendix B, RFC 7636 section 4.3, RFC 9396, RFC 9700 section
2.1.1, the immutable preimplementation receipt, form-codec factoring,
allocation/resource bounds, diagnostic contracts, public compatibility and
all focused/workspace verification.

## Findings

1. **Protocol behavior — accepted.** The transition emits one deterministic
   GET target with exact OAuth/PKCE fields, one OID4VCI
   `authorization_details` object and optional offered `issuer_state` in a
   fixed order. `locations` is present exactly for explicitly advertised
   Authorization Servers. Scope and resource are deliberately absent.
2. **Endpoint query handling — accepted.** Existing query bytes are retained
   only after bounded strict form decoding. Empty/malformed fields, decoded
   duplicates, all managed names and alternate scope/resource/request intent
   collide before construction. The validated endpoint cannot be replaced by
   the caller in this state transition.
3. **Least authority and cohesion — accepted.** The output retains the exact
   #352 predecessor and a sensitive URI but does not open a browser, perform
   PAR/HTTP, accept a callback or exchange a code. Shared form mechanics are a
   private module used by the two wire builders and the existing parser.
4. **Resource safety — accepted.** Positive caller-visible limits bound
   Authorization Details, retained endpoint query count and decoded
   components, and the final URI. JSON writes stop at the configured ceiling;
   arithmetic is checked before allocation.
5. **Security and privacy — accepted.** The request URI and inherited
   state/verifier are zeroizing and redacted from Debug. Static errors contain
   no endpoint, state, issuer state, identifiers or encoded request content.
   Exact response-state and issuer correlation remains impossible to claim
   until successor #356.
6. **Compatibility and coupling — accepted.** Public behavior is additive.
   Seven fieldless errors/codes append after the immutable prefix. Existing
   offer decoding and Pre-Authorized Token Request bytes retain regression
   coverage. No dependency, feature, lockfile, unsafe/native, target, storage
   or release contract changes.

## Residual limitations

- Authorization Response parsing and exact state/issuer correlation remain
  issue #356.
- PAR, JAR/JARM, browser/redirect adapters, HTTP/TLS, code exchange, client
  authentication, DPoP and token handling are not provided.
- Registration, trust, authorization success, persistence, product policy and
  Midnight/Cardano/PRISM behavior remain outside this generic primitive.

## Review decision

The implementation is a cohesive bounded wire transition with deterministic
bytes, explicit intent and no unjustified authority or dependency. No
unresolved correctness, security, privacy, compatibility, architecture or
delivery finding remains.
