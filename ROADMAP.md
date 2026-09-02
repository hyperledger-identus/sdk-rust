# Roadmap

The technical roadmap and component sequence live in the
[SDK-Rust blueprint](docs/architecture/sdk-rust-blueprint.md).

The active order is:

1. establish `develop` from the selected `yet-another-seed@662f8d7` baseline
   and merge the governance packet while leaving `main` unchanged;
2. stabilize the inherited workspace: align toolchains, make all gates
   deterministic, classify placeholders and document current API risk;
3. establish organization-controlled crates.io ownership and namespace policy
   (issue #3);
4. harden the existing ports/conformance, validated-newtype and cryptographic
   foundations instead of rebuilding them (issues #4 and #9 as applicable);
5. converge DID Core (#5), then the DID HTTP binding (#10);
6. converge VC Core (#6), bounded JOSE (#8) and OID4VCI (#7);
7. crystallize OID4VP/SIOPv2, credential formats and FFI as separate component
   decisions; do not treat inherited placeholder crates as accepted scope;
8. stabilize toward 1.0 only after independent audit, reproducible protected
   releases and two independent consumers;
9. decide if and how `develop` is promoted to `main` in a later release ADR.

Every component is released upstream before any downstream product deletes or
repoints an implementation. Oxid and chain-specific work streams remain
unchanged until their own adoption issues are approved.
