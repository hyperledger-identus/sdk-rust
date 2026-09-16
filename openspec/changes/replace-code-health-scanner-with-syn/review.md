# Review

## Scope and identities

- Base: `5dff6f38c861b858dd62dc8310246f7d485d3e92`
- Classifier implementation: `18394fc38cc085445979ae5e123fe9454ab22bae`
- Exact locally reviewed classifier/evidence head:
  `181b5c0e589b7fbdeb24d6c5e90ad8557bc6e148`
- Issue: [#275](https://github.com/hyperledger-identus/sdk-rust/issues/275)
- Review lenses: parser correctness, conservative population projection,
  module reachability, resource bounds, diagnostics, dependency direction,
  public/API isolation, reproducibility and migration integrity

## Findings

1. **Resolved — Python retained a hermetic test dependency on Cargo.** The
   source-binding unit test began invoking the Rust classifier after v2
   orchestration was introduced. The synthetic factory derivation intentionally
   has no Rust toolchain. The test now injects an exact classifier result; Rust
   protocol and integration behavior remain covered independently.
2. **Resolved — synthetic factory source omitted v2 evidence.** The fixture now
   copies both the migration report and classifier source required by the
   structural contract.
3. **Resolved — conformance dependency declarations were not Taplo-formatted.**
   Canonical alignment was applied with no dependency or semantic change.
4. **No blocker — conservative syntax boundary.** Unknown cfg predicates,
   opaque macro token streams, malformed syntax, invalid spans, ambiguous
   modules and escaping path overrides cannot become test-only evidence.
5. **No blocker — resource and diagnostic boundary.** Request, aggregate/file
   source, path, file-count, span and module-edge work are bounded. Diagnostics
   identify paths and parse locations without returning source contents.
6. **No blocker — architecture/API boundary.** The classifier remains a binary
   in the unpublished conformance package. No parser type enters an SDK crate,
   and no public, wire, persisted, target-runtime or release surface changes.
7. **No blocker — dependency cone.** `syn`, `proc-macro2`, `serde` and
   `serde_json` were already workspace-locked. The change adds no new package
   to `Cargo.lock` and no native or unsafe dependency boundary.
8. **Resolved hosted P1 — aggregate equality did not bind classifier output.**
   The v2 report now records and fast validation recomputes a canonical digest
   of every production source's exact inline-test line set plus external and
   generated path sets. A mutation test moves the digest without changing
   aggregate counts and fails closed.
9. **Resolved hosted P2 — direct path overrides used ordinary module base.**
   Literal `#[path]` overrides now resolve from the containing source directory
   plus inline-module context, while ordinary child modules retain the
   file-stem directory rule. A non-root `foo.rs` regression proves `bar.rs`.
10. **Resolved hosted P1 — baseline reachability depended on merge method.**
    The baseline now classifies the durable pre-change `develop` revision. It
    remains reachable under merge, squash, or rebase. The exact projection
    digest, classifier identity/protocol, and locked dependencies bind the
    current implementation independently of the historical source revision.
11. **Resolved hosted P1 — conditional module path could hide production.**
    An active or unknown nested `cfg_attr` that may apply `path` now fails
    closed. A regression combines conditional production and test-only edges to
    the same file.
12. **Resolved hosted P2 — nested scopes lost inherited cfg state.**
    Reachability now propagates through item, statement, expression, and arm
    scopes. A local module below a test-only block is covered by regression.
13. **Resolved hosted P2 — associated items lost inherited cfg state.**
    Impl, trait, and foreign item attributes now update inherited reachability
    before their nested syntax is visited. Test-only impl and trait methods with
    local modules are covered by regression.
14. **Resolved hosted P2 — generated modules disappeared from resolution.**
    Exact allowlisted generated sources are passed to the classifier graph but
    remain omitted from authored metrics and the production projection.
15. **Resolved hosted P2 — standard file-based binaries were not roots.**
    `src/bin/*.rs` now uses its containing directory as the ordinary child base,
    matching Cargo's conventional binary-target layout.
16. **Resolved hosted P2 — inline-module path attributes lost directory state.**
    Inline modules now replace their logical-name directory with a contained
    literal path context before nested declarations are traversed.
17. **Resolved hosted P2 — nested overrides in non-root modules lost the parent
    module directory.** Direct overrides remain source-directory relative;
    overrides inside inline contexts now begin at the ordinary parent module
    directory plus that context.
18. **Resolved hosted P2 — struct-literal field attributes were omitted.**
    `syn::FieldValue` spans and inherited reachability are now classified, with
    a focused mixed test/shipping literal regression.
19. **Resolved hosted P2 — test-only conditional paths used production cfg.**
    Module paths are evaluated for both test modes. Proven test-only edges use
    the test selection; unknown path predicates and configuration-dependent
    production paths fail closed.
20. **Resolved hosted P2 — binary roots and nested modules shared one path
    role.** Resolution now carries root versus nested entry. A target's child
    uses the source directory, while a child reached as a module uses its own
    module directory.
21. **Resolved hosted P2 — target roots could become inherited test-only.**
    Standard root filenames and sources with a top-level `main` seed production
    reachability. A test-only edge cannot remove an independently shipping
    target from production.
22. **Resolved hosted P2 — closure parameter attributes were omitted.**
    Attributed `syn::Pat` nodes now participate in span classification and
    inherited reachability, with a focused closure regression.
23. **Resolved hosted P2 — test edges used only one file-entry role.** A
    source reachable as both a target root and a nested module now contributes
    test edges from both contexts; the reachability search carries `(path,
    role)` identities and a dual-fixture regression proves both results.
24. **Resolved hosted P2 — declaration fields lost inherited cfg state.**
    `syn::Field` attributes now update module reachability before nested type
    and const syntax is visited, with a test-only field-local module regression.
25. **Resolved hosted P2 — bare function parameters were omitted.**
    `syn::BareFnArg` spans and inherited reachability are now classified, with
    a focused function-pointer parameter regression.
26. **Resolved local completeness audit — three attribute-bearing nodes were
    omitted.** An exhaustive review of the locked `syn` file AST found
    `FieldPat`, `Variadic`, and `BareVariadic` outside the already covered enum
    wrappers. All three now classify spans and inherited reachability, with a
    mixed shipping/test regression.
27. **Resolved hosted P2 — variant attributes did not reach discriminants.**
    `syn::Variant` now propagates cfg state into discriminant expressions and
    their local modules.
28. **Resolved hosted P2 — ordinary argument attributes did not reach types.**
    `syn::FnArg` now propagates receiver/typed argument cfg state into nested
    type and const syntax.
29. **Resolved hosted P2 — generic parameter attributes did not reach
    defaults.** `syn::GenericParam` now propagates lifetime/type/const cfg state
    into bounds and defaults. One combined regression proves all three module
    edges remain test-only.

## Result

No unresolved local architecture, security, privacy, compatibility, API or
operations finding remains. Hosted exact-head confirmation is pending. The
one-time v1/v2 comparison remains exact against the durable source revision.
