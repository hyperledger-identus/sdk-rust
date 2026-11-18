import type {
  DidDocumentValue,
  DidValue,
  PeerDidNumalgo2CompatibilityValue,
  PeerDidNumalgo4ShortValue,
  PeerDidNumalgo4Value,
} from "../public.js";
import {
  bindingFailure,
  cloneFrozenJson,
  requireMap,
} from "./document-projection.js";
import type {
  RawPeerDidNumalgo2CompatibilityView,
  RawPeerDidNumalgo4ShortView,
  RawPeerDidNumalgo4View,
  RawOwnedValue,
} from "./raw-binding.js";

interface PeerProjectionCallbacks {
  readonly did: (serialized: string) => DidValue;
  readonly document: (serialized: string) => DidDocumentValue;
}

function requireString(value: unknown): string {
  if (typeof value !== "string") {
    throw bindingFailure();
  }
  return value;
}

function requireProfile(value: unknown, expected: string): void {
  if (value !== expected) {
    throw bindingFailure();
  }
}

function dispose(value: RawOwnedValue): void {
  try {
    value.free();
  } catch {
    throw bindingFailure();
  }
}

function projectInputDocument(serialized: string) {
  let parsed: unknown;
  try {
    parsed = JSON.parse(serialized) as unknown;
  } catch {
    throw bindingFailure();
  }
  return requireMap(cloneFrozenJson(parsed));
}

export function projectPeerDidNumalgo4(
  raw: RawPeerDidNumalgo4View,
  callbacks: PeerProjectionCallbacks,
): PeerDidNumalgo4Value {
  let longForm: string;
  let shortForm: string;
  let inputDocument: string;
  let resolvedDocument: string;
  try {
    requireProfile(raw.profile, "numalgo4");
    longForm = requireString(raw.longForm);
    shortForm = requireString(raw.shortForm);
    inputDocument = requireString(raw.inputDocument);
    resolvedDocument = requireString(raw.resolvedDocument);
  } finally {
    dispose(raw);
  }
  return Object.freeze({
    profile: "numalgo4",
    longForm: callbacks.did(longForm),
    shortForm: callbacks.did(shortForm),
    inputDocument: projectInputDocument(inputDocument),
    document: callbacks.document(resolvedDocument),
  });
}

export function projectPeerDidNumalgo4Short(
  raw: RawPeerDidNumalgo4ShortView,
  parseDid: (serialized: string) => DidValue,
): PeerDidNumalgo4ShortValue {
  let shortForm: string;
  try {
    requireProfile(raw.profile, "numalgo4");
    shortForm = requireString(raw.shortForm);
  } finally {
    dispose(raw);
  }
  return Object.freeze({
    profile: "numalgo4",
    shortForm: parseDid(shortForm),
  });
}

export function projectPeerDidNumalgo2Compatibility(
  raw: RawPeerDidNumalgo2CompatibilityView,
  callbacks: PeerProjectionCallbacks,
): PeerDidNumalgo2CompatibilityValue {
  let did: string;
  let resolvedDocument: string;
  try {
    requireProfile(raw.profile, "numalgo2-compatibility");
    did = requireString(raw.did);
    resolvedDocument = requireString(raw.resolvedDocument);
  } finally {
    dispose(raw);
  }
  return Object.freeze({
    profile: "numalgo2-compatibility",
    did: callbacks.did(did),
    document: callbacks.document(resolvedDocument),
  });
}
