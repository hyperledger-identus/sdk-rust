# oid4vci-authorization-code-token-correlation Specification

## ADDED Requirements

### Requirement: Correlated dataset authority has a consuming request continuation

The correlated state SHALL expose a consuming continuation into bounded
authorized-dataset JWT Credential Request construction and SHALL expose no
metadata, token, configuration or raw-identifier replacement input.

#### Scenario: continuation preserves least authority

- **WHEN** one correlated state advances
- **THEN** only its retained endpoint, token and authorized identifier can
  appear in the resulting request
