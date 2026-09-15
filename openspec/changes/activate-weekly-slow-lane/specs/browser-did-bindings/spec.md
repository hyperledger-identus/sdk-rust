## MODIFIED Requirements

### Requirement: Two browser engines execute API version one

The package SHALL expose binding API version `1`. Headless Chromium and Firefox
SHALL execute matching valid, invalid, oversized, redaction, component and
version tests in the active hosted weekly/manual slow Ubuntu job or its exact
local reproduction. Ordinary Linux pull requests SHALL retain the existing fast
factory/build/lint/test line.

#### Scenario: slow browser evidence runs

- **WHEN** the browser job is invoked by the native weekly/manual workflow or
  reproduced locally
- **THEN** both engines SHALL pass the same behavior families or the job SHALL
  fail without converting one engine's evidence into the other's
