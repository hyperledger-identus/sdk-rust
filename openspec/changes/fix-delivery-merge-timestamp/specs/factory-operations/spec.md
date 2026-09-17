# factory-operations delta

## ADDED Requirements

### Requirement: Hosted merge timestamps use a closed interoperable UTC grammar

The factory SHALL accept canonical RFC 3339 UTC merge timestamps with whole-
second precision or one to nine fractional-second digits. It SHALL preserve
the exact hosted value in the receipt and SHALL reject invalid calendar/time
values, numeric offsets, missing or lowercase zones, whitespace, excessive
fraction precision, and trailing content.

#### Scenario: GitHub returns whole-second UTC time

- **WHEN** the exact merged PR reports a valid timestamp such as
  `2026-09-17T09:48:40Z`
- **THEN** post-merge verification may retain the exact value in its receipt

#### Scenario: Timestamp text normalizes an invalid or broader value

- **WHEN** the hosted value uses an invalid date/time, numeric offset, missing
  UTC marker, excessive precision, whitespace, or trailing content
- **THEN** delivery fails without retaining or overwriting a receipt
