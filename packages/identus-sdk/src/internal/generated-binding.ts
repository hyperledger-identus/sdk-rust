import type { RawDidBinding, RawDidParseError, RawDidUrlView, RawDidView } from "./raw-binding.js";

export interface GeneratedDidBindingModule {
  bindingApiVersion(): number;
  parseDid(value: string): RawDidView;
  parseDidUrl(value: string): RawDidUrlView;
}

export function adaptGeneratedBinding(
  generated: GeneratedDidBindingModule,
  isParseError: (error: unknown) => error is RawDidParseError,
): RawDidBinding {
  return Object.freeze({
    bindingApiVersion: () => generated.bindingApiVersion(),
    parseDid: (value: string) => generated.parseDid(value),
    parseDidUrl: (value: string) => generated.parseDidUrl(value),
    isParseError,
  });
}
