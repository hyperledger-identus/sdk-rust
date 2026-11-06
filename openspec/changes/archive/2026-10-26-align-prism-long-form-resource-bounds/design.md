# Design

## Lexical budgets

`Did` checks a literal 131,072-byte ceiling and `DidUrl` checks a literal
139,264-byte ceiling before the existing single-pass grammar traversal. Literal
public constants preserve architecture-checker resolution and make generated
adapter contracts unambiguous. DID URL parsing still enforces the embedded bare
DID ceiling separately and caps the combined path/query/fragment suffix at
8,192 bytes. The independent suffix budget prevents short DIDs from turning the
larger state-bearing envelope into dereferencing parameter and output
amplification.

`DidMethod` retains its 2,042-byte ceiling. The constant and specification no
longer derive from `MAX_DID_BYTES`; a large state-bearing method-specific ID
does not justify an enormous method name.

`Uri` uses the same literal 139,264-byte ceiling as `DidUrl`. DID document
identifiers use this generic URI type, and `From<DidUrl>` is intentionally
infallible; a lower URI ceiling would violate the value invariant and prevent
long-form document projection. Generic JSON ingress remains independently
bounded.

## Cache behavior

The 65,536-byte cache key ceiling remains a retention budget, not a resolution
validity ceiling. `CachedDidResolver` detects an oversized normalized identity
before touching the clock or backend, resolves upstream, and reports
`KeyBudgetBypass`. It does so in both bypass-on-error and fail-closed modes
because no infrastructure has failed. Real clock/backend/entry failures retain
their configured semantics.

## Portable evidence

The shared vector packet generates exact-limit and one-over strings. Its
checker permits at most 262,144 repeats, keeping fixture expansion finite while
covering both public ceilings. The packet digest, catalog descriptions,
language-adapter mappings, and input-resource inventory change atomically.
Fuzz boundary probes consume exported constants. WASM and UniFFI tests accept a
100 KB PRISM-shaped value and continue to reject over-limit values.

## Separated responsibilities

This design admits bytes only. Issue #521 owns PRISM raw-operation/context
bounds, base64url codec, semantic validation, state reduction, document
projection, output budgets, and local method routing. Generic document and
resolution-result hostile JSON parsers remain at 256 KiB and 512 KiB. HTTP and
foreign runtimes retain their outer preallocation and request-size policy.
