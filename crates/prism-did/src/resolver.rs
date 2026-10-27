//! Runtime-neutral PRISM DID resolution and method-registry integration.

use std::{collections::BTreeMap, fmt, future::Future, pin::Pin, sync::Arc};

use identus_did::{
    Did, DidDocumentMetadata, DidMethod, DidMethodBinding, DidResolutionError,
    DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata, DidResolutionResult,
    DidResolver, MediaType, ResolutionOptions,
};

use crate::{
    CanonicalPrismDid, LongFormPrismDid, PrismDid, PrismDidState, project_did_document_for,
};

const DID_JSON: &str = "application/did+json";
const DID_LD_JSON: &str = "application/did+ld+json";

/// Type-erased future returned by [`PrismDidHistory`].
pub type PrismDidHistoryFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<PrismDidState>, crate::Error>> + Send + 'a>>;

/// Chain-neutral port for obtaining validated published PRISM DID state.
///
/// Implementations may observe a ledger, database, snapshot, or test fixture,
/// but those dependencies remain outside this crate. `None` means the
/// canonical DID is not known. Operational failures are deliberately mapped
/// to a generic resolution error at the public resolver boundary. In
/// particular, [`crate::StateError::ResourceLimitExceeded`] means valid ledger
/// history is outside this SDK profile and MUST fail resolution; an adapter
/// must not skip that operation as invalid and continue from stale state.
pub trait PrismDidHistory: Send + Sync {
    /// Load the state selected by the generic resolution options.
    fn resolve<'a>(
        &'a self,
        did: &'a CanonicalPrismDid,
        options: &'a ResolutionOptions,
    ) -> PrismDidHistoryFuture<'a>;
}

/// Runtime-neutral resolver for local long-form and optional published state.
#[derive(Clone, Default)]
pub struct PrismDidResolver {
    history: Option<Arc<dyn PrismDidHistory>>,
}

impl PrismDidResolver {
    /// Construct a resolver that performs only local long-form resolution.
    #[must_use]
    pub const fn local() -> Self {
        Self { history: None }
    }

    /// Construct a resolver with an outer published-history provider.
    #[must_use]
    pub fn with_history(history: Arc<dyn PrismDidHistory>) -> Self {
        Self {
            history: Some(history),
        }
    }

    /// Consume this resolver into a `prism` method-registry binding.
    #[must_use]
    pub fn into_method_binding(self) -> DidMethodBinding {
        DidMethodBinding::new(
            DidMethod::parse("prism").expect("fixed PRISM method name is valid"),
            Arc::new(self),
        )
    }

    async fn resolve_inner(&self, did: &Did, options: &ResolutionOptions) -> DidResolutionResult {
        let parsed = match PrismDid::parse(did.as_str()) {
            Ok(parsed) => parsed,
            Err(_) => return failure(DidResolutionErrorKind::InvalidDid),
        };

        match parsed {
            PrismDid::LongForm(long) => {
                if let Some(history) = &self.history {
                    match history.resolve(long.canonical(), options).await {
                        Ok(Some(state)) if state.did() == long.canonical() => {
                            return resolution_from_state(
                                &state,
                                long.as_did().clone(),
                                options,
                                Some(long.canonical().as_did().clone()),
                            );
                        }
                        Ok(Some(_)) => {
                            return failure(DidResolutionErrorKind::InvalidDidDocument);
                        }
                        Ok(None) => {}
                        Err(_) => return failure(DidResolutionErrorKind::InternalError),
                    }
                }
                resolve_local_long_form(&long, options)
            }
            PrismDid::Canonical(canonical) => {
                let Some(history) = &self.history else {
                    return failure(DidResolutionErrorKind::NotFound);
                };
                match history.resolve(&canonical, options).await {
                    Ok(Some(state)) if state.did() == &canonical => {
                        resolution_from_state(&state, canonical.as_did().clone(), options, None)
                    }
                    Ok(Some(_)) => failure(DidResolutionErrorKind::InvalidDidDocument),
                    Ok(None) => failure(DidResolutionErrorKind::NotFound),
                    Err(_) => failure(DidResolutionErrorKind::InternalError),
                }
            }
        }
    }
}

impl fmt::Debug for PrismDidResolver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrismDidResolver")
            .field("has_history", &self.history.is_some())
            .finish_non_exhaustive()
    }
}

impl DidResolver for PrismDidResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(self.resolve_inner(did, options))
    }
}

fn resolve_local_long_form(
    did: &LongFormPrismDid,
    options: &ResolutionOptions,
) -> DidResolutionResult {
    if options.version_id().is_some()
        || options.version_time().is_some()
        || !options.extensions().is_empty()
    {
        return failure(DidResolutionErrorKind::FeatureNotSupported);
    }
    let state = match PrismDidState::from_initial_state(did.initial_state()) {
        Ok(state) => state,
        Err(_) => return failure(DidResolutionErrorKind::InvalidDid),
    };
    resolution_from_state(
        &state,
        did.as_did().clone(),
        options,
        Some(did.canonical().as_did().clone()),
    )
}

fn resolution_from_state(
    state: &PrismDidState,
    requested: Did,
    options: &ResolutionOptions,
    canonical: Option<Did>,
) -> DidResolutionResult {
    let content_type = match selected_media_type(options) {
        Ok(content_type) => content_type,
        Err(kind) => return failure(kind),
    };
    let metadata = match DidResolutionMetadata::new(Some(content_type), None, BTreeMap::new()) {
        Ok(metadata) => metadata,
        Err(_) => return failure(DidResolutionErrorKind::InternalError),
    };
    let mut document_metadata = DidDocumentMetadata::builder();
    if let Some(canonical) = canonical {
        document_metadata = document_metadata.canonical_id(canonical);
    }
    if state.is_deactivated() {
        return document_metadata
            .deactivated(true)
            .build()
            .and_then(|document_metadata| {
                DidResolutionResult::deactivated(metadata, document_metadata)
            })
            .unwrap_or_else(|_| failure(DidResolutionErrorKind::InternalError));
    }
    let document = match project_did_document_for(state, requested) {
        Ok(Some(document)) => document,
        Ok(None) | Err(_) => return failure(DidResolutionErrorKind::InvalidDidDocument),
    };
    document_metadata
        .build()
        .and_then(|document_metadata| {
            DidResolutionResult::success(metadata, document, document_metadata)
        })
        .unwrap_or_else(|_| failure(DidResolutionErrorKind::InvalidDidDocument))
}

fn selected_media_type(options: &ResolutionOptions) -> Result<MediaType, DidResolutionErrorKind> {
    let selected = options.accept().map_or(DID_LD_JSON, MediaType::as_str);
    if !matches!(selected, DID_JSON | DID_LD_JSON) {
        return Err(DidResolutionErrorKind::RepresentationNotSupported);
    }
    MediaType::parse(selected).map_err(|_| DidResolutionErrorKind::InternalError)
}

fn failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard PRISM resolution failure metadata is valid");
    DidResolutionResult::failure(metadata).expect("standard PRISM resolution failure is valid")
}

#[cfg(test)]
mod tests {
    use std::{
        future::Future,
        pin::pin,
        task::{Context, Poll, Waker},
    };

    use identus_crypto::Secp256k1PrivateKey;
    use prost::Message;

    use super::*;
    use crate::{RawPrismOperation, proto};

    fn block_on<F: Future>(future: F) -> F::Output {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn initial_state() -> RawPrismOperation {
        let public = Secp256k1PrivateKey::from_slice(&[1; 32])
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let create = proto::CreateDid {
            did_data: Some(proto::DidCreationData {
                public_keys: vec![
                    proto::PublicKey {
                        id: "master".to_owned(),
                        usage: proto::KeyUsage::Master as i32,
                        key_data: Some(proto::public_key::KeyData::Compressed(
                            proto::CompressedEcKeyData {
                                curve: "secp256k1".to_owned(),
                                data: public.to_vec(),
                            },
                        )),
                    },
                    proto::PublicKey {
                        id: "auth".to_owned(),
                        usage: proto::KeyUsage::Authentication as i32,
                        key_data: Some(proto::public_key::KeyData::Compressed(
                            proto::CompressedEcKeyData {
                                curve: "secp256k1".to_owned(),
                                data: public.to_vec(),
                            },
                        )),
                    },
                ],
                services: Vec::new(),
                context: Vec::new(),
            }),
        };
        let payload = create.encode_to_vec();
        let mut bytes = vec![0x0a];
        prost::encoding::encode_varint(payload.len() as u64, &mut bytes);
        bytes.extend_from_slice(&payload);
        RawPrismOperation::parse(&bytes).unwrap()
    }

    fn error_kind(result: &DidResolutionResult) -> DidResolutionErrorKind {
        result.metadata().error().unwrap().kind().unwrap()
    }

    #[test]
    fn local_long_form_resolution_is_self_contained_and_canonicalized() {
        let long = LongFormPrismDid::from_initial_state(initial_state()).unwrap();
        let result =
            block_on(PrismDidResolver::local().resolve(long.as_did(), &ResolutionOptions::empty()));

        assert_eq!(result.document().unwrap().id(), long.as_did());
        assert_eq!(
            result.document_metadata().canonical_id(),
            Some(long.canonical().as_did())
        );
        assert_eq!(
            result.metadata().content_type().unwrap().as_str(),
            DID_LD_JSON
        );
        let json = serde_json::to_string(result.document().unwrap()).unwrap();
        assert!(json.contains(long.as_did().as_str()));
        assert!(!json.contains(&format!("\"id\":\"{}\"", long.canonical().as_did())));
    }

    #[test]
    fn local_resolver_never_invents_published_history() {
        let canonical = CanonicalPrismDid::from_digest(&[7; 32]);
        let result = block_on(
            PrismDidResolver::local().resolve(canonical.as_did(), &ResolutionOptions::empty()),
        );

        assert_eq!(error_kind(&result), DidResolutionErrorKind::NotFound);
    }

    #[test]
    fn local_resolution_rejects_unsupported_options_and_media() {
        let long = LongFormPrismDid::from_initial_state(initial_state()).unwrap();
        let versioned = ResolutionOptions::builder()
            .extensions(BTreeMap::from([(
                "network".to_owned(),
                serde_json::json!("x"),
            )]))
            .build()
            .unwrap();
        let result = block_on(PrismDidResolver::local().resolve(long.as_did(), &versioned));
        assert_eq!(
            error_kind(&result),
            DidResolutionErrorKind::FeatureNotSupported
        );

        let unsupported = ResolutionOptions::builder()
            .accept(MediaType::parse("application/did+cbor").unwrap())
            .build()
            .unwrap();
        let result = block_on(PrismDidResolver::local().resolve(long.as_did(), &unsupported));
        assert_eq!(
            error_kind(&result),
            DidResolutionErrorKind::RepresentationNotSupported
        );
    }

    struct FixedHistory(PrismDidState);

    impl PrismDidHistory for FixedHistory {
        fn resolve<'a>(
            &'a self,
            _did: &'a CanonicalPrismDid,
            _options: &'a ResolutionOptions,
        ) -> PrismDidHistoryFuture<'a> {
            Box::pin(async { Ok(Some(self.0.clone())) })
        }
    }

    #[test]
    fn published_resolution_is_supplied_only_through_the_history_port() {
        let state = PrismDidState::from_initial_state(&initial_state()).unwrap();
        let canonical = state.did().clone();
        let resolver = PrismDidResolver::with_history(Arc::new(FixedHistory(state)));
        let result = block_on(resolver.resolve(canonical.as_did(), &ResolutionOptions::empty()));

        assert_eq!(result.document().unwrap().id(), canonical.as_did());
        assert!(result.metadata().error().is_none());
        assert!(resolver.clone().into_method_binding().method().as_str() == "prism");
    }

    #[test]
    fn long_form_prefers_published_history_over_embedded_initial_state() {
        let raw_create = initial_state();
        let long = LongFormPrismDid::from_initial_state(raw_create.clone()).unwrap();
        let mut state = PrismDidState::from_initial_state(&raw_create).unwrap();
        let deactivate = proto::DeactivateDid {
            previous_operation_hash: state.last_operation_hash().to_vec(),
            id: state.did().suffix().to_owned(),
        };
        let payload = deactivate.encode_to_vec();
        let mut operation = vec![0x32];
        prost::encoding::encode_varint(payload.len() as u64, &mut operation);
        operation.extend_from_slice(&payload);
        let operation = RawPrismOperation::parse(&operation).unwrap();
        let private = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
        let signed = crate::SignedPrismOperation::from_signature(
            "master",
            &private.sign(operation.as_bytes()),
            &operation,
        )
        .unwrap();
        state.apply_signed(&signed).unwrap();

        let resolver = PrismDidResolver::with_history(Arc::new(FixedHistory(state)));
        let result = block_on(resolver.resolve(long.as_did(), &ResolutionOptions::empty()));

        assert!(result.document().is_none());
        assert_eq!(result.document_metadata().deactivated(), Some(true));
        assert_eq!(
            result.document_metadata().canonical_id(),
            Some(long.canonical().as_did())
        );
    }
}
