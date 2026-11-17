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
