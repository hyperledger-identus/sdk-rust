# Roadmap

The technical roadmap and component sequence live in the
[SDK-Rust blueprint](docs/architecture/sdk-rust-blueprint.md).
The thirty SDK-owned deliverables and their actual state live in the canonical
[SSI upstream dependency backlog](docs/roadmap/ssi-upstream-dependency-backlog.csv),
supported by the [source matrix](docs/architecture/ssi-upstream-source-matrix.md)
and tracked in
[issue #20](https://github.com/hyperledger-identus/sdk-rust/issues/20).

The active order is:

1. establish `develop` from the selected `yet-another-seed@662f8d7` baseline
   and merge the governance packet while leaving `main` unchanged;
2. close `IDR-001` through `IDR-003`: stabilize the inherited workspace,
   finish the architecture and target matrices, classify placeholders and
   document current API risk;
3. establish organization-controlled crates.io ownership and namespace policy
   (issue #3);
4. harden the existing ports/conformance, validated-newtype and cryptographic
   foundations instead of rebuilding them (`IDR-004`, issues #4 and #9);
5. converge DID Core and method-neutral ports (`IDR-005` and `IDR-006`, issue
   #5), then the optional DID HTTP binding (#10);
6. converge credential, presentation, verification and storage foundations
   (`IDR-007` through `IDR-010`) before format and protocol engines;
7. deliver the first immutable consumer candidate (`IDR-011`) with SBOM,
   provenance and downstream adapter evidence;
8. converge bounded JOSE (#8), OID4VCI (#7), OID4VP, SD-JWT VC, mdoc, HAIP and
   status in the dependency order encoded by the canonical backlog;
9. crystallize conditional DIDComm, SIOPv2, compatibility formats and FFI as
   separate component decisions; inherited placeholder crates do not enter the
   roadmap automatically;
10. stabilize toward 1.0 only after independent audit, reproducible protected
   releases and two independent consumers;
11. decide if and how `develop` is promoted to `main` in a later release ADR.

Every component is released upstream before any downstream product deletes or
repoints an implementation. Oxid and chain-specific work streams remain
unchanged until their own adoption issues are approved.

Apollo is a Kotlin compatibility/vector source, while NeoPRISM `lib/apollo` is
the current Rust extraction source. Apollo deprecation and NeoPRISM reduction
are separate downstream governance/adoption outcomes after equivalent SDK
components exist; neither is implied by a backlog row.
