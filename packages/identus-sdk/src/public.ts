export type DidErrorCode = "did.invalid_did" | "did.invalid_did_url";

export interface DidValue {
  readonly serialized: string;
  readonly method: string;
  readonly methodSpecificId: string;
}

export interface DidUrlValue extends DidValue {
  readonly did: string;
  readonly path: string;
  readonly query: string | undefined;
  readonly fragment: string | undefined;
}

export type DidOperation = "did.parse" | "did-url.parse";

export type DidProviderRoute = "browser-wasm" | "node-wasm" | "legacy";

export type DidMappingId =
  | "did.value.typescript.legacy-v1"
  | "did-url.value.typescript.legacy-v1"
  | "did.error.invalid-did.typescript.legacy-v1"
  | "did.error.invalid-did-url.typescript.legacy-v1";

export interface DidObservation {
  readonly operation: DidOperation;
  readonly route: DidProviderRoute;
  readonly result: "success" | "error";
  readonly mappingId: DidMappingId;
  readonly errorCode: DidErrorCode | undefined;
  readonly durationMs: number;
}

export type DidObserver = (observation: DidObservation) => void;

export interface DidProviderOptions {
  readonly observer?: DidObserver;
}

export interface DidProvider {
  parseDid(value: string): DidValue;
  parseDidUrl(value: string): DidUrlValue;
}

export type DidProviderFactory = (
  options?: DidProviderOptions,
) => Promise<DidProvider>;

const ERROR_MESSAGES: Readonly<Record<DidErrorCode, string>> = Object.freeze({
  "did.invalid_did": "DID value is invalid",
  "did.invalid_did_url": "DID URL value is invalid",
});

export class DidError extends Error {
  readonly code: DidErrorCode;

  constructor(code: DidErrorCode) {
    super(ERROR_MESSAGES[code]);
    this.name = "DidError";
    this.code = code;
    Object.defineProperty(this, "stack", {
      configurable: false,
      enumerable: false,
      value: undefined,
      writable: false,
    });
    Object.freeze(this);
  }
}
