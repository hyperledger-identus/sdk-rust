# Accept a bounded UniFFI generator dependency risk

## Why

The 2026-10-26 natural weekly slow run rejected `basic-toml 0.1.10` after
RustSec classified it as unmaintained in RUSTSEC-2026-0374. The crate is
reachable only through the separately locked, unpublished UniFFI binding
generator. UniFFI 0.32.2 still uses Askama 0.16, so a compatible upstream
upgrade does not remove it. Disabling the audit or the Swift/Kotlin/Android E2E
lane would hide rather than manage the dependency.

## What changes

- Record a narrow, time-bounded risk acceptance for RUSTSEC-2026-0374 only.
- Add one deterministic guard that proves the exact expected generator-only
  dependency path before invoking `cargo audit` with the single accepted ID.
- Make the acceptance fail closed after 2027-01-31 and whenever the dependency
  version/path changes, so upstream removal becomes mandatory rather than
  silently extending the exception.
- Keep generator `cargo deny` plus Swift, Kotlin/JVM, Apple and Android package
  execution unchanged.
- Document the owner, renewal prohibition and upstream exit condition.

## Capability

### Modified capability

- `native-did-bindings`: make temporary generator risk acceptance explicit,
  exact, expiring and independently testable.

## Non-goals

No runtime crate accepts an advisory. No vulnerability is ignored. This change
does not fork Askama, replace UniFFI, publish bindings, expand the FFI surface,
or weaken root/fuzz audits.

## Delivery

Issue #564 owns the decision and implementation. The natural scheduled lane
remains the authoritative post-merge evidence; it will not be manually rerun
or dispatched to manufacture a receipt.
