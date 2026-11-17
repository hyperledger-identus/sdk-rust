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

export interface RawDidResolutionRequestView extends RawOwnedValue {
  readonly did: unknown;
  readonly method: unknown;
  readonly methodSpecificId: unknown;
  readonly options: unknown;
}

export interface RawDidDereferencingRequestView extends RawOwnedValue {
  readonly didUrl: unknown;
  readonly did: unknown;
  readonly method: unknown;
  readonly methodSpecificId: unknown;
  readonly path: unknown;
  readonly query: unknown;
  readonly fragment: unknown;
  readonly options: unknown;
}

export interface RawDidResolutionResultView extends RawOwnedValue {
  readonly serialized: unknown;
  readonly kind: unknown;
}

export interface RawDidDereferencingResultView extends RawOwnedValue {
  readonly serialized: unknown;
  readonly kind: unknown;
}

export interface RawDidParseError extends RawOwnedValue {
  readonly code: unknown;
}

export interface RawDidBinding {
  bindingApiVersion(): unknown;
  parseDid(value: string): RawDidView;
  parseDidUrl(value: string): RawDidUrlView;
  parseDidDocument(value: string): RawDidDocumentView;
  prepareDidResolution(
    did: string,
    optionsJson: string,
  ): RawDidResolutionRequestView;
  prepareDidDereferencing(
    didUrl: string,
    optionsJson: string,
  ): RawDidDereferencingRequestView;
  parseDidResolutionResult(
    did: string,
    rawJson: string,
  ): RawDidResolutionResultView;
  parseDidDereferencingResult(rawJson: string): RawDidDereferencingResultView;
  isParseError(error: unknown): error is RawDidParseError;
}
