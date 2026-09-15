# Design: VCS-independent candidate assembly

## Boundary

The canonical SDK checkout remains the only source of package content and
source revision. Generated Cargo workspaces are build intermediates, not source
repositories and not evidence authorities.

## Build and output staging

Candidate preparation creates one build-scratch directory using the platform
temporary root without passing the requested output parent. It verifies that
the resolved scratch is not beneath the canonical repository, then creates the
two independent Cargo workspaces below that scratch. Cargo therefore does not
discover the SDK `.git` directory or embed stage-specific `path_in_vcs` values.

A second temporary directory is created beneath the requested output parent.
Verified packages, API evidence, SBOMs, and the receipt are copied there. The
completed directory is renamed to the requested output on the same filesystem,
preserving the existing atomic publication contract. Both temporary contexts
clean partial state on failure.

## Verification

A focused test exercises the repository-containment guard. The static candidate
policy requires independent build and output scratch construction so later
refactoring cannot silently place Cargo stages under the checkout again. The
existing real candidate command remains the end-to-end proof: it assembles each
package twice and compares native `.crate` bytes before closure/API/SBOM work.

## Failure and rollback

An output that already exists still fails before scratch creation. A temporary
root beneath the source repository fails before Cargo runs. Cross-filesystem
rename is avoided because completed output staging is destination-local.
Rollback is a repository revert and has no external-state cleanup.
