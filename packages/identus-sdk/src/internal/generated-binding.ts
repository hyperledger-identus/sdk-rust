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
    isParseError,
  });
}
