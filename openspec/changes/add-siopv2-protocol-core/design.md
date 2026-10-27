# Design

## Component boundary

```text
protocol-specific invocation
          │
          ▼
 identus-openid4vc
 bounded request retrieval ───── caller HTTP adapter
          │ response-bound bytes
          ▼
 identus-siopv2
 request JAR verification ────── caller-authorized RP key
          │
          ├─ validate draft-13 DID/direct_post profile
          ├─ create replay fingerprint
          └─ start deterministic flow
                     │ approval(subject DID + DID URL method)
                     ▼
                identus-jose
             bounded signing input ───── injected signer/custody
                     │ signed compact ID Token
                     ▼
           exact id_token/state form plan ───── caller transport
                     │
                     ├─ delivered
                     └─ outcome unknown
```

Dependency direction is
`identus-siopv2 -> identus-openid4vc + identus-jose + identus-did -> core/crypto`.
No edge points to OID4VP or a product.

## Shared request retrieval

Move the neutral retrieval request/input/result types and media/status binding
from OID4VP to `identus-openid4vc`. OID4VP re-exports its existing names and
delegates behavior so current callers do not change. SIOPv2 builds its own
invocation/request policy on the same finite retrieval envelope. The shared
crate does not parse protocol claims or perform HTTP.

## SIOPv2 request stages

1. Parse a bounded by-reference invocation and retain only outer client ID,
   request URI, method and optional caller nonce evidence.
2. Bind retrieved status/media/body to that exact request.
3. Parse the compact request object, require the selected protected type and
   expose only bounded header evidence for caller-owned key authorization.
4. Verify through `SignatureSuiteRegistry` and a bound
   `JwsVerificationKey`.
5. Parse duplicate-free bounded claims and validate exact client/issuer,
   audience, time, nonce, redirect, response type/mode, ID token type, scope,
   algorithm and DID subject-syntax metadata.
6. Create a validated request plus SHA-256 replay fingerprint; no request value
   is trusted before stage 4.

## Response and signer boundary

Approval supplies a typed DID subject, DID URL authentication method, issued
time and lifetime within configured limits. The crate verifies the method
belongs to the subject syntactically, constructs claims containing only
`iss`, `sub`, `aud`, `nonce`, `iat`, and `exp`, and constructs a protected JWS
header with exact algorithm and `kid`. `identus-jose` produces and signs the
bounded input. The crate then emits an exact bounded form containing
`id_token` and request `state` when present.

The signer knows no SIOP claims. The SIOP crate knows no private keys. A caller
must separately prove that the chosen method is controlled by and authorized
for authentication in the current DID document.

## Deterministic lifecycle

The pure flow states are `awaiting_decision`, `response_ready`, `refused`,
`canceled`, `timed_out`, `delivered`, and `outcome_unknown`. Events carry a
bounded opaque event ID. The flow retains only a finite receipt window and
returns the prior result for an identical event ID; reusing an ID with a
different event fails closed. Terminal events are idempotent only for their
original identity.

Approval returns a one-shot response-building capability. Refusal,
cancellation and timeout return no signing effect. Delivery acknowledgement
consumes the response. A timeout after response creation becomes
`outcome_unknown`, not `timed_out`, because the RP may have accepted the
response. Automatic resubmission is prohibited.

## Evidence and migration

Clean-room vectors assert draft-13 wire semantics. Attributed oracle fixtures
normalize Impierce, Affinidi and Oxid behavior to claims and result categories.
The E2E test uses SDK software signers and independent SDK verification over
the exact form-decoded token. The Oxid guide maps its prepare/accept/refuse
aggregate and standalone adapter onto the SDK flow while retaining product
consent, loopback runtime, managed DID/custody and activity projections.
