import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const vectorsUrl = new URL(
  "../../../docs/conformance/fixtures/did-document/v1/vectors.json",
  import.meta.url,
);

export async function loadDocumentVectors() {
  const document = JSON.parse(await readFile(vectorsUrl, "utf8"));
  assert.equal(document.schemaVersion, 1);
  assert.equal(document.packetId, "did.document.v1");
  assert.equal(document.capability, "did.document");
  assert.equal(Array.isArray(document.cases), true);
  assert.equal(document.cases.length > 0, true);
  for (const vector of document.cases) {
    assert.equal(vector.operation, "did-document.parse", vector.id);
  }
  return document.cases;
}
