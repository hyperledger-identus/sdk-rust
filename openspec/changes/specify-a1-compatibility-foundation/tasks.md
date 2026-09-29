# Tasks

## 1. Specification and planning

- [x] 1.1 Create milestone 5, parent issue #504, sub-issue #505, and reuse
      #420, #422, and #501 as the four A1 capabilities.
- [x] 1.2 Encode GitHub sub-issue and blocked-by relationships; keep #492 as a
      downstream blocked consumer proof.
- [x] 1.3 Audit pinned official, SDK-Rust, SDK-TS, SDK-Swift, SDK-KMP, Apollo,
      and NeoPRISM evidence and classify its prospective authority.
- [x] 1.4 Specify the four schemas, cross-record references, DID seed packet,
      delivery order, completion evidence, limitations, and rollback.
- [ ] 1.5 Publish the human and machine implementation blueprint and validate
      the roadmap hierarchy offline.
- [ ] 1.6 Merge and archive this planning change through protected `develop`.

## 2. A1 implementation — separate child issues

- [ ] 2.1 Issue #420 implements the source/vector catalog, validator, mutation
      tests, and first DID packet.
- [ ] 2.2 Issue #505 implements canonical Rust-to-language DTO/error mappings,
      validator, mutation tests, and DID mapping seed.
- [ ] 2.3 Issue #422 implements the consumer change ledger and renderer after
      #420 and #505 IDs exist.
- [ ] 2.4 Issue #501 implements risk-routed property, fuzz, benchmark, and
      differential declarations and binds the DID seed.
- [ ] 2.5 Issue #504 validates the combined graph and records A1 completion.

## 3. Explicit stop boundary

- [ ] 3.1 Issue #492 may implement the SDK-TS DID canary only after all four A1
      child receipts exist.
- [ ] 3.2 Peer DID, SD-JWT, AnonCreds, DIDComm, agent runtime, browser/Node
      networking, and broader language adoption remain in later milestones.
