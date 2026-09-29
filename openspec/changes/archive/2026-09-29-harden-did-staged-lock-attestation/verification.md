# Verification

## Exact revision

Initial implementation evidence was collected from
`3d0d3140537cba451677f55732cd8767f668dede`. Discovery-review remediation and
the final clean implementation evidence were collected from
`cf0d8a79289773a5ad531e4aa9b5a30f5f171e32`. The final evidence-record update
is documentation-only relative to that implementation.

## Focused gates

- `python3 -m py_compile scripts/prepare-did-candidate.py scripts/check-release-candidates.py scripts/tests/release-candidates.py`
- `python3 scripts/check-release-candidates.py`
- `python3 scripts/tests/release-candidates.py`
- `scripts/factory check`
- `nix flake check --print-build-logs`

All gates passed. The local macOS Nix evaluation explicitly omitted the
incompatible `x86_64-linux` checks; exact-head hosted CI remains the Linux
evidence authority.

## Refresh evidence

`nix run .#did-candidate -- --refresh-staged-lock --output <external-temp>/proposal --revision 3d0d3140537cba451677f55732cd8767f668dede`
passed under pinned Rust/Cargo 1.98.1. The report recorded:

- status `unchanged`;
- no added or removed package identities;
- current and proposed SHA-256
  `1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447`;
- identical repository status before and after the run; and
- an empty diff between the tracked and proposed lockfiles.

The proposal was emitted outside the repository and no tracked file changed.
The remediated exact-revision refresh repeated this result at `cf0d8a7` after
the output directory was reserved before resolution.

## Candidate evidence

`nix run .#did-candidate -- --output <external-temp>/candidate --revision 3d0d3140537cba451677f55732cd8767f668dede`
passed. Its receipt recorded a clean source, two byte-identical archive passes,
the staged lock digest above, and the Rustdoc and CycloneDX evidence.

The first candidate run on `d1b2cc70f3d575e25ca3b2b6ef466bf945475172`
correctly exposed that `cargo-cyclonedx` does not accept subcommand-local
`--locked`. A global Cargo option made the command pass at `3d0d314`, but the
subsequent discovery review demonstrated that Cargo consumed it without
forwarding locked behavior to the external plugin. The remediated path invokes
the pinned plugin directly with a closed `CARGO` wrapper that executes
`cargo metadata --locked`; its focused wrapper test and a complete dirty-tree
candidate precheck passed before the remediation commit. The complete clean
candidate was then rerun at `cf0d8a7`; it recorded `sourceDirty=false`,
`twoPassByteIdentical=true`, the descriptor-bound observed lock digest, and
both public-API and CycloneDX evidence documents for both packages.

## Matrix evidence

- Primary macOS matrix at `cf0d8a7`: Rust/Cargo 1.98.1, 8/8 rows passed in
  51.981 seconds.
- MSRV macOS matrix at `cf0d8a7`: Rust/Cargo 1.89.0, 8/8 rows passed in
  32.454 seconds.

Both aggregates recorded clean sources and the exact descriptor-bound staged
lock digest. The natural weekly slow lane remains the authority for the full
cross-platform matrix; this change does not dispatch or rerun it.

## Mutation evidence

The mutation suite rejected variable-built lock generation, ordinary runtime
generation through both command runners, purpose-owned manifest escape,
aggregate/descriptor digest drift, descriptor-echoed archive receipts,
post-separator Rustdoc locking, an unlocked CycloneDX metadata wrapper,
unbound CycloneDX wrapper environment, and non-exclusive refresh output.
