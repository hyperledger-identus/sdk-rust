# OpenID4VP 1.0 Final wallet-core capability report

**SDK revision:** issue #541 candidate based on
`develop@63b23f7ebce3bab7741873b0147e7943a12ba73b`; exact merge revision pending

**Normative baseline:** [OpenID for Verifiable Presentations 1.0 Final](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html),
published 2025-07-09 with current errata retrieved 2026-10-28.

**Machine matrix:**
[`oid4vp-final-wallet-core.csv`](oid4vp-final-wallet-core.csv)

## Verdict

The bounded protocol core is coherent and executable, but IDR-024 is not yet
complete. Ten capability groups are implemented. Seven required rows retain a
bounded partial or unsupported surface and are owned by three focused issues:

- [#545](https://github.com/hyperledger-identus/sdk-rust/issues/545) — client
  identifier schemes and verifier metadata policy ports;
- [#546](https://github.com/hyperledger-identus/sdk-rust/issues/546) — remaining
  redirect response modes and response-code binding; and
- [#547](https://github.com/hyperledger-identus/sdk-rust/issues/547) — transaction
  data and credential-format metadata ports.

The matrix checker prevents IDR-024 from being marked delivered while these
required gaps remain. This is a useful milestone: completion is now an
executable condition rather than a subjective issue-count decision.

## Coverage summary

<!-- oid4vp-summary: implemented=10 partial=8 unsupported=4 not-applicable=3 required-gaps=7 -->

| Status | Rows | Meaning |
| --- | ---: | --- |
| Implemented | 10 | Bounded implementation, canonical contract, and executable tests exist. |
| Partial | 8 | A real executable subset exists with an exact limitation. |
| Unsupported | 4 | The current profile deliberately has no implementation. |
| Not applicable | 3 | The behavior belongs to an independently versioned capability. |

These are traceability labels rather than compliance scores. `implemented`
does not mean live transport, verifier trust, presentation authenticity,
consent, product readiness, downstream adoption, or official certification.

## Architecture and evidence flow

```text
OpenID4VP invocation
        │
        ▼
bounded Request URI envelope ── runtime adapter performs HTTP
        │
        ▼
signed Request Object ── JWS verification ── audience/time/replay evidence
        │                                      │
        │                                      ▼
        │                              DCQL validation
        │                                      │
        │                                      ▼
        │                             caller selects presentations
        │                                      │
        └──────────────────────────────────────┤
                                               ▼
                           direct_post / direct_post.jwt envelope

Matrix row ── implementation path ── canonical OpenSpec ── executable test
    │
    ├─ complete: implemented
    └─ bounded gap: partial/unsupported ── focused issue #544–#547
```

The protocol crate owns validated states and deterministic inert response
material. Runtime adapters own network effects. Product layers own trust,
consent, privacy, custody, and UI. Credential-format crates own proof creation
and verification.

## Section findings

- **Section 5:** static by-reference invocation, bounded GET/POST Request URI,
  JAR signature/type/client correlation, and the current request profile are
  executable. Discovery-bound audience, injected time policy and atomic replay
  acceptance are bound before typed routing; scheme-specific verifier authority
  remains separate.
- **Sections 6–7:** DCQL validation, bounded evaluation, credential-set rules,
  and generic JSON claims paths are implemented behind SDK-owned types.
- **Section 8:** request-bound `vp_token`, plain `direct_post`, explicit errors,
  and engine-neutral `direct_post.jwt` construction are implemented.
  Transaction data and format-specific VP validation remain incomplete.
- **Section 9:** the static `openid4vp:` by-reference invocation profile is
  implemented. Platform dispatch and navigation remain adapter effects.
- **Sections 10–12:** bounded wallet metadata can be carried during POST and
  encrypted-response recipient metadata is interpreted. General metadata,
  client-scheme binding, and verifier attestation remain outside this subset.
- **Sections 13–15:** selected static/direct-post implementation guidance and
  security controls are present. Replay storage and atomic decision policy stay
  caller-owned behind the SDK port; trust, consent, and privacy remain explicit
  product responsibilities.
- **Appendices A–C:** DC API, credential-format engines, and combined SIOPv2
  are separate capabilities and cannot be inferred from this HTTPS profile.

## Reference E2E and consumer-shaped canaries

The clean-room direct-post packet executes:

```text
invocation → retrieval binding → signed JAR verification
           → audience/time/atomic replay acceptance → DCQL validation
           → caller-selected presentation → exact request-bound form response
```

`final_capability_adapters.rs` adds two deliberately private test adapters.
One accepts a representative camelCase TypeScript shape and one accepts a
representative snake_case Swift shape. Both translate immediately into the
same public Rust invocation and retrieval types. They prove that thin adapters
are feasible without changing the Rust API or importing foreign error models.

No SDK-TS or SDK-Swift bytes are copied and no consumer repository is executed
or mutated. The canaries are not compatibility promises.

## Completion hierarchy

```text
#541 executable Final matrix and durable closeout owner
 ├── #545 client scheme and verifier metadata ports
 ├── #546 redirect response modes
 └── #547 transaction and format metadata ports
```

Issue #541 remains open until child delivery has reduced the pinned required
capability set to zero non-implemented rows. Changing that required set is a
review-visible contract change rather than a CSV relabelling shortcut.

## Assurance boundary

The checker proves schema closure, repository path existence, gap ownership,
and consistency with the backlog status. Rust tests prove the checked source
behaves as specified for synthetic inputs. Neither proves official OpenID
certification, live verifier interoperability, HTTP origin/TLS safety,
verifier or issuer trust, credential validity, presentation proof validity,
End-User consent, privacy compliance, or fitness for a wallet product.
