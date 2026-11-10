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

export interface RawDidParseError extends RawOwnedValue {
  readonly code: unknown;
}

export interface RawDidBinding {
  bindingApiVersion(): unknown;
  parseDid(value: string): RawDidView;
  parseDidUrl(value: string): RawDidUrlView;
  isParseError(error: unknown): error is RawDidParseError;
}
