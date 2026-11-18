import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const vectorsUrl = new URL(
  "../../../docs/conformance/fixtures/did-peer/v1/vectors.json",
  import.meta.url,
);

export async function loadPeerVectors() {
  const document = JSON.parse(await readFile(vectorsUrl, "utf8"));
  assert.equal(document.schemaVersion, 1);
  assert.equal(document.packetId, "did.peer.profiles.v1");
  assert.equal(document.capability, "did.peer.profiles");
  assert.equal(document.cases.length, 13);
  assert.equal(new Set(document.cases.map(({ id }) => id)).size, 13);
  return document.cases;
}
