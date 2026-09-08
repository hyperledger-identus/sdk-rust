## Context

The first-party unsafe forbid is inherited by every package but rustc may
suppress the lint for external expansion spans. Direct `Newtype` templates are
the only first-party generated-code surface. Their output is finite and
currently safe, so the goal is preventive compiler enforcement without an
expanded-source scanner or new CI lane.

## Goals and non-goals

Goals are stable caller-spanned direct generation, compiler-negative evidence,
unchanged macro semantics and an exact residual limitation. Non-goals are
certifying third-party/nested expansions, parsing arbitrary rendered output,
adding nightly, approving unsafe code or changing macro APIs.

## Decisions

### Span direct template syntax at the derived identifier

Each quote template in `bytes`, `str` and `num` receives `ctx.name.span()` and
uses `quote_spanned!`. Tokens written directly in a template receive that
caller span; interpolated identifiers, paths and types retain the spans they
already carry. Helper functions receive the same span explicitly where they
emit template syntax. Empty token streams use `TokenStream2::new()` rather than
a plain quote invocation.

The `port` attribute already parses and emits only caller input on success; it
does not generate implementation syntax and needs no re-spanning.

### Prove the compiler premise independently

Extend the existing unsafe-policy behavioral fixture with a separate
dependency-free proc-macro/consumer workspace. Its macro uses
`quote_spanned!`-equivalent stable token spans from caller input to emit an
unsafe block. The opted-in consumer must fail and diagnostics must include the
unsafe-code lint identifier. The fixture does not snapshot full stderr.

### Use existing finite-shape tests for semantics

The current derive runtime and trybuild suites cover the supported string,
bytes and numeric forms, validation, serde/display and invalid inputs. They are
the appropriate regression surface for the production span change. A source
lexical guard would only police one spelling and is not treated as semantic
proof.

## Risks and mitigations

- Caller spans can move diagnostics: existing trybuild snapshots and focused
  tests detect material changes; derived-type attribution is intentional.
- A future template may use default spans: the ADR/spec mandate is reviewed
  with macro changes, while the compiler fixture continuously proves the
  stable mechanism.
- Nested output can still bypass the lint: `SDK-LIM-008` retains that explicit
  boundary.
- Compiler behavior may change: the pinned behavioral test fails closed on an
  etalon update.

## Rollout and rollback

Commit this specification and ADR before changing macro code. Then change only
the direct templates and behavioral fixture, update the effective constraint
record, run focused derive/conformance tests and all repository gates, archive
the completed OpenSpec change and open a signed/DCO PR to `develop`. Rollback
restores default template spans and the broader limitation.
