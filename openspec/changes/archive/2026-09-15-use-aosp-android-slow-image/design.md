## Context

ADR 0100 requires one fresh arm64 API-35 AVD but does not require Google APIs or
Play Store. The implementation chose `google_apis_playstore`; the hosted runner
did not hold its distinct ARM translation/system-image license. GitHub Actions
correctly stopped at an interactive prompt.

## Decisions

### Minimize the emulator dependency

Use stable package `system-images;android-35;default;arm64-v8a`. Google's AOSP
catalog identifies revision 2, API 35, ABI arm64-v8a, tag `default`, dependency
on emulator >=29.1.11 and `android-sdk-license`. The existing Play catalog
identifies the old selection as Google Play revision 9 under distinct
`android-sdk-arm-dbt-license`.

The workflow installs the AOSP package with the already exact NDK and platform.
The verifier's package string, image directory and `avdmanager --package`
argument use the identical tag. No compatibility fallback is allowed because a
fallback could silently test a different runtime.

### Do not accept licenses broadly in automation

The workflow does not run `sdkmanager --licenses` or pipe blanket acceptance.
If the runner does not already satisfy the selected package's ordinary license,
installation fails closed and maintainers reconsider runner provisioning. This
avoids converting a technical canary repair into acceptance of every license
known to the SDK installation.

### Preserve evidence and support boundaries

The change affects only external test infrastructure. API level, ABI, NDK,
minimum application API, AAR content, JNA version and behavior cases are
unchanged. The image remains a moving package revision behind an exact SDK path;
the run receipt records installed tool/package versions, and a future hermetic
image pin remains separate work.

## Risks and mitigations

- AOSP could differ from Play in irrelevant services: the smoke test has no
  Google service/library declaration and exercises only SDK-owned/JNA behavior.
- The hosted runner may also lack the ordinary license: fail closed rather than
  accept legal terms automatically.
- Package revision can move: exact path, stable channel, hosted receipt and
  deterministic SDK artifacts bound what is tested; this is not hermetic image
  provenance.
- Installation and execution could drift: one checker validates the workflow,
  verifier constant, directory tag and negative mutations together.

## Alternatives

- Blanket `yes | sdkmanager --licenses`: rejected because it accepts unrelated
  present/future licenses and requires broader authority.
- Accept only the Google Play license hash: rejected because Google Play is not
  a test requirement and hard-coded acceptance files are opaque/brittle.
- Retain Google Play and inject interaction: rejected for the same unnecessary
  dependency and legal coupling.
- Remove Android runtime execution: rejected because it would weaken accepted
  native-binding evidence.

## Rollback

Revert ADR 0122, workflow/verifier identity, policy checks and capability delta.
The macOS Android runtime job returns to known-red without changing Rust or
released consumer state.
