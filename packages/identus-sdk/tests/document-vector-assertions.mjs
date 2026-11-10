function assertEqual(actual, expected, label) {
  if (!Object.is(actual, expected)) {
    throw new Error(`DID Document vector assertion failed: ${label}`);
  }
}

function assertDeepEqual(actual, expected, label) {
  if (Object.is(actual, expected)) return;
  if (actual === null || expected === null || typeof actual !== "object" || typeof expected !== "object") {
    throw new Error(`DID Document vector assertion failed: ${label}`);
  }
  const actualKeys = Object.keys(actual).sort();
  const expectedKeys = Object.keys(expected).sort();
  assertEqual(JSON.stringify(actualKeys), JSON.stringify(expectedKeys), `${label}:keys`);
  for (const key of expectedKeys) assertDeepEqual(actual[key], expected[key], `${label}.${key}`);
}

function assertDeepFrozen(value, label) {
  const pending = [{ value, label }];
  const visited = new Set();
  let nodes = 0;
  while (pending.length > 0) {
    const current = pending.pop();
    if (current.value === null || typeof current.value !== "object" || visited.has(current.value)) continue;
    visited.add(current.value);
    nodes += 1;
    if (nodes > 16_384) throw new Error(`DID Document vector assertion failed: ${label}:node-budget`);
    assertEqual(Object.isFrozen(current.value), true, `${current.label}:frozen`);
    for (const [key, child] of Object.entries(current.value)) {
      pending.push({ value: child, label: `${current.label}.${key}` });
    }
  }
}

function materializeInput(input) {
  if (typeof input.literal === "string") return input.literal;
  const { prefix, repeat, count, suffix } = input.generated;
  return `${prefix}${repeat.repeat(count)}${suffix}`;
}

function projectedField(result, field) {
  if (field !== "did") return result[field];
  if (typeof result.id === "string") return result.id;
  return result.id?.serialized;
}

export function assertDocumentProviderCase(provider, vector) {
  const input = materializeInput(vector.input);
  const invoke = () => provider.parseDidDocument(input);

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
  assertDeepFrozen(result, vector.id);
  for (const [field, expected] of Object.entries(vector.expected.fields)) {
    assertDeepEqual(projectedField(result, field), expected === null ? undefined : expected, `${vector.id}:${field}`);
  }
}

export function assertAllDocumentVectors(provider, vectors) {
  for (const vector of vectors) assertDocumentProviderCase(provider, vector);
}
