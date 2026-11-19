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
  RawPeerDidNumalgo2CompatibilityView,
  RawPeerDidNumalgo4ShortView,
  RawPeerDidNumalgo4View,
} from "./raw-binding.js";

export interface GeneratedDidBindingModule {
  bindingApiVersion(): number;
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
}

export function adaptGeneratedBinding(
  generated: GeneratedDidBindingModule,
  isParseError: (error: unknown) => error is RawDidParseError,
): RawDidBinding {
  return Object.freeze({
    bindingApiVersion: () => generated.bindingApiVersion(),
    parseDid: (value: string) => generated.parseDid(value),
    parseDidUrl: (value: string) => generated.parseDidUrl(value),
    parseDidDocument: (value: string) => generated.parseDidDocument(value),
    prepareDidResolution: (did: string, optionsJson: string) =>
      generated.prepareDidResolution(did, optionsJson),
    prepareDidDereferencing: (didUrl: string, optionsJson: string) =>
      generated.prepareDidDereferencing(didUrl, optionsJson),
    parseDidResolutionResult: (did: string, rawJson: string) =>
      generated.parseDidResolutionResult(did, rawJson),
    parseDidDereferencingResult: (rawJson: string) =>
      generated.parseDidDereferencingResult(rawJson),
    createPeerDidNumalgo4: (inputDocumentJson: string) =>
      generated.createPeerDidNumalgo4(inputDocumentJson),
    parsePeerDidNumalgo4Long: (value: string) =>
      generated.parsePeerDidNumalgo4Long(value),
    parsePeerDidNumalgo4Short: (value: string) =>
      generated.parsePeerDidNumalgo4Short(value),
    resolvePeerDidNumalgo4Short: (shortForm: string, longForm: string) =>
      generated.resolvePeerDidNumalgo4Short(shortForm, longForm),
    createPeerDidNumalgo2Compatibility: (inputJson: string) =>
      generated.createPeerDidNumalgo2Compatibility(inputJson),
    parsePeerDidNumalgo2Compatibility: (value: string) =>
      generated.parsePeerDidNumalgo2Compatibility(value),
    isParseError,
  });
}
