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

export interface RawPeerDidNumalgo4View extends RawOwnedValue {
  readonly profile: unknown;
  readonly longForm: unknown;
  readonly shortForm: unknown;
  readonly inputDocument: unknown;
  readonly resolvedDocument: unknown;
}

export interface RawPeerDidNumalgo4ShortView extends RawOwnedValue {
  readonly profile: unknown;
  readonly shortForm: unknown;
}

export interface RawPeerDidNumalgo2CompatibilityView extends RawOwnedValue {
  readonly profile: unknown;
  readonly did: unknown;
  readonly resolvedDocument: unknown;
}

export interface RawPrismDidView extends RawOwnedValue {
  readonly requestedDid: unknown;
  readonly canonicalDid: unknown;
  readonly form: unknown;
  readonly resolvedDocument: unknown;
}

export interface RawPrismDidStateView extends RawOwnedValue {
  readonly requestedDid: unknown;
  readonly canonicalDid: unknown;
  readonly deactivated: unknown;
  readonly resolvedDocument: unknown;
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
  createPeerDidNumalgo4(inputDocumentJson: string): RawPeerDidNumalgo4View;
  parsePeerDidNumalgo4Long(value: string): RawPeerDidNumalgo4View;
  parsePeerDidNumalgo4Short(value: string): RawPeerDidNumalgo4ShortView;
  resolvePeerDidNumalgo4Short(
    shortForm: string,
    longForm: string,
  ): RawPeerDidNumalgo4View;
  createPeerDidNumalgo2Compatibility(
    inputJson: string,
  ): RawPeerDidNumalgo2CompatibilityView;
  parsePeerDidNumalgo2Compatibility(
    value: string,
  ): RawPeerDidNumalgo2CompatibilityView;
  parsePrismDid(value: string): RawPrismDidView;
  createPrismDid(inputJson: string): RawPrismDidView;
  resolvePrismDidLongForm(value: string): RawPrismDidView;
  reducePrismDidHistory(
    value: string,
    historyJson: string,
  ): RawPrismDidStateView;
  isParseError(error: unknown): error is RawDidParseError;
}
