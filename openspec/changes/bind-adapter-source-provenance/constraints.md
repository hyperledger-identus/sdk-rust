# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/512
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`, `SDK-ARCH-002`, `SDK-DELIVERY-001`, and `SDK-LIM-005` remain
unchanged. The work is repository evidence and introduces no production
dependency or consumer policy.

## Introduced or changed constraints

Canonical adapter source identity must validate offline from bounded regular
non-symlink files. It must not call Git, GitHub, a network service, a compiler,
or a parser. SHA-256 and the declared Git blob algorithm are computed from the
same exact bytes. Mappings share source records rather than duplicate
revision/path/digest fields.

## Introduced or changed limitations

The manifest does not attest maintainer intent, commit signatures, or remote
repository state. It detects local/source-archive drift. Exact byte identity
means formatting changes rotate evidence. Git SHA-1 is retained only as an
interoperable object identity and is paired with SHA-256.

## Consumer and product impact

None. The public Rust, wire, error, persistence, ABI, target, runtime, and
security behavior remain unchanged. No SDK-TS, SDK-Swift, SDK-KMP, Oxid,
Midnight, or other downstream repository is modified.

## Activation and rollback

Activation requires schema-v2 data, deterministic rendering, mutation tests,
factory validation, independent review, and protected CI. Before merge the
change can be removed. After merge, corrections require a registry version
increment; mappings must not silently fall back to unbound revision strings.

## Evidence

Issue #512, merged A1 milestone #504, PR #511 review findings, the canonical
mapping registry/checker, and the pinned Git/Cargo primary documentation in
`research.md` provide the decision and implementation boundary.
