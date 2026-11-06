# OpenID4VP 1.0 Final wallet-core capability report

**SDK revision:** issue #541 candidate based on
`develop@63b23f7ebce3bab7741873b0147e7943a12ba73b`; exact merge revision pending

**Normative baseline:** [OpenID for Verifiable Presentations 1.0 Final](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html),
published 2025-07-09 with current errata retrieved 2026-10-28.

**Machine matrix:**
[`oid4vp-final-wallet-core.csv`](oid4vp-final-wallet-core.csv)

## Verdict

The bounded protocol core is coherent and executable. Eighteen capability
groups are implemented and the pinned required set has zero non-implemented
rows. The reviewed matrix therefore supports IDR-024 delivery. Official OpenID
certification, publication, live interoperability and downstream adoption
remain separate gates.

Issue [#547](https://github.com/hyperledger-identus/sdk-rust/issues/547)
delivers exact transaction-data lineage and credential-format metadata ports
without moving SD-JWT VC, mdoc, or W3C proof engines into this crate. Completion
is an executable matrix condition rather than an issue-count decision.

## Coverage summary

<!-- oid4vp-summary: implemented=18 partial=3 unsupported=1 not-applicable=3 required-gaps=0 -->

| Status | Rows | Meaning |
| --- | ---: | --- |
| Implemented | 18 | Bounded implementation, canonical contract, and executable tests exist. |
| Partial | 3 | A real executable adjacent subset exists with an exact limitation. |
| Unsupported | 1 | The product-owned privacy surface deliberately has no protocol-core implementation. |
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
        │                         exact verifier/key policy
        │                                      │
        │                                      ▼
        │                              DCQL validation
        │                                      │
        │                                      ▼
        │                             caller selects presentations
        │                                      │
        └──────────────────────────────────────┤
                                               ▼
                redirect modes / direct_post(.jwt) inert material
                                               │
                                               ▼
                       optional bound direct-post acknowledgement

Matrix row ── implementation path ── canonical OpenSpec ── executable test
    │
    ├─ complete: implemented
    └─ adjacent/separate limitation: explicit, outside required completion set
```

The protocol crate owns validated states and deterministic inert response
material. Runtime adapters own network effects. Product layers own trust,
consent, privacy, custody, and UI. Credential-format crates own proof creation
and verification.

## Section findings

- **Section 5:** static by-reference invocation, bounded GET/POST Request URI,
  JAR signature/type/client correlation, and the current request profile are
  executable. Discovery-bound audience, injected time policy and atomic replay
  acceptance and scheme-specific exact verifier/key policy are bound before
  typed routing.
- **Sections 6–7:** DCQL validation, bounded evaluation, credential-set rules,
  and generic JSON claims paths are implemented behind SDK-owned types.
- **Section 8:** request-bound `vp_token`, fragment/query/form-post, plain
  `direct_post`, explicit errors, engine-neutral `direct_post.jwt`, and bounded
  acknowledgement binding are implemented.
  Transaction data and format-specific candidate and selected-result linkage
  are implemented through bounded ports; proof engines remain format-owned.
- **Section 9:** the static `openid4vp:` by-reference invocation profile is
  implemented. Platform dispatch and navigation remain adapter effects.
- **Sections 10–12:** bounded wallet metadata can be carried during POST;
  verifier metadata, verifier-info and all Final client schemes are classified
  behind exact-key trust ports. Concrete PKI, DID resolution, Federation and
  attestation engines remain adapter responsibilities.
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
           → audience/time/atomic replay acceptance → verifier/key policy
           → DCQL validation
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
 ├── #545 client scheme and verifier metadata ports (delivered by this slice)
 ├── #546 redirect response modes (delivered by this slice)
 └── #547 transaction and format metadata ports (delivered by this slice)
```

Issue #541 may close after this reviewed child merges because the pinned
required capability set is now zero non-implemented rows. Changing that set is
a review-visible contract change rather than a CSV relabelling shortcut.

## Assurance boundary

The checker proves schema closure, repository path existence, gap ownership,
and consistency with the backlog status. Rust tests prove the checked source
behaves as specified for synthetic inputs. Neither proves official OpenID
certification, live verifier interoperability, HTTP origin/TLS safety,
verifier or issuer trust, credential validity, presentation proof validity,
End-User consent, privacy compliance, or fitness for a wallet product.
