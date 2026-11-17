const RESOLUTION_DID = "did:example:123";
const DEREFERENCING_DID_URL = "did:example:123#key-1";

function assertEqual(actual, expected, label) {
  if (!Object.is(actual, expected)) {
    throw new Error(`DID Resolution vector assertion failed: ${label}`);
  }
}

function assertDeepFrozen(value, label) {
  const pending = [value];
  const visited = new Set();
  let nodes = 0;
  while (pending.length > 0) {
    const current = pending.pop();
    if (current === null || typeof current !== "object" || visited.has(current)) continue;
    visited.add(current);
    nodes += 1;
    if (nodes > 16_384) throw new Error(`DID Resolution vector assertion failed: ${label}:node-budget`);
    assertEqual(Object.isFrozen(current), true, `${label}:frozen`);
    pending.push(...Object.values(current));
  }
}

async function invokeVector(createProvider, vector) {
  const input = vector.input.literal;
  const effects = vector.operation === "did-resolution.parse"
    ? { resolver: { resolve: async () => input } }
    : { dereferencer: { dereference: async () => input } };
  const provider = await createProvider(effects);
  return vector.operation === "did-resolution.parse"
    ? provider.resolveDid(RESOLUTION_DID)
    : provider.dereferenceDidUrl(DEREFERENCING_DID_URL);
}

export async function assertResolutionProviderCase(createProvider, vector) {
  if (vector.expected.kind === "error") {
    let thrown;
    try {
      await invokeVector(createProvider, vector);
    } catch (error) {
      thrown = error;
    }
    assertEqual(thrown !== undefined, true, `${vector.id}:rejected`);
    assertEqual(thrown.code, vector.expected.errorCode, `${vector.id}:error-code`);
    assertEqual(thrown.stack, undefined, `${vector.id}:stack-redacted`);
    assertEqual(
      JSON.stringify(thrown).includes(vector.input.literal),
      false,
      `${vector.id}:input-redacted`,
    );
    return;
  }

  const result = await invokeVector(createProvider, vector);
  assertDeepFrozen(result, vector.id);
  const expectedKind = vector.expected.fields.resultKind;
  const projectedKind = result.kind === "failure"
    ? "error"
    : vector.operation === "did-url-dereferencing.parse"
      ? "dereferenced"
      : result.kind;
  assertEqual(projectedKind, expectedKind, `${vector.id}:result-kind`);
  if (vector.expected.fields.did !== undefined) {
    assertEqual(
      result.kind === "success" ? result.document.id.serialized : undefined,
      vector.expected.fields.did,
      `${vector.id}:did`,
    );
  }
}

export async function assertAllResolutionVectors(createProvider, vectors) {
  for (const vector of vectors) {
    await assertResolutionProviderCase(createProvider, vector);
  }
}
