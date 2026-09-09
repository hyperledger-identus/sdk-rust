# `identus-core` public input-boundary inventory

This inventory covers every public type exported by `identus-core`. It records
existing behavior; it does not add a wire or API promise. It is crate-scoped
evidence for issue #246, not completion of the repository-wide audit tracked by
issue #168 and `SDK-LIM-007`.

An intrinsic bound starts only when the typed SDK API receives its argument.
Callers must bound transport and decompression bytes before allocation. A serde
deserializer may allocate or scan its input before invoking the SDK type, and
an owned `String` has already been allocated before `Url::try_new` or
`TryFrom<String>` can inspect it.

## Inventory

| Public item | Construction, input, and serde surface | Retained representation and intrinsic bound/work | Outer obligation | Deterministic evidence |
| --- | --- | --- | --- | --- |
| `Component` and `COMPONENT` | External struct literal with public `name` and `summary`; no serde | Two `&'static str` references. No intrinsic text-byte bound or constructor allocation; access is constant-time. | Static storage is caller-owned. Do not leak allocated request data to manufacture a `'static` input. | `crates/core/src/lib.rs`; `tests::component_self_describes` |
| `CapabilityId` | `new(&'static str)`; no serde | One `&'static str` reference. Construction/access is constant-time; display work scales with caller-supplied static text. | No runtime text parser is provided; caller-created static storage remains caller-budgeted. | `crates/core/src/lib.rs`; `tests::capability_attribution_flows_through_public` |
| `ErrorCode` | `new(&'static str)`; no serde | One `&'static str` reference, with the same static-only bound as `CapabilityId`. | No runtime text parser is provided; caller-created static storage remains caller-budgeted. | `crates/core/src/lib.rs`; `tests::error_code_round_trips_stable_string` |
| `ErrorKind` | Eleven fieldless public variants; no serde | Finite enum; fixed retained size and constant work. | None. | `crates/core/src/lib.rs`; `tests::all_error_kind_families_are_present` |
| `IdentusError` | `public(ErrorCode, ErrorKind, CapabilityId, &'static str)` and `internal(ErrorCode, &'static str)`; no serde | Fixed fields containing static references and enums. Construction/access is constant-time; display work scales only with static code/message text. | Dynamic context must remain outside the value; caller-created static storage remains caller-budgeted. | `crates/core/src/lib.rs`; `tests::display_renders_code_and_public_message_only`, `tests::internal_sets_internal_kind_and_no_capability`, `tests::capability_attribution_flows_through_public` |
| `IdentusResult<T>` | Type alias only; no constructor or serde implementation of its own | Adds no storage, input, or work beyond `Result<T, IdentusError>` and imposes no bound on caller-defined `T`. | The crate that owns `T` owns its input and allocation bounds. | Alias declaration in `crates/core/src/lib.rs` |
| `UnixTimestampMillis` | `new(u64)`, `From<u64>`, `get`, `whole_seconds`, `checked_add(DurationMillis)`; serde serializes/deserializes a JSON-compatible unsigned integer | One `u64`, range `0..=u64::MAX`; fixed retained size and constant-time arithmetic, with checked overflow. | A format parser may scan or allocate the numeric token before typed `u64` deserialization; outer bytes remain caller-bounded. | `crates/core/src/time.rs`; `time::tests::checked_time_arithmetic_never_wraps`, `time::tests::civil_values_serialize_but_monotonic_values_have_no_wire_contract`, `time::tests::serialized_time_values_enforce_u64_json_range` |
| `DurationMillis` | `new(u64)`, `From<u64>`, `get`; serde serializes/deserializes a JSON-compatible unsigned integer | One `u64`, range `0..=u64::MAX`; fixed retained size and constant work. | Same pre-deserialization lexical-byte caveat as `UnixTimestampMillis`. | `crates/core/src/time.rs`; the three time tests cited above |
| `MonotonicTimestampMillis` | `new(u64)`, `From<u64>`, `get`, `checked_add(DurationMillis)`, `elapsed_since(Self)`; deliberately no serde | One `u64`, range `0..=u64::MAX`; fixed retained size and constant-time checked arithmetic. | Values come only through typed Rust construction or a `MonotonicClock`; there is no wire input owned by this type. | `#[newtype(display)]` declaration in `crates/core/src/time.rs`; `time::tests::checked_time_arithmetic_never_wraps`, `time::tests::clock_ports_are_independent_and_object_safe` |
| `ClockError` | Two currently constructible fieldless variants; `to_identus_error` consumes only the enum; no serde | Finite non-exhaustive enum; fixed retained size and constant work. | None. | `crates/core/src/time.rs`; `time::tests::clock_errors_bridge_without_runtime_detail` |
| `WallClock` | Object-safe `now(&self)` port with no caller-supplied argument | The trait retains nothing and accepts no input; it returns a range-bounded `UnixTimestampMillis`. | Implementer state, I/O, and work bounds belong to the adapter. | `crates/core/src/time.rs`; `time::tests::clock_ports_are_independent_and_object_safe` |
| `MonotonicClock` | Object-safe `now(&self)` port with no caller-supplied argument | The trait retains nothing and accepts no input; it returns a range-bounded `MonotonicTimestampMillis`. | Implementer state and work bounds belong to the adapter. | `crates/core/src/time.rs`; `time::tests::clock_ports_are_independent_and_object_safe` |
| `Url` and `MAX_URL_BYTES` | `parse(&str)`, `FromStr`, `try_new(String)`, `TryFrom<String>` and string serde; `as_str`/`AsRef<str>` are borrowed accessors | One owned `String`, at most 8,192 UTF-8 bytes after every public validated construction path. Length precedes syntax traversal; validation, clone, display, hash and serialization work are linear in at most 8,192 retained bytes. | Borrowed parsing validates before cloning. Owned construction receives an existing allocation, and serde allocates a `String` before validation; transports and serializers need their own limits. | `crates/core/src/url.rs`; `url::tests::url_accepts_every_validated_path_at_exact_byte_limit`, `url::tests::url_rejects_every_validated_path_above_byte_limit`, `url::tests::url_length_limit_precedes_syntax_validation`, `url::tests::url_limit_counts_utf8_bytes_not_characters`, `url::tests::url_serde_roundtrips_as_plain_string` |
| `UrlError` | Four fieldless public variants; `to_identus_error(&self)` accepts only the enum; no serde | Finite enum; fixed retained size and constant work with fixed diagnostic strings. | Rejected URL text and measured length are not retained; adapter-local diagnostics must preserve that property. | `crates/core/src/url.rs`; `url::tests::url_error_bridges_to_redaction_safe_identus_error`, `url::tests::url_rejects_every_validated_path_above_byte_limit` |

The generated constructors and serde implementations above are supplied by
`identus-derive::Newtype`; their exact expansion rules are in
`crates/derive/src/num.rs` and `crates/derive/src/str.rs`. `Url::new_unchecked`
is `pub(crate)`, so it is not an external input surface. Re-audit this inventory
whenever an exported core item, constructor, trait method, serde implementation,
retained representation, or validation order changes.
