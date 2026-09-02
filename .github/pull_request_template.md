# Pull request

## Outcome

<!-- What consumer or maintainer outcome does this PR produce? -->

## Issue and decision record

- Issue: <!-- Required: #123 -->
- Discussion/ADR:
- Owner crate:

## Factory contract

- OpenSpec change/archive:
- OpenSpec required: yes/no
- If no, exemption:
- Mandate/roadmap source:
- Agent roles used:
- `scripts/factory ready <change>`: passed/not applicable
- Factory receipt attached: yes/no/not applicable

## Local review

- Local review: <!-- Required: passed/completed, reviewer or review context -->
- Findings resolved:
- Specialist review required/completed:

## Scope and non-scope

- In scope:
- Explicitly out of scope:

## Sources and provenance

- Base SHA:
- Normative versions:
- Source repository/SHAs/paths/licenses:
- Fixture provenance updated: yes/no/not applicable

## Compatibility

- Public API:
- Serialized/wire behavior:
- Feature/MSRV/target impact:
- Migration or rollback:

## Security and privacy

- Threats/resource bounds considered:
- Secret/PII/FFI impact:
- Security review required/completed:

## Validation receipt

```text
commands passed:
commands not run and why:
coverage/conformance evidence:
factory receipt:
```

## Repository isolation

- Consumer repositories inspected:
- Consumer preflight HEAD/status:
- Consumer final HEAD/status:
- Consumer changed: no
- Downstream adoption issue:

## Checklist

- [ ] focused and independently reversible
- [ ] corresponding issue exists and is linked above
- [ ] distinct local review completed with no unresolved blocker
- [ ] OpenSpec contract complete or exemption recorded
- [ ] factory readiness and receipt recorded when applicable
- [ ] DCO and verified signatures
- [ ] tests, negative cases and docs updated
- [ ] provenance/license recorded
- [ ] public/wire compatibility reviewed
- [ ] no chain or product dependency in generic crates
- [ ] no raw secret material in logs/errors/serialization/FFI
- [ ] release notes/migration updated when applicable
