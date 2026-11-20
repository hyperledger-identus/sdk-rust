import type {
  DidDocumentValue,
  DidValue,
  PrismDidStateValue,
  PrismDidValue,
} from "../public.js";
import { bindingFailure } from "./document-projection.js";
import type {
  RawOwnedValue,
  RawPrismDidStateView,
  RawPrismDidView,
} from "./raw-binding.js";

interface PrismProjectionCallbacks {
  readonly did: (serialized: string) => DidValue;
  readonly document: (serialized: string) => DidDocumentValue;
}

function requireString(value: unknown): string {
  if (typeof value !== "string") {
    throw bindingFailure();
  }
  return value;
}

function requireOptionalString(value: unknown): string | undefined {
  if (value === undefined || typeof value === "string") {
    return value;
  }
  throw bindingFailure();
}

function dispose(value: RawOwnedValue): void {
  try {
    value.free();
  } catch {
    throw bindingFailure();
  }
}

function project<T>(callback: () => T): T {
  try {
    return callback();
  } catch {
    throw bindingFailure();
  }
}

export function projectPrismDid(
  raw: RawPrismDidView,
  callbacks: PrismProjectionCallbacks,
): PrismDidValue {
  let requestedDid: string;
  let canonicalDid: string;
  let form: "canonical" | "long-form";
  let resolvedDocument: string | undefined;
  try {
    requestedDid = requireString(raw.requestedDid);
    canonicalDid = requireString(raw.canonicalDid);
    if (raw.form !== "canonical" && raw.form !== "long-form") {
      throw bindingFailure();
    }
    form = raw.form;
    resolvedDocument = requireOptionalString(raw.resolvedDocument);
  } finally {
    dispose(raw);
  }
  return Object.freeze({
    form,
    did: project(() => callbacks.did(requestedDid)),
    canonicalDid: project(() => callbacks.did(canonicalDid)),
    document: resolvedDocument === undefined
      ? undefined
      : project(() => callbacks.document(resolvedDocument)),
  });
}

export function projectPrismDidState(
  raw: RawPrismDidStateView,
  callbacks: PrismProjectionCallbacks,
): PrismDidStateValue {
  let requestedDid: string;
  let canonicalDid: string;
  let deactivated: boolean;
  let resolvedDocument: string | undefined;
  try {
    requestedDid = requireString(raw.requestedDid);
    canonicalDid = requireString(raw.canonicalDid);
    if (typeof raw.deactivated !== "boolean") {
      throw bindingFailure();
    }
    deactivated = raw.deactivated;
    resolvedDocument = requireOptionalString(raw.resolvedDocument);
  } finally {
    dispose(raw);
  }
  const did = project(() => callbacks.did(requestedDid));
  const canonical = project(() => callbacks.did(canonicalDid));
  if (deactivated) {
    if (resolvedDocument !== undefined) {
      throw bindingFailure();
    }
    return Object.freeze({
      status: "deactivated",
      did,
      canonicalDid: canonical,
      document: undefined,
    });
  }
  if (resolvedDocument === undefined) {
    throw bindingFailure();
  }
  return Object.freeze({
    status: "active",
    did,
    canonicalDid: canonical,
    document: project(() => callbacks.document(resolvedDocument)),
  });
}
