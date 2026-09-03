# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-03
- **Issue:** #35, child of #5 / `IDR-005`
- **Develop base:** `267b435ece23d935fc4a4946df0b496c91466e66`
- **Started:** `2026-09-03T09:44:39Z`
- **Result:** contract is implementable with no unresolved blocker

## Ownership and provenance findings

Generic DID/DID URL lexical assurance belongs with the generic parser in
sdk-rust. Midnight's existing property tests generate method-specific off-chain
state and therefore remain in midnight-identity. NeoPRISM, Lace and Oxid supply
consumer-shaped identifiers only; Apollo has no DID parser. A new shared crate
or donor extraction would add coupling without reusable behavior. No donor code
or fixture is needed.

The fixed W3C Recommendation remains the grammar authority. Current W3C editor
and test-suite revisions are provenance/drift evidence, not moving normative
inputs. The repository-pinned nightly and Nixpkgs revision resolve cargo-fuzz
without an ambient install; exact `libfuzzer-sys` prevents silent runner-runtime
drift.

## Security and resource findings

Arbitrary bytes are appropriate input, but non-UTF-8 cannot reach an API that
accepts `&str` and should not be decoded lossily. An 8 KiB generator limit
crosses both production caps. Five-second per-input and 1 GiB process ceilings
contain pathological harness/compiler behavior while remaining generous for a
linear 4 KiB parser.

Accepted-value assertions must use public accessors, not internal offsets, so
the campaign protects the contract consumers actually receive. Pointer-range
checks plus exact delimiter-aware reconstruction expose stale or invalid ranges
without expanding the production API. Serde and owned/native constructors are
worth fuzzing because each is a public validation boundary.

## Automation and compatibility findings

Fixed-run PR smoke with one seed/worker is a reproducible regression search;
time-boxed scheduled soak is intentionally nondeterministic evidence. Keeping
those modes separate protects normal PR latency. Crash artifacts must upload
only on failure and be treated as untrusted, potentially sensitive data.

The independent fuzz workspace leaves all published dependencies, features,
public types, wire shapes, MSRV and cross-target production builds unchanged.
Linux sanitizer CI is sufficient for this bounded slice; the existing Nix
matrix retains cross-platform authority. No downstream edit, release,
publication, unsafe production code, repository setting, or `main` change is
justified.

## Implementation review

- **Reviewed head:** `f959fb289eb9f6c78bbc22eadf0e0fc274ba897c`
- **Reviewer:** fresh local second pass after the implementation was frozen
- **Result:** no semantic, security, API, licensing, or delivery blocker found

The exact `develop...f959fb2` diff changes no production parser, public API,
wire type, crate feature, root lock, consumer tree, or target-specific build.
The standalone manifest is an independent workspace with exact fuzz-runtime and
serde pins; its lock resolves 74 packages because `identus-did` currently owns
generic cryptographic document types, but that graph remains fuzz-only.

Both targets permit rejection and run every accepted value through the public
owned, borrowed, Display, `FromStr`, serde, and conversion paths. Component
range checks compare pointers only inside each value's own immutable string,
use checked arithmetic, and verify UTF-8 boundaries before reconstruction.
One-time resource probes hit exactly 2,048/4,096 bytes and one byte beyond.
Inputs, mutations, dictionaries, campaign growth, and artifacts cannot enter
panic text through custom messages; libFuzzer artifacts remain explicitly
untrusted.

The wrapper validates its mode, target, and exact cargo-fuzz version, enters the
locked Nix shell when needed, copies committed corpora to a temporary writable
directory, and removes that directory on exit. Fixed smoke options match the
contract; soak is separately time-boxed. The workflow is path-scoped, read-only,
uses immutable action revisions, runs the independent cargo-deny/RustSec gates,
and retains artifacts only on failure. The 20-minute job ceiling contains both
five-minute sequential soak targets plus build and audit time.

The NCSA license is OSI-approved and required only by LLVM's fuzz runtime. A
version-exact cargo-deny exception for `libfuzzer-sys@0.4.13` is materially
narrower than extending the global license allow-list. Cargo-deny and RustSec
both pass against the independent lock. No unsafe production code, ambient
installer, network/runtime capability, method policy, chain policy, custody,
trust, release, publication, repository setting, or `main` change appears.

## Corrections made during review

- Corpus seeds initially carried text-file newlines, causing every intended
  valid seed to be rejected. The corpus now stores exact raw bytes and has a
  narrowly scoped editorconfig exception.
- The first pointer-range assertion compared owned component pointers with the
  fuzzer input buffer. It now compares every component with its own validated
  value, and accepted corpus replay proves the paths execute.
- The independent lock exposed the fuzz runtime's NCSA license to cargo-deny.
  The package gained its own Apache-2.0 declaration and the runtime received a
  version-exact exception plus a dedicated license/advisory CI gate.

Residual risk remains explicit: 4,096 fixed mutations and a five-minute soak do
not prove absence of defects; compiler/sanitizer-specific findings need
minimization and deterministic regression. Continuous hosted services,
structure-aware method generation, and broader parser families are follow-up
work, not blockers for this roughly 85% lexical robustness target.

## Downstream immutability review

Post-implementation receipts equal the pre-implementation receipts:

| Repository | HEAD / branch | Preserved pre-existing status |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` / `main` | modified `secp256k1-kmp/native/secp256k1` submodule |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` / `main` | clean; behind origin by five |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` / `develop` | untracked `third_party/midnight-did` submodule state |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` / `main` | untracked `.pi-subagents/`, `.pi/`, `tmp/` |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` / `integration` | behind origin by twelve; untracked `.claude/`, `.pi/taskflows/` |

No downstream file, branch, index, worktree, submodule, or remote was changed.
The reserved sdk-rust main worktree remains clean at `2c267d65af5c`.
