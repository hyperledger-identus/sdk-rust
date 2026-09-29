# weekly-slow-evidence

## ADDED Requirements

### Requirement: Sequential candidate lanes preserve a clean source checkout

When multiple candidate qualification lanes share one Git checkout, generated evidence SHALL be written outside that checkout. The workflow SHALL explicitly
verify source cleanliness between lanes and SHALL retain the same bounded,
attempt-scoped artifact identity and retention policy after relocating output.
It SHALL NOT weaken the candidate builder's independent clean-source check.

#### Scenario: Primary qualification completes before MSRV

- **WHEN** the primary lane emits its candidate receipt
- **THEN** the receipt is below runner-temporary storage and Git porcelain
  status remains empty before MSRV qualification starts

#### Scenario: A lane contaminates the checkout

- **WHEN** primary qualification creates or modifies any tracked, staged, or
  untracked checkout path
- **THEN** the explicit boundary fails before MSRV execution rather than
  deleting, ignoring, or accepting the mutation

#### Scenario: Matrix evidence is uploaded

- **WHEN** both compiler lanes finish successfully
- **THEN** the workflow uploads the runner-temporary matrix directory under the
  unchanged host/SHA/attempt artifact name with seven-day retention
