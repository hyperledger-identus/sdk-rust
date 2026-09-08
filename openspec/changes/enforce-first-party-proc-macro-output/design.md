## Context

The first-party unsafe forbid is inherited by every package but rustc may
suppress it for external expansion spans. Direct `Newtype` templates are the
only first-party generated-item surface. Their output is finite and currently
safe, so the goal is preventive validation without changing downstream lint
spans or adding an expanded-source scanner.

## Goals and non-goals

Goals are fail-closed direct-output validation, machine-negative evidence,
unchanged macro behavior and an exact residual limitation. Non-goals are
executing nested macros, certifying third-party expansions, approving unsafe
code or changing macro APIs.

## Decisions

### Validate the completed generated item syntax

After `bytes`, `str` or `num` expansion and helper-stream composition, parse
the resulting token stream as `syn::File`. A private `syn::visit::Visit`
implementation recursively records the first prohibited construct. Parse or
validation failure becomes `syn::Error` before tokens leave the derive macro.

The visitor mirrors exact Rust 1.98.1 `UnsafeCode` early-lint cases: unsafe
blocks, signatures, traits, trait implementations and extern blocks;
`global_asm!`; and `allow_internal_unsafe`. It additionally rejects all four
unsafe attributes listed by the current Rust Reference: `export_name`,
`link_section`, `naked` and `no_mangle`, wrapped or legacy. This is structured
syntax validation, not rendered-source matching.

The successful `port` attribute returns caller-authored input unchanged and
does not pass through the generated-item guard. Authored unsafe port syntax is
already handled by the workspace compiler policy.

### Preserve generated spans

Default `quote!` templates stay unchanged. Applying caller spans globally was
rejected after a focused production experiment caused 21 unrelated warning
errors for valid derives. Adding blanket generated allows was rejected because
it would alter downstream lint policy and mask maintenance problems.

### Prove validation and compatibility independently

Unit tests exercise every prohibited syntax category, nesting and safe control
tokens. Existing runtime and trybuild suites cover all supported string, bytes
and numeric shapes, validation, serde/display and invalid input diagnostics.
No hidden public probe macro or duplicated conformance implementation is added.

## Risks and mitigations

- The unsafe inventory may evolve: exact compiler/Reference sources are pinned
  and a Rust etalon update triggers review.
- Nested macro output is opaque until later expansion: `SDK-LIM-008` retains
  that explicit boundary.
- Parsing could reject context-dependent output: derives are required to emit
  items, and every current finite shape is re-tested.
- The `syn` feature surface grows: only existing stable `visit` is enabled and
  the version/resolved package cone remains unchanged.

## Rollout and rollback

Correct this specification and ADR before implementation. Then add the private
visitor and tests, update the effective constraint record, run focused and all
repository gates, archive the completed OpenSpec change and open a signed/DCO
PR to `develop`. Rollback removes the visitor and restores the broader
limitation.
