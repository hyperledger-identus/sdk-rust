# SSI Domain Naming Policy

New `sdk-rust` public surfaces must use SSI domain terminology. Legacy Identus
SDK codenames are historical source evidence only; they must not become new
Rust crate names, module names, traits, DTOs, APIs, documentation headings, or
backlog item names.

## Required Vocabulary

Prefer direct SSI domain names:

- cryptography
- key
- DID
- DID resolver
- credential
- presentation
- DIDComm
- mediation
- wallet
- agent
- trust
- OpenID4VC
- storage
- adapter
- binding

## Historical Evidence Boundary

Legacy names may appear only when a document is explicitly mapping or citing
source repositories. Examples include migration maps, compatibility notes,
source evidence tables, and conformance tests that enforce the restriction.

New product-facing names must describe their SSI capability, not the old SDK
module lineage.

## Audit Scope

The default conformance suite checks the core public surface files:

- workspace and crate manifests
- crate source files
- constitution and Spec Kit plan/tasks
- component architecture docs
- binding boundary docs

Migration maps and source evidence documents are intentionally excluded from the
strict block because they must cite legacy repositories accurately.

## Acceptance Rule

A new public name is acceptable when a maintainer can answer both questions:

1. Does the name describe an SSI capability or technical boundary directly?
2. Would the name make sense to a contributor who has never seen the old SDK
   module codenames?

If either answer is no, rename the surface before implementation continues.
