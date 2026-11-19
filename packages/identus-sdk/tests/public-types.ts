import type {
  DidDocumentContextEntryValue,
  DidDocumentContextValue,
  DidDocumentControllerValue,
  DidDocumentExtensionMapValue,
  DidDocumentExtensionValue,
  DidDocumentValue,
  DidDereferencingOptions,
  DidDereferencingOptionsValue,
  DidDereferencingRequestValue,
  DidDocumentMetadataValue,
  DidProvider,
  DidProviderFactory,
  DidQueryEffects,
  DidQueryProvider,
  DidQueryProviderFactory,
  DidResolutionMetadataValue,
  DidResolutionOptions,
  DidResolutionOptionsValue,
  DidResolutionProblemValue,
  DidResolutionProvider,
  DidResolutionRequestValue,
  DidResolutionResultValue,
  DidResolverEffect,
  DidServiceEndpointEntryValue,
  DidServiceEndpointValue,
  DidServiceValue,
  DidUrlValue,
  DidUrlDereferencerEffect,
  DidUrlDereferencingProvider,
  DidUrlDereferencingResultValue,
  DidValue,
  DidVerificationMethodValue,
  DidVerificationRelationshipValue,
  PeerDidNumalgo2CompatibilityInput,
  PeerDidNumalgo2CompatibilityValue,
  PeerDidNumalgo2PublicKeyInput,
  PeerDidNumalgo2Purpose,
  PeerDidNumalgo2ServiceEndpointInput,
  PeerDidNumalgo2ServiceEndpointMapInput,
  PeerDidNumalgo2ServiceInput,
  PeerDidNumalgo4InputDocument,
  PeerDidNumalgo4ShortValue,
  PeerDidNumalgo4Value,
  PeerDidProvider,
  PeerDidProviderFactory,
} from "../src/index.js";
import {
  createDidProvider as createBrowserDidProvider,
  selectDidProvider as selectBrowserDidProvider,
} from "../src/browser.js";
import {
  createDidProvider as createNodeDidProvider,
  selectDidProvider as selectNodeDidProvider,
} from "../src/node.js";
import type { DidProviderState } from "../src/react.js";

type IsAny<T> = 0 extends (1 & T) ? true : false;
type AssertFalse<T extends false> = T;

type DidIsNotAny = AssertFalse<IsAny<DidValue>>;
type DidUrlIsNotAny = AssertFalse<IsAny<DidUrlValue>>;
type ProviderIsNotAny = AssertFalse<IsAny<DidProvider>>;
type DocumentIsNotAny = AssertFalse<IsAny<DidDocumentValue>>;
type ResolutionOptionsIsNotAny = AssertFalse<IsAny<DidResolutionOptions>>;
type ResolutionOptionsValueIsNotAny = AssertFalse<IsAny<DidResolutionOptionsValue>>;
type DereferencingOptionsIsNotAny = AssertFalse<IsAny<DidDereferencingOptions>>;
type DereferencingOptionsValueIsNotAny = AssertFalse<IsAny<DidDereferencingOptionsValue>>;
type ResolutionRequestIsNotAny = AssertFalse<IsAny<DidResolutionRequestValue>>;
type DereferencingRequestIsNotAny = AssertFalse<IsAny<DidDereferencingRequestValue>>;
type ResolutionProblemIsNotAny = AssertFalse<IsAny<DidResolutionProblemValue>>;
type ResolutionMetadataIsNotAny = AssertFalse<IsAny<DidResolutionMetadataValue>>;
type DocumentMetadataIsNotAny = AssertFalse<IsAny<DidDocumentMetadataValue>>;
type ResolutionResultIsNotAny = AssertFalse<IsAny<DidResolutionResultValue>>;
type DereferencingResultIsNotAny = AssertFalse<IsAny<DidUrlDereferencingResultValue>>;
type ResolutionProviderIsNotAny = AssertFalse<IsAny<DidResolutionProvider>>;
type DereferencingProviderIsNotAny = AssertFalse<IsAny<DidUrlDereferencingProvider>>;
type QueryProviderIsNotAny = AssertFalse<IsAny<DidQueryProvider>>;
type QueryEffectsIsNotAny = AssertFalse<IsAny<DidQueryEffects>>;
type ResolverEffectIsNotAny = AssertFalse<IsAny<DidResolverEffect>>;
type DereferencerEffectIsNotAny = AssertFalse<IsAny<DidUrlDereferencerEffect>>;
type QueryFactoryIsNotAny = AssertFalse<IsAny<DidQueryProviderFactory>>;
type DocumentContextIsNotAny = AssertFalse<IsAny<DidDocumentContextValue>>;
type DocumentContextEntryIsNotAny = AssertFalse<IsAny<DidDocumentContextEntryValue>>;
type DocumentControllerIsNotAny = AssertFalse<IsAny<DidDocumentControllerValue>>;
type DocumentExtensionIsNotAny = AssertFalse<IsAny<DidDocumentExtensionValue>>;
type DocumentExtensionMapIsNotAny = AssertFalse<IsAny<DidDocumentExtensionMapValue>>;
type VerificationMethodIsNotAny = AssertFalse<IsAny<DidVerificationMethodValue>>;
type VerificationRelationshipIsNotAny = AssertFalse<IsAny<DidVerificationRelationshipValue>>;
type ServiceIsNotAny = AssertFalse<IsAny<DidServiceValue>>;
type ServiceEndpointIsNotAny = AssertFalse<IsAny<DidServiceEndpointValue>>;
type ServiceEndpointEntryIsNotAny = AssertFalse<IsAny<DidServiceEndpointEntryValue>>;
type FactoryIsNotAny = AssertFalse<IsAny<DidProviderFactory>>;
type BrowserFactoryIsNotAny = AssertFalse<IsAny<typeof createBrowserDidProvider>>;
type NodeFactoryIsNotAny = AssertFalse<IsAny<typeof createNodeDidProvider>>;
type ReactStateIsNotAny = AssertFalse<IsAny<DidProviderState>>;
type PeerNumalgo2InputIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2CompatibilityInput>>;
type PeerNumalgo2ValueIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2CompatibilityValue>>;
type PeerPublicKeyIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2PublicKeyInput>>;
type PeerPurposeIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2Purpose>>;
type PeerEndpointIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2ServiceEndpointInput>>;
type PeerEndpointMapIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2ServiceEndpointMapInput>>;
type PeerServiceIsNotAny = AssertFalse<IsAny<PeerDidNumalgo2ServiceInput>>;
type PeerNumalgo4InputIsNotAny = AssertFalse<IsAny<PeerDidNumalgo4InputDocument>>;
type PeerNumalgo4ShortIsNotAny = AssertFalse<IsAny<PeerDidNumalgo4ShortValue>>;
type PeerNumalgo4ValueIsNotAny = AssertFalse<IsAny<PeerDidNumalgo4Value>>;
type PeerProviderIsNotAny = AssertFalse<IsAny<PeerDidProvider>>;
type PeerFactoryIsNotAny = AssertFalse<IsAny<PeerDidProviderFactory>>;

const compileAssertions: readonly [
  DidIsNotAny,
  DidUrlIsNotAny,
  ProviderIsNotAny,
  DocumentIsNotAny,
  ResolutionOptionsIsNotAny,
  ResolutionOptionsValueIsNotAny,
  DereferencingOptionsIsNotAny,
  DereferencingOptionsValueIsNotAny,
  ResolutionRequestIsNotAny,
  DereferencingRequestIsNotAny,
  ResolutionProblemIsNotAny,
  ResolutionMetadataIsNotAny,
  DocumentMetadataIsNotAny,
  ResolutionResultIsNotAny,
  DereferencingResultIsNotAny,
  ResolutionProviderIsNotAny,
  DereferencingProviderIsNotAny,
  QueryProviderIsNotAny,
  QueryEffectsIsNotAny,
  ResolverEffectIsNotAny,
  DereferencerEffectIsNotAny,
  QueryFactoryIsNotAny,
  DocumentContextIsNotAny,
  DocumentContextEntryIsNotAny,
  DocumentControllerIsNotAny,
  DocumentExtensionIsNotAny,
  DocumentExtensionMapIsNotAny,
  VerificationMethodIsNotAny,
  VerificationRelationshipIsNotAny,
  ServiceIsNotAny,
  ServiceEndpointIsNotAny,
  ServiceEndpointEntryIsNotAny,
  FactoryIsNotAny,
  BrowserFactoryIsNotAny,
  NodeFactoryIsNotAny,
  ReactStateIsNotAny,
  PeerNumalgo2InputIsNotAny,
  PeerNumalgo2ValueIsNotAny,
  PeerPublicKeyIsNotAny,
  PeerPurposeIsNotAny,
  PeerEndpointIsNotAny,
  PeerEndpointMapIsNotAny,
  PeerServiceIsNotAny,
  PeerNumalgo4InputIsNotAny,
  PeerNumalgo4ShortIsNotAny,
  PeerNumalgo4ValueIsNotAny,
  PeerProviderIsNotAny,
  PeerFactoryIsNotAny,
] = [
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
  false,
];

declare const parseDid: DidProvider["parseDid"];
declare const parseDidUrl: DidProvider["parseDidUrl"];
declare const parseDidDocument: DidProvider["parseDidDocument"];
const peerlessProvider: DidProvider = Object.freeze({
  parseDid,
  parseDidUrl,
  parseDidDocument,
});
const peerlessFactory: DidProviderFactory = async () => peerlessProvider;
const peerFactory: PeerDidProviderFactory = createNodeDidProvider;
const browserPeerSelection: Promise<PeerDidProvider> = selectBrowserDidProvider({
  kind: "peer-wasm",
});
const nodePeerlessSelection: Promise<DidProvider> = selectNodeDidProvider({
  kind: "injected",
  provider: peerlessProvider,
});

void peerlessFactory;
void peerFactory;
void browserPeerSelection;
void nodePeerlessSelection;

declare const document: DidDocumentValue;
const documentId: string = document.id.serialized;
const documentJson: string = document.serialized;
const extension: DidDocumentExtensionValue | undefined = document.properties.example;
const relationship: DidVerificationRelationshipValue | undefined = document.authentication?.[0];
const relationshipId: string | undefined = relationship?.kind === "reference"
  ? relationship.reference
  : relationship?.method.id;

void documentId;
void documentJson;
void extension;
void relationshipId;

void compileAssertions;
