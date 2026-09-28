# Local architecture and security review

- **Review date:** 2026-09-29
- **Reviewed code head:** `22a747d0779ce6a2c2019d84dee7b95f0992f38c`
- **Review scope:** exact implementation diff after planning head
  `628db4c5dfb0cc9237526b6a32b525cd95a58462`
- **Result:** passed after the findings below were resolved

## Resolved findings

- The first implementation concentrated structural validation in three
  functions. They were decomposed by invariant; the live code-health audit now
  reports neither oversized production modules nor OID4VP complexity signals.
- Claim-set alternatives initially checked only reference existence. Repeated
  identifiers inside an alternative now fail closed, matching the uniqueness
  treatment already applied to credential-set alternatives.
- The first resource test directly exercised only query bytes, work, and
  combination count. It now independently proves collection, string, path,
  credential-count, work, and combination ceilings, plus nonzero construction.

## Architecture and coupling

- `siros-dcql` remains a private implementation dependency of
  `identus-oid4vp`; public parameters, results, errors, paths, credential ports,
  and limits are SDK-owned.
- The facade delegates matching, holder-binding selection, credential-set
  evaluation, and combination enumeration to the engine. SDK code adds only
  boundary validation, bounded policy, redaction, and type adaptation required
  by the repository's contracts.
- The credential capability is runtime-neutral and object-independent. It
  requires only a stable identifier, exact format, holder-binding fact, and
  bounded claims-path resolution; it does not depend on a wallet store,
  transport, UI, chain, or credential concrete type.
- The protocol layer depends inward on existing JOSE/crypto verification and
  does not expand `identus-jose` into a general-purpose JOSE implementation.

## Security and protocol truth

- JSON is bounded recursively before candidate deserialization; all collection,
  string, path, candidate-count, work, and combination limits use checked or
  capped operations.
- Candidate-tolerated shapes that conflict with the OpenID4VP 1.0 Final core
  are rejected before the private engine: missing/nonempty metadata, unsafe or
  duplicate identifiers, empty collections, duplicate paths, broken
  references, invalid value types, and unsupported trusted-authority policy.
- Caller- and verifier-controlled values are absent from error messages and
  manual `Debug`; retained sensitive strings use zeroizing ownership.
- A validated DCQL value represents signature and structural evidence only.
  Format-specific metadata, complete Authorization Request policy, verifier
  trust, consent, credential authenticity, and presentation construction remain
  explicit non-claims.

## Supply chain and portability

The exact crate version, checksum, BSD-2-Clause license, dependency cone, and
RustSec result are recorded and mechanically checked. Workspace MSRV 1.89 and
primary WASM, iOS, and Android builds passed. A script guard prevents accidental
promotion of candidate types into public signatures.

No unresolved blocking local finding remains. Hosted exact-head review and the
protected fast gate remain the delivery authorities.
