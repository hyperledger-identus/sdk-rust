# oid4vci-proof-jwt Specification Delta

## MODIFIED Requirements

### Requirement: Final-profile holder proof construction

The OID4VCI capability SHALL provide a holder-side builder for the
`openid4vci-proof+jwt` proof defined by OpenID4VCI 1.0 Final Appendix F.1. It
SHALL compose protocol-neutral compact and signature mechanics through the
JOSE capability, require one accepted asymmetric algorithm, exactly one key
reference, one non-empty bounded Credential Issuer audience and an explicit
integer issuance time. The emitted protected `typ` SHALL equal
`openid4vci-proof+jwt`.

#### Scenario: identified holder input is canonical

- **WHEN** an identified client prepares a proof with a DID URL `kid`, audience,
  issuance time and server nonce
- **THEN** the protected header and claims SHALL contain exactly the selected
  algorithm, proof type, `kid`, `iss`, `aud`, `iat` and `nonce` values and the
  builder SHALL expose the exact bytes supplied to the signer

#### Scenario: anonymous pre-authorized input omits issuer

- **WHEN** an anonymous pre-authorized client prepares an otherwise valid proof
- **THEN** the claims SHALL omit `iss` by construction rather than serialize a
  null, empty or inferred client identifier

### Requirement: Bounded Final-profile issuer parsing

The OID4VCI capability SHALL parse the OpenID4VCI 1.0 Final
`openid4vci-proof+jwt` issuer profile through the existing `JwsLimits` and
`Oid4vciProofJwtLimits`. It SHALL compose protocol-neutral compact parsing
through the JOSE capability and require exact protected `typ`, one accepted
asymmetric algorithm, exactly one `kid`, public `jwk`, or `x5c` reference, one
non-empty bounded string `aud`, one integer `iat`, and optional bounded string
`iss` and `nonce`. Known claim duplicates, wrong types, missing required
claims, private JWK material and invalid references SHALL fail through static
errors. Unknown claims MAY be skipped but SHALL remain bounded by the complete
payload ceiling and SHALL NOT become trusted public state.

Parsing SHALL produce an explicit profile-parsed but signature-unverified
state and SHALL invoke no resolver, certificate, signature, clock, or replay
provider.

#### Scenario: Final proof shape becomes parsed evidence only

- **WHEN** a bounded compact JWT contains the exact proof type, one key
  reference, required audience and integer issuance time plus optional issuer,
  nonce and unrelated extension claims
- **THEN** parsing SHALL retain the exact compact signing input and recognized
  claims in a parsed state without asserting signature or policy acceptance

#### Scenario: malformed profile fails before providers

- **WHEN** the JWT has a wrong or absent type, no key reference, duplicate or
  malformed recognized claims, missing audience or issuance time, or an
  unsupported algorithm spelling
- **THEN** parsing SHALL fail and recording DID, certificate, signature, clock
  and replay providers SHALL observe zero calls
