//! The conformance rulebook — the static invariant *data* encoding the
//! workspace's crate-ring layer rules and dependency-direction policy.
//!
//! This module is the referenceable source of truth for guards: it holds the
//! `Layer`, `Member`, `LayerRule`, and `LAYER_RULES` definitions, kept
//! `pub(crate)` and runtime-available (NOT gated by `#[cfg(test)]`) so that
//! any guard may cite them. The enforcement *logic* (guards) lives under
//! `guard/` and is `#[cfg(test)]`; this module holds only data, separating
//! invariant data from guard logic per the `conformance-crate-structure`
//! capability.

/// One of the seven hexagonal-ring layers, ordered inward to outer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum Layer {
    Foundation,
    DomainPrimitives,
    CredentialSemantics,
    ProtocolSemantics,
    Orchestration,
    OuterBoundary,
    Verification,
}

impl Layer {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Layer::Foundation => "foundation",
            Layer::DomainPrimitives => "domain-primitives",
            Layer::CredentialSemantics => "credential-semantics",
            Layer::ProtocolSemantics => "protocol-semantics",
            Layer::Orchestration => "orchestration",
            Layer::OuterBoundary => "outer-boundary",
            Layer::Verification => "verification",
        }
    }
}

/// A member crate of a layer, carrying the per-member `proc_macro` flag.
/// `proc_macro = true` marks a build-time proc-macro crate (a crate whose
/// `Cargo.toml` declares `[lib] proc-macro = true`); such crates are exempt
/// from the inward-direction policy (any crate may depend on them).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct Member {
    pub(crate) name: &'static str,
    pub(crate) proc_macro: bool,
}

/// A single layer's rulebook entry: its member crates and the layers its
/// dependencies may point inward to.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct LayerRule {
    pub(crate) layer: Layer,
    pub(crate) members: &'static [Member],
    pub(crate) allowed_target_layers: &'static [Layer],
}

/// The in-source layer rulebook: the 7 layers, their `identus-*` crate
/// membership (each member carrying a `proc_macro` flag), and each source
/// layer's allowed inward target layers. Ported from the seed's
/// `layer_rules` + `allowed_target_layers_by_source_layer`; no generated-
/// snapshot fields. `identus-derive` is a foundation member flagged
/// `proc_macro = true`; proc-macro crates are build-time tooling and are
/// exempt from the inward-direction policy (any crate may depend on them).
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const LAYER_RULES: &[LayerRule] = &[
    LayerRule {
        layer: Layer::Foundation,
        members: &[
            Member {
                name: "identus-core",
                proc_macro: false,
            },
            Member {
                name: "identus-derive",
                proc_macro: true,
            },
        ],
        allowed_target_layers: &[],
    },
    LayerRule {
        layer: Layer::DomainPrimitives,
        members: &[
            Member {
                name: "identus-crypto",
                proc_macro: false,
            },
            Member {
                name: "identus-did",
                proc_macro: false,
            },
            Member {
                name: "identus-trust",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[Layer::Foundation, Layer::DomainPrimitives],
    },
    LayerRule {
        layer: Layer::CredentialSemantics,
        members: &[
            Member {
                name: "identus-credentials",
                proc_macro: false,
            },
            Member {
                name: "identus-presentations",
                proc_macro: false,
            },
            Member {
                name: "identus-jose",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[
            Layer::Foundation,
            Layer::DomainPrimitives,
            Layer::CredentialSemantics,
        ],
    },
    LayerRule {
        layer: Layer::ProtocolSemantics,
        members: &[
            Member {
                name: "identus-messaging",
                proc_macro: false,
            },
            Member {
                name: "identus-openid4vc",
                proc_macro: false,
            },
            Member {
                name: "identus-oid4vci",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[
            Layer::Foundation,
            Layer::DomainPrimitives,
            Layer::CredentialSemantics,
            Layer::ProtocolSemantics,
        ],
    },
    LayerRule {
        layer: Layer::Orchestration,
        members: &[
            Member {
                name: "identus-wallet",
                proc_macro: false,
            },
            Member {
                name: "identus-agent",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[
            Layer::Foundation,
            Layer::DomainPrimitives,
            Layer::CredentialSemantics,
            Layer::ProtocolSemantics,
            Layer::Orchestration,
        ],
    },
    LayerRule {
        layer: Layer::OuterBoundary,
        members: &[
            Member {
                name: "identus-adapters-entropy",
                proc_macro: false,
            },
            Member {
                name: "identus-did-resolver-http",
                proc_macro: false,
            },
            Member {
                name: "identus-bindings",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[
            Layer::Foundation,
            Layer::DomainPrimitives,
            Layer::CredentialSemantics,
            Layer::ProtocolSemantics,
            Layer::Orchestration,
        ],
    },
    LayerRule {
        layer: Layer::Verification,
        members: &[
            Member {
                name: "identus-conformance",
                proc_macro: false,
            },
            Member {
                name: "identus-wallet-conformance",
                proc_macro: false,
            },
        ],
        allowed_target_layers: &[Layer::Foundation, Layer::Orchestration],
    },
];
