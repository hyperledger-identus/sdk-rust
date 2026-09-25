# Exact-diff architecture, security and provenance review

Review status: completed
Review date: 2026-09-25
Base: develop@149a35e62fc54f91dc99a496feb884d5b2f92cce
Reviewed implementation head: b76ff21b10e89b7006c8a005b1ab8c1424b32d59
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #376, ADR 0152,
the closed provenance manifest, all positive/negative payloads, their public
API execution, the conformance report/matrix and IDR-023 program closeout.
Oxid and Lace ID Portal were treated only as pinned, read-only design oracles.

## Findings

1. **Provenance and licensing — accepted.** Every executable byte is authored
   for this Apache-2.0 SDK from named Final sections, carries a SHA-256 digest
   and an independent transformation statement, and is rejected if the closed
   manifest drifts. No Portal byte is imported despite its manifest-level
   license claim; Oxid remains reference-only despite its usable repository
   license.
2. **Public behavior — accepted.** The positive journey crosses the existing
   public wallet APIs from offer transport through immediate Credential
   Response. The three negative vectors assert stable public error variants.
   No crate-private parser or consumer implementation becomes the oracle.
3. **Security and privacy — accepted.** Paths are relative, bounded,
   non-symlinked, regular and digest-bound; IDs and paths are unique; unknown
   manifest fields fail closed. Synthetic secrets and domains are used. Error
   assertions do not disclose caller or fixture content.
4. **Architecture and coupling — accepted.** This is test/documentation
   evidence only. Runtime sources, dependencies, manifests, features, public
   APIs and workflows do not change. Consumer repositories, network stacks,
   credential formats and chain semantics remain outside the generic crate.
5. **Claim boundary — accepted.** Matrix status means repository-executable
   generic wallet-core coverage. It does not mean current Portal compatibility,
   live issuer interoperability, certification, publication, production
   readiness or downstream adoption.
6. **Decomposition — accepted.** The 27-path/1,330-line diff is dominated by
   one strict test, its closed manifest and auditable evidence. Those artifacts
   must agree atomically; independent consumer/live/format-specific work is
   excluded rather than hidden inside this slice.

## Residual limitations

- Five partial and four unsupported Final rows remain deliberately visible.
- No live HTTP/TLS flow, external issuer, wallet UI or network is exercised.
- Consumer canaries and format-specific interoperability remain owned by their
  repositories and future issue-scoped deliveries.
- Hosted Linux `fast` evidence is still required on the exact pull-request
  candidate before merge.

## Review decision

The candidate provides a cohesive, reusable and license-safe generic
interoperability suite with deterministic drift controls and an honest claim
boundary. No unresolved correctness, security, privacy, architecture,
dependency, provenance or delivery finding remains.
