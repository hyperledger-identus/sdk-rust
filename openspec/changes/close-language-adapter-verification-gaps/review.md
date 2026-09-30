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
