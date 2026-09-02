# Security policy

The Identus SDK for Rust follows the
[Hyperledger Identus security policy](https://github.com/hyperledger-identus/.github/blob/main/SECURITY.md)
and the mandatory
[Hyperledger vulnerability disclosure policy](https://toc.hyperledger.org/governing-documents/security.html).

## Reporting a vulnerability

Do not open a public issue, Discussion or pull request for a suspected
vulnerability.

- Use **Report a vulnerability** in this repository's GitHub Security tab; or
- email the [Hyperledger Foundation security list](mailto:security@lists.hyperledger.org)
  and identify `hyperledger-identus/sdk-rust` in the report.

Include affected revision/version, platform/features, impact, reproduction and
known mitigations. Do not include real wallet keys, credentials or personal
data; use synthetic encrypted material if a reproducer requires sensitive
structure.

The Identus security response team handles acknowledgement, embargo, private
patching, CVE coordination and disclosure under the controlling policies.

## Supported versions

The SDK is pre-release. Until the first published support matrix:

| Version | Support |
| --- | --- |
| Latest released `0.x` line | Security fixes when maintainers can reproduce and safely patch it |
| Current `develop` before a release | Best effort; not a released compatibility promise |
| Reserved `main`, candidate branches and unmerged commits | Unsupported reference material |

Each stable release updates this table and declares an end-of-support date.

## Security-sensitive areas

Changes in these areas require explicit security review:

- cryptographic algorithms, key conversion, derivation and algorithm registries;
- secret handles, zeroization, redaction and FFI boundaries;
- DID/URL, JOSE, CBOR/JSON, credential, status and protocol parsers;
- signature/holder binding, replay, nonce, audience, origin and time checks;
- decompression, redirects, network fetching and resource limits;
- dependency/source/license policy, release automation and trusted publishing;
- unsafe code and platform-specific bindings.

Technical verification results must not claim business or ecosystem trust. A
consumer's trust, consent and authorization policy remains outside the SDK.

## Secure-development baseline

- stable pinned toolchain and reproducible lockfile;
- dependency/source/license and advisory gates;
- secret scanning and redaction tests;
- parser fuzzing and negative/resource-exhaustion tests;
- architecture checks preventing chain/product dependencies;
- signed commits, protected branches, signed releases, SBOM and provenance;
- no release from a developer workstation or personal crates.io token.

Passing these gates is evidence, not a security certification or warranty.
