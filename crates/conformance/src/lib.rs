//! Conformance and layout enforcement for the Identus Rust SDK workspace.
//!
//! Owns the verification of workspace invariants. The invariant *data*
//! (the crate-ring layer rulebook) lives in `rulebook` — `pub(crate)` and
//! runtime-available, the referenceable source of truth. The enforcement
//! *logic* (the guards) lives under `guard` and is `#[cfg(test)]`, so no
//! guard logic compiles into a production build. Sits in the `verification`
//! layer and depends only on foundation (`identus-core`); production crates
//! must never depend on it.

use identus_core::Component;

mod rulebook;
#[cfg(test)]
pub(crate) use rulebook::*;

#[cfg(test)]
mod guard;

/// Metadata for the `identus-conformance` crate.
pub const COMPONENT: Component = Component {
    name: "identus-conformance",
    summary: "Conformance and layout enforcement for the Identus Rust SDK workspace.",
};
