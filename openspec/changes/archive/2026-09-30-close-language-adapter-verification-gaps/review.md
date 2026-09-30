# Exact-diff review

## Scope

- Base: `644993c859b87e2da16612fcdf7cf1868cfe5981` (`origin/develop` at
  preflight)
- Reviewed implementation head:
  `546602f588153b68e74609ecd4302665a6246fab`
- Issue: #510
- Boundary: registry metadata, deterministic renderer, validator, mutation
  evidence, and specification only

The 23-path replacement diff includes the complete reviewed #509 mapping
baseline because that draft was never merged, followed by the #510 corrections.
No Rust crate source, generated binding, consumer repository, release, or
publication path is changed.

## Review findings

No unresolved correctness, security, architecture, or test finding remains in
the reviewed head.

The review specifically confirmed:

1. Cargo package identity, canonical Rust API path, and stable error code are
   independent data and render independently.
2. Canonical source evidence rejects traversal, Windows-style separators,
   every symlink component, outside-root resolution, non-regular or oversized
   files, invalid UTF-8, and non-literal or ambiguous bounds without traceback.
3. One-way mappings cannot admit `both` or reverse field directions.
4. Every lossy mapping has a closed structured loss record; unsupported cases
   reference declared loss IDs without requiring every error coalescing loss to
   be an unsupported value.
5. Schema-v1 language evidence is restricted to the pinned patch interval.
6. Vector IDs resolve exactly once against the canonical catalog, match the
   capability and include the adapter language target; shared references remain
   valid for additive mappings.
7. The mutation harness discovers every bound source from registry data,
   refuses to conceal source symlinks, copies the vector catalog, and proves a
   fifth Swift mapping with a second source path.
8. Rendered Markdown is deterministic and exposes every governed compatibility,
   resource, migration, error, and loss field needed for human review.

## Residual risk

This contract validates declared metadata; it does not generate or execute a
language adapter. Runtime binding behavior remains a later issue and must use
the stable registry plus canonical vectors rather than infer compatibility from
the SDK-TS DTO shape.

## Independent discovery review

Claude Code reviewed PR #511 at `14827c51297ae8d3916c2ac188065f325fa096b4`
and found no active registry-data error or containment vulnerability. It raised
four gate-quality findings:

1. Mutation evidence did not exercise enough pre-existing schema rules.
2. Vector resolution did not correlate catalog outcomes with mapping kind.
3. A syntactically valid canonical revision is not proven to exist or match the
   working source in a Git-independent Nix input.
4. Compatibility versions admitted non-canonical leading-zero spellings.

Findings 1, 2, and 4 were accepted and remediated in #510. The mutation suite
now covers the closed schema, provenance shape, mapping kinds, redaction,
ownership, direction, loss, error, resource, lifecycle, deterministic-render,
and vector-outcome rules. Value mappings require `success`; error mappings
require a declared stable Rust error code. Semver components are canonical.

Finding 3 is valid research input but is not a blocker for this frozen offline
contract: a Git-object lookup would make identical validation depend on whether
`.git` is present, while Nix source inputs intentionally omit it. The current
record therefore treats the full SHA as immutable provenance and validates the
working bound independently. A follow-up issue owns a Git-independent content
binding design rather than silently adding an environment-sensitive check.

A fresh verification review at remediated head
`b6f257a18f40994875c4bf6d90cd524734f36e34` found four additional blockers:
scalar error metadata could reach outcome correlation before its type guard; a
mapping could borrow an unrelated crate's bound; the vector catalog could be a
symlink; and invalid UTF-8 was misclassified as outside-root resolution because
`UnicodeDecodeError` is a `ValueError` subclass. All four were reproduced and
fixed. Outcome correlation now fails safely for malformed error tables; bound
sources must remain below the declared Cargo package source; catalog loading
uses the no-symlink containment rule; and exception order preserves the bounded
unreadable-evidence diagnostic.

The next exact-head verification review at
`ee24ad37d9c9b5d0af7dc32f0fd53e702bcff222` found five additional schema
integrity blockers: same-crate bound substitution, boolean/float numeric
coercion, duplicate Rust field and error identities, and incomplete lifecycle
coherence. All were reproduced and corrected. A selected bound must now be
named by referenced vector provenance; schema/owner values use exact integer
types; Rust and language fields plus stable Rust error codes are unique; and a
closed lifecycle matrix governs state/phase combinations.

The final independent verification at
`c6f520251fdc4f1e3840ac9226061098fbe119d1` reported no blocking correctness,
security, test, documentation, or release-risk finding. It independently ran
the focused checker, mutation suite, full factory gate, factory-contract suite,
diff hygiene, and signature/DCO inspection. Its useful non-blocking observation
that some negative cases asserted only exit status was accepted: the final
suite now asserts the intended diagnostics for malformed error tables, empty
value/error records, and lossless/lossy invariants. The suggested history
rewrite for an `Assisted-by` trailer was not adopted because SDK-Rust governance
requires verified signatures and DCO trailers, both already satisfied, and
does not require that additional trailer.
