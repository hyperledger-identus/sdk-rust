function assertEqual(actual, expected, label) {
  if (!Object.is(actual, expected)) {
    throw new Error(`DID vector assertion failed: ${label}`);
  }
}

export function materializeInput(input) {
  if (typeof input.literal === "string") {
    return input.literal;
  }
  const { prefix, repeat, count, suffix } = input.generated;
  return `${prefix}${repeat.repeat(count)}${suffix}`;
}

export function assertProviderCase(provider, vector) {
  const input = materializeInput(vector.input);
  const invoke = vector.operation === "did.parse"
    ? () => provider.parseDid(input)
    : () => provider.parseDidUrl(input);

  if (vector.expected.kind === "error") {
    let thrown;
    try {
      invoke();
    } catch (error) {
      thrown = error;
    }
    assertEqual(thrown !== undefined, true, `${vector.id}:rejected`);
    assertEqual(thrown.code, vector.expected.errorCode, `${vector.id}:error-code`);
    assertEqual(thrown.stack, undefined, `${vector.id}:stack-redacted`);
    assertEqual(JSON.stringify(thrown).includes(input), false, `${vector.id}:input-redacted`);
    return;
  }

  const result = invoke();
  assertEqual(Object.isFrozen(result), true, `${vector.id}:frozen`);
  for (const [field, expected] of Object.entries(vector.expected.fields)) {
    assertEqual(result[field], expected === null ? undefined : expected, `${vector.id}:${field}`);
  }
}

export function assertAllVectors(provider, vectors) {
  for (const vector of vectors) {
    assertProviderCase(provider, vector);
  }
}
