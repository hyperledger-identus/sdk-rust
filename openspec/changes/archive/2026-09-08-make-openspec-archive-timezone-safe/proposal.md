## Why

`scripts/factory archive` currently predicts OpenSpec's destination with the
host-local `date +%F`. OpenSpec 1.2.0 was observed using a different date basis:
at the UTC/WITA date boundary it completed a valid archive, after which the
wrapper falsely reported failure because it looked for the next local date.
An autonomous delivery receipt must observe the completed transition without
depending on host timezone or on the date remaining unchanged while the
command runs.

## What Changes

- Snapshot archive entries matching the requested change before OpenSpec runs.
- After OpenSpec returns, require exactly one new matching entry and require it
  to be a regular, non-symlink directory before accepting it as the archive.
- Retain fail-closed collision, no-op, symlink, mandatory-artifact,
  canonical-spec and whole-store validation protections.
- Add hermetic coverage in which fake OpenSpec chooses a date different from
  the wrapper's current date without changing the host clock.
- Update the factory documentation and canonical archive receipt contract.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `ai-software-factory`: archive completion is discovered from the requested
  before/after state transition rather than a predicted host-local path.

## Impact

This is a repository-local delivery-facade correction. It changes no Cargo
dependency, SDK public API, protocol, wire format, persisted consumer data,
supported target or downstream repository. A valid archive created under a
different date basis will now receive a truthful success result; ambiguous,
colliding, symlinked, incomplete and no-op outcomes continue to fail closed.
