export interface RawOwnedValue {
  free(): void;
}

export interface RawDidView extends RawOwnedValue {
  readonly value: unknown;
  readonly method: unknown;
  readonly methodSpecificId: unknown;
}

export interface RawDidUrlView extends RawDidView {
  readonly did: unknown;
  readonly path: unknown;
  readonly query: unknown;
  readonly fragment: unknown;
}

export interface RawDidDocumentView extends RawOwnedValue {
  readonly serialized: unknown;
  readonly id: unknown;
}

export interface RawDidParseError extends RawOwnedValue {
  readonly code: unknown;
}

export interface RawDidBinding {
  bindingApiVersion(): unknown;
  parseDid(value: string): RawDidView;
  parseDidUrl(value: string): RawDidUrlView;
  parseDidDocument(value: string): RawDidDocumentView;
  isParseError(error: unknown): error is RawDidParseError;
}
