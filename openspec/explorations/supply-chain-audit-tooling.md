# supply-chain-audit-tooling

> **Status: Exploring** — not yet fleshed out. This is a pre-change exploration;
> it is not tracked by `openspec` as a change or spec, and is not apply-able.
> When ready, promote to `openspec/changes/` via `openspec new change`.

## Why

`nix-tooling` already runs `cargo-deny` (`rust-deny`) and `cargo audit`
(`rust-audit`, pinned `advisory-db` flake input, zero suppressions) on every
`nix flake check`. That covers the high-severity-CVE case for any dep in
`Cargo.lock`. But three policy gaps remain, and `add-crypto-capability` is the
first change to pull real third-party crypto crates into the graph — exactly
when those gaps bite. This exploration proposes tightening `deny.toml` *ahead*
of that workload so the crypto deps enter an already-strict graph, rather than
discovering policy gaps reactively.

## What (sketch)

- Flip `deny.toml` `[graph] all-features = true` — cargo-deny audits the **full**
  optional graph, independent of the (currently unenforced) "default = all
  features" invariant.
- Promote `[bans] multiple-versions` from `"warn"` to `"deny"` — a duplicate
  primitive (e.g. two `sha2` versions pulled transitively) fails CI. Sustainable
  because `workspace-dependency-conventions` makes versions single-source.
- Add a `[bans] deny` forbidden-crate list for legacy/unaudited crypto the
  workspace never wants: `openssl`, `rust-crypto`, `sodiumoxide`, `rand` `< 0.8`.
- Keep `[advisories] ignore = []` (zero suppressions) as an enduring invariant;
  document it. `rust-audit` stays the single source of truth for advisories;
  cargo-deny's own advisories check stays disabled (avoid dual verdicts).
- No new CI jobs, no new nix checks — the existing `rust-deny` / `rust-audit`
  absorb the change.

## Open Questions (to resolve before promoting to a change)

- **Minimal vs expanded scope**: keep this change to `deny.toml` only (default),
  or also add (a) an advisory-db update cadence/CI job, (b) an SBOM/cyclonedx
  generation step, (c) enabling cargo-deny's own advisories check as
  belt-and-suspenders? Default = minimal; expanded items deferred to follow-ons.
- **`multiple-versions = "deny"` scope**: across all crates (default), or scoped
  to a curated set? Revisit if a legitimate transitive duplicate forces a
  `[bans] skip` (documented escape hatch).
- **Final forbidden-crate list**: confirm `sodiumoxide` and `rand < 0.8` are the
  right legacy entries vs alternatives; `openssl` (keep crypto pure-Rust) and
  `rust-crypto` (unmaintained) are the clear ones.
- **No current breakage**: today's graph (only `toml`, no duplicates, no
  forbidden crates) passes under the tightened policy — verified by reasoning;
  to re-confirm during apply.

## Notes

- Sequenced **after** `add-workspace-dependency-conventions` (soft — single-source
  makes `multiple-versions = "deny"` trivially satisfiable) and **before**
  `add-crypto-capability`.
- The crypto crate's dependency choices themselves (RustCrypto / dalek / ring =
  well-audited crates) belong to `add-crypto-capability`'s design, not here. This
  exploration only governs the *policy* (deny.toml), not the *choices*.