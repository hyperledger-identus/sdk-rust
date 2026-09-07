# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

ADR 0061 already rejects whole-framework convergence and permits narrow private
engines behind Identus-owned facades. Its report covers several crates and six
of the repositories in this change, but it does not give every repository a
standalone decision. Procivis ONE Core, DIF `did-key.rs`, AnonCreds v1/v2,
VCX, and DIDKit have no repository-specific ADR.

No production dependency is added here. The current implementation and
their correctness gaps remain owned by their existing component issues.
Consumer evidence comes from the roadmap needs of Midnight Identity,
NeoPRISM, Lace ID Portal, and Oxid; those repositories remain read-only.

## Normative sources

The relevant standards remain the exact versions selected by each component
row: DID Core 1.0, VCDM 1.1/2.0, RFC 9901, the pinned SD-JWT VC profile,
OID4VCI/OID4VP 1.0, SIOPv2, DIDComm 2.1, ISO 18013 profiles, and an
issue-pinned AnonCreds profile. Repository claims are checked against upstream
manifests, READMEs, workflows, releases, and default-branch state at immutable
revisions recorded in the portfolio report.

Primary source URL examples include the
[Spruce SSI manifest](https://github.com/spruceid/ssi/blob/89630368438c81b55335362b21621d3aadd48d93/Cargo.toml)
and the
[AnonCreds v1 manifest](https://github.com/anoncreds/anoncreds-rs/blob/08317a7428afe81f7b710669dc64878f98a6447b/Cargo.toml);
the report supplies revision-pinned repository links for all twelve projects
and labels the one mutable vendor standards page with its retrieval date.

## Candidate decisions

| Candidate | Revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Spruce SSI | `89630368438c81b55335362b21621d3aadd48d93` | `conditional-adopt` | Narrow modules are strong standards engines; the umbrella is too coupled. | A named subcrate passes exact-profile, effective Rust/toolchain, cone, target, security, and facade-isolation gates. |
| Procivis ONE Core | `e66ec8c533acb919611df75a38c7d23c2ccf658a` | `oracle` | Broad production behavior is valuable; core packages are unpublished and application-coupled. | A cohesive published component is isolated from transport, database, runtime, and product policy. |
| Impierce OpenID4VC | `e9d99d211036f61d46f2c5c56e26197f74290929` | `oracle` | Current profiles are useful, but Git/fork dependencies and target/toolchain gaps reject production use. | Released narrow protocol crates remove Git patches and pass SDK targets. |
| IOTA Identity | `7dd527087b96bf26ea97d486225eff8278c9a89c` | `oracle` | Mature independent behavior, but importing another DID/VC object model increases coupling. | A unique narrow capability preserves Identus public states and passes the cone/target gates. |
| OWF Askar | `48a495920e2166771c6be1aca2fd057a8b0c8831` | `spike` | Viable optional native storage/KMS adapter with native, unsafe, and runtime costs. | Issue #162 proves the adapter boundary and platform/security behavior. |
| DIF did-key.rs | `eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf` | `oracle` | Useful method behavior, but stale dependencies, nightly CI, and maintenance risk reject production use. | Maintained release plus exact key-family and target parity. |
| SICPA DIDComm Rust | `4388350def84b6d7f6b65cf4a451607200035d8d` | `not-adopt` | Useful reference, but its last release and dependency generation predate the selected profile. | Maintained DIDComm 2.1 release with modern dependencies and target PoC. |
| AnonCreds Rust | `08317a7428afe81f7b710669dc64878f98a6447b` | `conditional-adopt` | Reimplementing CL-signature AnonCreds is unjustified; native/FFI/VDR boundaries require isolation. | IDR-050 activation plus native/mobile, secret, unsafe, and conformance evidence. |
| AnonCreds v2 Rust | `691297a7f9ffcc1f51a5d30741086402d64544c9` | `not-adopt` | No settled interoperability profile/release and pre-release cryptography dependencies. | Stable profile, vectors, reviewed release, MSRV, and mobile/WASM evidence. |
| OWF VCX | `5fa3cd5023ed4a0dc42a36f48bb1834961c3812d` | `oracle` | Mature Aries behavior, but a large unpublished runtime-coupled DIDComm v1 workspace. | A unique cohesive capability can be isolated without the VCX wallet/runtime model. |
| OWF Labs SD-JWT Rust | `146546cc20682b6ddee0fcc270211aac9b0bc83b` | `not-adopt` | Implements obsolete SD-JWT draft-07, not RFC 9901; legacy reference use remains allowed. | Published RFC 9901 release passes SDK disclosure/key-binding/target gates. |
| Spruce DIDKit | `57a3b45111f8b1003ef1a50cf7cf21c81cd59cb1` | `not-adopt` | Archived/deprecated facade over old SSI and binding generations. | Maintained unarchived release provides unique value absent from underlying libraries. |

## Compatibility and dependency evidence

The portfolio report records declared Rust versions, editions, releases,
publication state, Git dependencies, default features, runtime/native/unsafe
surfaces, and upstream platform evidence. These are source observations, not
SDK build results. All effective Rust/toolchain and target gates at integration
time remain mandatory in each future adoption issue; no candidate is deemed
compatible merely because its upstream CI is green.

Every future integration records the exact version and features, direct and
resolved dependency cone, MSRV, public and wire compatibility, and the private
facade boundary. A primary source URL for each observation is recorded in the
report beside its pinned revision or dated mutable vendor evidence. License and
provenance must be rechecked at file level before source or fixtures are copied.

## Security, privacy and maintenance evidence

The review inspects repository-owned security-relevant metadata and flags
framework type leakage, secret ownership, FFI, native libraries, databases,
network runtimes, Git dependencies, old crypto generations, and draft
profiles. It does not reproduce an audit or assert absence of vulnerabilities.
Every production adoption must run current SDK supply-chain/security gates and
specialist review proportional to reachable cryptography, unsafe, and native
code.

## Rejected or deferred candidates

Every non-immediate disposition has a current alternative and objective
trigger. Runtime use is forbidden for `oracle` and `not-adopt` until
a new issue updates the ADR. Conditional adoption authorizes only a focused
spike/adapter issue, never direct framework exposure.

Rollback for this documentation-only decision is a revert of its PR. A future
runtime dependency has its own independently reversible integration and cannot
inherit rollback evidence from this portfolio.

## Open questions and blockers

There are no blockers for recording the decisions. Component demand,
normative-profile selection, licensed ISO material, mobile runtime behavior,
and security review remain blockers inside future adoption issues rather than
implicit approval here.

## Evidence commands

- `scripts/factory doctor` passed on `develop@ff924db61452d4a1fd2a8bd4a1bfd3d702519604` before edits.
- GitHub repository metadata and immutable default-branch revisions were read on 2026-09-08.
- Repository-owned manifests, READMEs, workflows, releases, and archival state were inspected.
- Existing ADR 0061, its research report, negative ledger, #162, and #164 were reconciled.
- No candidate was added to Cargo and no consumer repository was changed.
- Factory, OpenSpec, documentation, and exact-diff review remain tasks and are not yet claimed complete.
- Exact commands run before implementation: `scripts/factory doctor`, GitHub
  repository metadata/revision queries, and repository-local policy searches.
  Unrun checks at this stage are the final factory, text, link, and diff gates;
  they remain explicit tasks and are not represented as passing.
- Candidate-specific `cargo tree`, target compilation, advisory, unsafe/native,
  and conformance commands were not run for newly added candidates. A
  `conditional-adopt` decision permits that focused evidence work; production
  integration remains blocked until the owning issue records and passes it.
