# Static-Model Fixtures

Use this directory for specification fixtures that can be checked without
network services or protocol peers.

Current fixtures:

- `capability-contracts.json`: first stable capability-first API contracts for
  issuer, holder, verifier, peer, wallet, mediator, DID, credential,
  presentation, DIDComm, OpenID4VC, trust/status, storage, and binding facade
  surfaces.
- `workspace-dependency-graph.json`: Cargo metadata-derived crate graph,
  dependency edges, hexagonal architecture layers, allowed dependency policy,
  policy violations, cycle violations, and boundary constraints.
- `typed-error-catalog.json`: stable core typed error codes, public messages,
  source variants, and binding targets for DID and DIDComm parser surfaces.
- `trust-status-policy.json`: Docker-free trust/status policy ports,
  mechanisms, typed errors, and first consumer crates.

Expected future layout:

```text
<spec-id>/
  valid/*.json
  invalid/*.json
  README.md
```
