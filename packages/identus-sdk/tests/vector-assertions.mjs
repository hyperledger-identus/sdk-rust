import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const vectorsUrl = new URL(
  "../../../docs/conformance/fixtures/did/v1/vectors.json",
  import.meta.url,
);

export async function loadVectors() {
  const document = JSON.parse(await readFile(vectorsUrl, "utf8"));
  assert.equal(document.packetId, "did.syntax.v1");
  assert.equal(document.cases.length, 22);
  return document.cases;
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
    assert.throws(invoke, (error) => {
      assert.equal(error.code, vector.expected.errorCode, vector.id);
      assert.equal(error.stack, undefined, vector.id);
      assert.equal(JSON.stringify(error).includes(input), false, vector.id);
      return true;
    });
    return;
  }

  const result = invoke();
  assert.equal(Object.isFrozen(result), true, vector.id);
  for (const [field, expected] of Object.entries(vector.expected.fields)) {
    assert.equal(result[field], expected === null ? undefined : expected, `${vector.id}:${field}`);
  }
}

export async function assertAllVectors(provider) {
  for (const vector of await loadVectors()) {
    assertProviderCase(provider, vector);
  }
}
