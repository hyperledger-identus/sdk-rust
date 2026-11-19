import {
  DidError,
  type DidDereferencingRequestValue,
  type DidDereferencingOptions,
  type DidQueryEffects,
  type DidDocumentValue,
  type DidErrorCode,
  type DidMappingId,
  type DidObservation,
  type DidOperation,
  type DidProviderOptions,
  type DidProviderRoute,
  type PeerDidProvider,
  type PeerDidQueryProvider,
  type DidOperationContext,
  type DidResolutionOptions,
  type DidResolutionRequestValue,
  type DidResolutionResultValue,
  type DidUrlDereferencingResultValue,
  type DidUrlValue,
  type DidValue,
  type PeerDidNumalgo2CompatibilityInput,
  type PeerDidNumalgo2CompatibilityValue,
  type PeerDidNumalgo4InputDocument,
  type PeerDidNumalgo4ShortValue,
  type PeerDidNumalgo4Value,
} from "../public.js";
import { projectDidDocument as projectDidDocumentValue } from "./document-projection.js";
import {
  projectDereferencingOptions,
  projectDidDereferencingResult,
  projectDidResolutionResult,
  projectResolutionOptions,
} from "./resolution-projection.js";
import {
  serializeDereferencingOptions,
  serializeResolutionOptions,
} from "./resolution-options.js";
import type {
  RawDidBinding,
  RawDidDereferencingRequestView,
  RawDidDereferencingResultView,
  RawDidDocumentView,
  RawDidParseError,
  RawDidResolutionRequestView,
  RawDidResolutionResultView,
  RawDidUrlView,
  RawDidView,
  RawOwnedValue,
} from "./raw-binding.js";
import {
  serializePeerDidNumalgo2Input,
  serializePeerDidNumalgo4Input,
  requirePeerDidNumalgo2Input,
  requirePeerDidNumalgo4LongInput,
  requirePeerDidNumalgo4ShortInput,
} from "./peer-input.js";
import {
  projectPeerDidNumalgo2Compatibility,
  projectPeerDidNumalgo4,
  projectPeerDidNumalgo4Short,
} from "./peer-projection.js";

const BINDING_API_VERSION = 4;
const MAX_OBSERVED_DURATION_MS = 2_147_483_647;
const MAX_DID_RESOLUTION_RESULT_BYTES = 512 * 1_024;

interface ProviderInternals {
  readonly now?: () => number;
}

function bindingFailure(): Error {
  const error = new Error("DID binding contract failed");
  error.name = "DidBindingError";
  Object.defineProperty(error, "stack", {
    configurable: false,
    enumerable: false,
    value: undefined,
    writable: false,
  });
  return Object.freeze(error);
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

function projectDid(raw: RawDidView): DidValue {
  try {
    return Object.freeze({
      serialized: requireString(raw.value),
      method: requireString(raw.method),
      methodSpecificId: requireString(raw.methodSpecificId),
    });
  } finally {
    dispose(raw);
  }
}

function projectDidUrl(raw: RawDidUrlView): DidUrlValue {
  try {
    return Object.freeze({
      serialized: requireString(raw.value),
      did: requireString(raw.did),
      method: requireString(raw.method),
      methodSpecificId: requireString(raw.methodSpecificId),
      path: requireString(raw.path),
      query: requireOptionalString(raw.query),
      fragment: requireOptionalString(raw.fragment),
    });
  } finally {
    dispose(raw);
  }
}

function projectDidDocument(raw: RawDidDocumentView): DidDocumentValue {
  try {
    return projectDidDocumentValue(
      requireString(raw.serialized),
      requireString(raw.id),
    );
  } finally {
    dispose(raw);
  }
}

function projectResolutionRequest(
  raw: RawDidResolutionRequestView,
  signal: AbortSignal | undefined,
): DidResolutionRequestValue {
  try {
    return Object.freeze({
      did: Object.freeze({
        serialized: requireString(raw.did),
        method: requireString(raw.method),
        methodSpecificId: requireString(raw.methodSpecificId),
      }),
      options: projectResolutionOptions(requireString(raw.options)),
      signal,
    });
  } finally {
    dispose(raw);
  }
}

function projectDereferencingRequest(
  raw: RawDidDereferencingRequestView,
  signal: AbortSignal | undefined,
): DidDereferencingRequestValue {
  try {
    return Object.freeze({
      didUrl: Object.freeze({
        serialized: requireString(raw.didUrl),
        did: requireString(raw.did),
        method: requireString(raw.method),
        methodSpecificId: requireString(raw.methodSpecificId),
        path: requireString(raw.path),
        query: requireOptionalString(raw.query),
        fragment: requireOptionalString(raw.fragment),
      }),
      options: projectDereferencingOptions(requireString(raw.options)),
      signal,
    });
  } finally {
    dispose(raw);
  }
}

function projectResolutionResult(
  raw: RawDidResolutionResultView,
): DidResolutionResultValue {
  try {
    return projectDidResolutionResult(
      requireString(raw.serialized),
      requireString(raw.kind),
    );
  } finally {
    dispose(raw);
  }
}

function projectDereferencingResult(
  raw: RawDidDereferencingResultView,
): DidUrlDereferencingResultValue {
  try {
    return projectDidDereferencingResult(
      requireString(raw.serialized),
      requireString(raw.kind),
    );
  } finally {
    dispose(raw);
  }
}

function readErrorCode(raw: RawDidParseError): DidErrorCode {
  let code: unknown;
  try {
    code = raw.code;
  } finally {
    dispose(raw);
  }
  if (
    code === "did.invalid_did"
    || code === "did.invalid_did_url"
    || code === "did.invalid_document"
    || code === "did.invalid_resolution_options"
    || code === "did.invalid_dereferencing_options"
    || code === "did.invalid_resolution_result"
    || code === "did.invalid_dereferencing_result"
    || code === "did.peer.wrong_profile"
    || code === "did.peer.unsupported_algorithm"
    || code === "did.peer.malformed_encoding"
    || code === "did.peer.unsupported_codec"
    || code === "did.peer.unsupported_purpose"
    || code === "did.peer.invalid_input_document"
    || code === "did.peer.hash_mismatch"
    || code === "did.peer.long_form_required"
    || code === "did.peer.resource_limit"
    || code === "did.peer.projection_failure"
    || code === "adapter.unsupported_lossy_mapping"
  ) {
    return code;
  }
  throw bindingFailure();
}

function mappedError(binding: RawDidBinding, error: unknown): Error {
  try {
    if (!binding.isParseError(error)) {
      return bindingFailure();
    }
    return new DidError(readErrorCode(error));
  } catch {
    return bindingFailure();
  }
}

function boundedDuration(start: number, end: number): number {
  if (!Number.isFinite(start) || !Number.isFinite(end)) {
    return 0;
  }
  return Math.min(
    MAX_OBSERVED_DURATION_MS,
    Math.max(0, Math.round(end - start)),
  );
}

function notify(
  observer: DidProviderOptions["observer"],
  observation: DidObservation,
): void {
  if (observer === undefined) {
    return;
  }
  try {
    observer(Object.freeze(observation));
  } catch {
    // Observation is deliberately isolated from parsing and ownership.
  }
}

function successMapping(operation: DidOperation): DidMappingId {
  switch (operation) {
    case "did.parse":
      return "did.value.typescript.legacy-v1";
    case "did-url.parse":
      return "did-url.value.typescript.legacy-v1";
    case "did-document.parse":
      return "did.document.typescript.legacy-v1";
    case "did.resolve":
    case "did-url.dereference":
      return "did.resolution.typescript.host-v1";
    case "did.peer.numalgo4.create":
    case "did.peer.numalgo4.parse-long":
    case "did.peer.numalgo4.parse-short":
    case "did.peer.numalgo4.resolve-short":
      return "did.peer.typescript.numalgo4-v1";
    case "did.peer.numalgo2-compatibility.create":
    case "did.peer.numalgo2-compatibility.parse":
      return "did.peer.typescript.numalgo2-compat-v1";
  }
}

function errorMapping(code: DidErrorCode): DidMappingId {
  switch (code) {
    case "did.invalid_did":
      return "did.error.invalid-did.typescript.legacy-v1";
    case "did.invalid_did_url":
      return "did.error.invalid-did-url.typescript.legacy-v1";
    case "did.invalid_document":
      return "did.document.typescript.legacy-v1";
    case "adapter.unsupported_lossy_mapping":
      return "did.document.typescript.legacy-v1";
    case "did.invalid_resolution_options":
    case "did.invalid_dereferencing_options":
    case "did.invalid_resolution_result":
    case "did.invalid_dereferencing_result":
      return "did.resolution.typescript.host-v1";
    case "did.peer.wrong_profile":
    case "did.peer.unsupported_algorithm":
    case "did.peer.malformed_encoding":
    case "did.peer.unsupported_codec":
    case "did.peer.unsupported_purpose":
    case "did.peer.invalid_input_document":
    case "did.peer.hash_mismatch":
    case "did.peer.long_form_required":
    case "did.peer.resource_limit":
    case "did.peer.projection_failure":
      return "did.peer.typescript.numalgo4-v1";
    case "did.resolution_host_failure":
    case "did.dereferencing_host_failure":
    case "did.resolution_provider_unavailable":
    case "did.dereferencing_provider_unavailable":
    case "did.operation_aborted":
    case "did.observer_failure":
    case "adapter.binding_contract":
      return "did.resolution.typescript.host-v1";
  }
}

function requireInput(value: string): void {
  if (typeof value !== "string") {
    throw new TypeError("DID input must be a string");
  }
}

function hasBoundedUtf8Length(value: string): boolean {
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const codeUnit = value.charCodeAt(index);
    if (codeUnit <= 0x7f) {
      bytes += 1;
    } else if (codeUnit <= 0x7ff) {
      bytes += 2;
    } else if (
      codeUnit >= 0xd800
      && codeUnit <= 0xdbff
      && index + 1 < value.length
    ) {
      const next = value.charCodeAt(index + 1);
      if (next >= 0xdc00 && next <= 0xdfff) {
        bytes += 4;
        index += 1;
      } else {
        bytes += 3;
      }
    } else {
      // Unpaired surrogates are encoded as the three-byte replacement
      // character by the JavaScript/WASM string boundary.
      bytes += 3;
    }
    if (bytes > MAX_DID_RESOLUTION_RESULT_BYTES) {
      return false;
    }
  }
  return true;
}

function requireBoundedHostResult(
  value: string,
  errorCode: Extract<
    DidErrorCode,
    "did.invalid_resolution_result" | "did.invalid_dereferencing_result"
  >,
): void {
  if (!hasBoundedUtf8Length(value)) {
    throw new DidError(errorCode);
  }
}

export function createRustDidProvider(
  binding: RawDidBinding,
  route: Exclude<DidProviderRoute, "legacy">,
  options: DidProviderOptions = {},
  internals: ProviderInternals = {},
): PeerDidProvider {
  if (binding.bindingApiVersion() !== BINDING_API_VERSION) {
    throw bindingFailure();
  }
  const now = internals.now ?? Date.now;

  function run<T>(
    operation: DidOperation,
    invoke: () => T,
  ): T {
    const start = now();
    try {
      const result = invoke();
      notify(options.observer, {
        operation,
        route,
        result: "success",
        mappingId: successMapping(operation),
        errorCode: undefined,
        durationMs: boundedDuration(start, now()),
      });
      return result;
    } catch (error) {
      const mapped = error instanceof DidError ? error : mappedError(binding, error);
      notify(options.observer, {
        operation,
        route,
        result: "error",
        mappingId: mapped instanceof DidError
          ? errorMapping(mapped.code)
          : successMapping(operation),
        errorCode: mapped instanceof DidError ? mapped.code : undefined,
        durationMs: boundedDuration(start, now()),
      });
      throw mapped;
    }
  }

  function peerCallbacks() {
    return {
      did: (serialized: string) => projectDid(binding.parseDid(serialized)),
      document: (serialized: string) =>
        projectDidDocument(binding.parseDidDocument(serialized)),
    };
  }

  function runPeer<T>(operation: Extract<DidOperation,
    | "did.peer.numalgo4.create"
    | "did.peer.numalgo4.parse-long"
    | "did.peer.numalgo4.parse-short"
    | "did.peer.numalgo4.resolve-short"
    | "did.peer.numalgo2-compatibility.create"
    | "did.peer.numalgo2-compatibility.parse"
  >, invoke: () => T): T {
    const start = now();
    let result: T;
    try {
      result = invoke();
    } catch (error) {
      const mapped = error instanceof DidError ? error : mappedError(binding, error);
      strictNotify(options.observer, {
        operation,
        route,
        result: "error",
        mappingId: successMapping(operation),
        errorCode: mapped instanceof DidError ? mapped.code : undefined,
        durationMs: boundedDuration(start, now()),
      });
      throw mapped;
    }
    strictNotify(options.observer, {
      operation,
      route,
      result: "success",
      mappingId: successMapping(operation),
      errorCode: undefined,
      durationMs: boundedDuration(start, now()),
    });
    return result;
  }

  return Object.freeze({
    parseDid(value: string): DidValue {
      requireInput(value);
      return run("did.parse", () => projectDid(binding.parseDid(value)));
    },
    parseDidUrl(value: string): DidUrlValue {
      requireInput(value);
      return run("did-url.parse", () => projectDidUrl(binding.parseDidUrl(value)));
    },
    parseDidDocument(value: string): DidDocumentValue {
      requireInput(value);
      return run(
        "did-document.parse",
        () => projectDidDocument(binding.parseDidDocument(value)),
      );
    },
    createPeerDidNumalgo4(
      input: PeerDidNumalgo4InputDocument,
    ): PeerDidNumalgo4Value {
      return runPeer("did.peer.numalgo4.create", () =>
        projectPeerDidNumalgo4(
          binding.createPeerDidNumalgo4(serializePeerDidNumalgo4Input(input)),
          peerCallbacks(),
        ));
    },
    parsePeerDidNumalgo4Long(value: string): PeerDidNumalgo4Value {
      requirePeerDidNumalgo4LongInput(value);
      return runPeer("did.peer.numalgo4.parse-long", () =>
        projectPeerDidNumalgo4(
          binding.parsePeerDidNumalgo4Long(value),
          peerCallbacks(),
        ));
    },
    parsePeerDidNumalgo4Short(value: string): PeerDidNumalgo4ShortValue {
      requirePeerDidNumalgo4ShortInput(value);
      return runPeer("did.peer.numalgo4.parse-short", () =>
        projectPeerDidNumalgo4Short(
          binding.parsePeerDidNumalgo4Short(value),
          (serialized) => projectDid(binding.parseDid(serialized)),
        ));
    },
    resolvePeerDidNumalgo4Short(
      shortForm: string,
      longForm: string,
    ): PeerDidNumalgo4Value {
      requirePeerDidNumalgo4ShortInput(shortForm);
      return runPeer("did.peer.numalgo4.resolve-short", () => {
        if (typeof longForm !== "string" || longForm.length === 0) {
          throw new DidError("did.peer.long_form_required");
        }
        requirePeerDidNumalgo4LongInput(longForm);
        return projectPeerDidNumalgo4(
          binding.resolvePeerDidNumalgo4Short(shortForm, longForm),
          peerCallbacks(),
        );
      });
    },
    createPeerDidNumalgo2Compatibility(
      input: PeerDidNumalgo2CompatibilityInput,
    ): PeerDidNumalgo2CompatibilityValue {
      return runPeer("did.peer.numalgo2-compatibility.create", () =>
        projectPeerDidNumalgo2Compatibility(
          binding.createPeerDidNumalgo2Compatibility(
            serializePeerDidNumalgo2Input(input),
          ),
          peerCallbacks(),
        ));
    },
    parsePeerDidNumalgo2Compatibility(
      value: string,
    ): PeerDidNumalgo2CompatibilityValue {
      requirePeerDidNumalgo2Input(value);
      return runPeer("did.peer.numalgo2-compatibility.parse", () =>
        projectPeerDidNumalgo2Compatibility(
          binding.parsePeerDidNumalgo2Compatibility(value),
          peerCallbacks(),
        ));
    },
  });
}

function strictNotify(
  observer: DidProviderOptions["observer"],
  observation: DidObservation,
): void {
  if (observer === undefined) {
    return;
  }
  try {
    observer(Object.freeze(observation));
  } catch {
    throw new DidError("did.observer_failure");
  }
}

function isAborted(signal: AbortSignal | undefined): boolean {
  if (signal === undefined) {
    return false;
  }
  try {
    return signal.aborted === true;
  } catch {
    throw new DidError("adapter.binding_contract");
  }
}

function mapQueryBindingError(binding: RawDidBinding, error: unknown): DidError {
  const mapped = mappedError(binding, error);
  return mapped instanceof DidError
    ? mapped
    : new DidError("adapter.binding_contract");
}

export function createRustDidQueryProvider(
  binding: RawDidBinding,
  route: Exclude<DidProviderRoute, "legacy">,
  effects: DidQueryEffects,
  options: DidProviderOptions = {},
  internals: ProviderInternals = {},
): PeerDidQueryProvider {
  const base = createRustDidProvider(binding, route, options, internals);
  const now = internals.now ?? Date.now;

  async function runQuery<T>(
    operation: Extract<DidOperation, "did.resolve" | "did-url.dereference">,
    invoke: () => Promise<T>,
  ): Promise<T> {
    const start = now();
    let result: T;
    try {
      result = await invoke();
    } catch (error) {
      const mapped = error instanceof DidError
        ? error
        : new DidError("adapter.binding_contract");
      strictNotify(options.observer, {
        operation,
        route,
        result: "error",
        mappingId: successMapping(operation),
        errorCode: mapped.code,
        durationMs: boundedDuration(start, now()),
      });
      throw mapped;
    }
    strictNotify(options.observer, {
      operation,
      route,
      result: "success",
      mappingId: successMapping(operation),
      errorCode: undefined,
      durationMs: boundedDuration(start, now()),
    });
    return result;
  }

  return Object.freeze({
    ...base,
    resolveDid(
      did: string,
      resolutionOptions?: DidResolutionOptions,
      context?: DidOperationContext,
    ) {
      requireInput(did);
      return runQuery("did.resolve", async () => {
        if (isAborted(context?.signal)) {
          throw new DidError("did.operation_aborted");
        }
        if (effects.resolver === undefined) {
          throw new DidError("did.resolution_provider_unavailable");
        }
        let serializedOptions: string;
        try {
          serializedOptions = serializeResolutionOptions(resolutionOptions);
        } catch {
          throw new DidError("did.invalid_resolution_options");
        }
        let request: DidResolutionRequestValue;
        try {
          request = projectResolutionRequest(
            binding.prepareDidResolution(
              did,
              serializedOptions,
            ),
            context?.signal,
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
        let raw: string;
        try {
          raw = await effects.resolver.resolve(request);
        } catch {
          throw new DidError("did.resolution_host_failure");
        }
        if (typeof raw !== "string") {
          throw new DidError("did.resolution_host_failure");
        }
        requireBoundedHostResult(raw, "did.invalid_resolution_result");
        try {
          return projectResolutionResult(
            binding.parseDidResolutionResult(request.did.serialized, raw),
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
      });
    },
    dereferenceDidUrl(
      didUrl: string,
      dereferencingOptions?: DidDereferencingOptions,
      context?: DidOperationContext,
    ) {
      requireInput(didUrl);
      return runQuery("did-url.dereference", async () => {
        if (isAborted(context?.signal)) {
          throw new DidError("did.operation_aborted");
        }
        if (effects.dereferencer === undefined) {
          throw new DidError("did.dereferencing_provider_unavailable");
        }
        let serializedOptions: string;
        try {
          serializedOptions = serializeDereferencingOptions(dereferencingOptions);
        } catch {
          throw new DidError("did.invalid_dereferencing_options");
        }
        let request: DidDereferencingRequestValue;
        try {
          request = projectDereferencingRequest(
            binding.prepareDidDereferencing(
              didUrl,
              serializedOptions,
            ),
            context?.signal,
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
        let raw: string;
        try {
          raw = await effects.dereferencer.dereference(request);
        } catch {
          throw new DidError("did.dereferencing_host_failure");
        }
        if (typeof raw !== "string") {
          throw new DidError("did.dereferencing_host_failure");
        }
        requireBoundedHostResult(raw, "did.invalid_dereferencing_result");
        try {
          return projectDereferencingResult(
            binding.parseDidDereferencingResult(raw),
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
      });
    },
  });
}
