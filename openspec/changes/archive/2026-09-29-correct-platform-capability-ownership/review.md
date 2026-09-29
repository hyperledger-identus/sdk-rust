# Exact-diff architecture and specification review

Review status: completed
Review date: 2026-09-30
Base: develop@d6bc213e757b51f8647521774778791396efb76a
Planning commit: fec348b7ef4d7bfa1ceefb2d578b1eca0e182a6c
Implementation head: 631ed4f59e08d6f5cca145d05cf81ff31a64452b
Reviewed head: 631ed4f59e08d6f5cca145d05cf81ff31a64452b
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, sponsor corrections in
issue #497, ADRs 0169–0172, the SDK-TS and cross-SDK inventories, the portable
runtime design, both roadmap representations, checker and mutation changes,
all routed child issues, and the OpenSpec proposal, research, constraints,
design, requirements, tasks, receipt, and archive intent.

## Findings

1. **Authority direction — accepted.** SDK-TS remains the first and most
   current discovery baseline without becoming the source of canonical DTOs,
   errors, package boundaries, or Rust architecture. SDK-Rust owns portable
   contracts; compatibility adapters are explicit, versioned, observable, and
   removable.
2. **Capability boundaries — accepted.** Generic DID/domain/service behavior,
   peer DID, portable Prism semantics, protocol state, and the bounded runtime
   kernel are Rust responsibilities. Cardano effects, product frameworks,
   custody, consent, concrete stores, deployment, and host I/O policy remain
   downstream or injected.
3. **Protocol versioning — accepted.** RFC 9901 is separated from the evolving
   SD-JWT VC profile and legacy media compatibility. DIDComm Messaging v2.1 is
   separated from independently versioned application protocols and their
   roles/state schemas.
4. **Roadmap granularity — accepted.** A0–A9 express dependency-ordered
   adoption proofs rather than repository-wide parity. Each milestone has a
   bounded deliverable, exact issue owner, exit evidence, adoption proof,
   explicit non-claims, and all four engineering evidence classes.
5. **Backlog cohesion — accepted.** Existing issues carry the corrected scope;
   only missing runtime, networking, protocol-catalog, quality-policy, and
   capability-migration issues were added. The next implementation milestone
   is A1 shared compatibility evidence, not a large protocol port.
6. **Regression controls — accepted.** The machine inventory rejects donor
   ownership, missing required capabilities, and weakened quality policy. The
   roadmap validator rejects missing fields, forward dependencies, reordered
   milestones, or incomplete evidence classes. Mutation tests exercise those
   failures.

## Residual limitations

- No runtime, DID method, credential format, DIDComm engine/protocol, network
  adapter, binding, consumer migration, or reference service is implemented.
- Candidate crates are research oracles until their owning issues complete
  supply-chain, dependency-cone, target, security, conformance, resource, and
  API-boundary qualification.
- SDK-Swift and SDK-KMP still require their planned capability-level
  inventories; neither can retroactively become normative authority.
- No release, support, certification, or service-deprecation claim follows
  from this planning change.

## Review decision

The planning package is cohesive, chain-neutral, dependency-directed, and
enforceable offline. It reflects the sponsor corrections without activating a
dependency or feature. No unresolved correctness, security, privacy,
compatibility, architecture, or delivery blocker remains.
