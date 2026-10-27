//! Authenticated, deterministic and atomic PRISM DID state reduction.

use std::{collections::BTreeSet, fmt};

use identus_crypto::{Sha256Digest, Verifiable};

use crate::{
    CanonicalPrismDid, CreateDidOperation, DeactivateDidOperation, Error, KeyUsage,
    MAX_RETIRED_IDENTIFIERS_PER_KIND, MAX_SERVICES, MAX_VERIFICATION_METHODS, OperationError,
    OperationKind, PrismPublicKey, PrismPublicKeyMaterial, PrismService, RawPrismOperation,
    SignedPrismOperation, StateError, UpdateAction, UpdateDidOperation,
};

/// Materialized PRISM DID method state.
///
/// Active keys and services retain insertion order. Removed values are dropped
/// while their bounded identifier tombstones prevent reuse. Signed state
/// transitions verify against an active master key over the exact nested
/// operation bytes before a cloned candidate is mutated and committed.
#[derive(Clone, PartialEq)]
pub struct PrismDidState {
    did: CanonicalPrismDid,
    last_operation_hash: Sha256Digest,
    public_keys: Vec<PrismPublicKey>,
    retired_public_key_ids: BTreeSet<String>,
    services: Vec<PrismService>,
    retired_service_ids: BTreeSet<String>,
    contexts: Vec<String>,
    deactivated: bool,
}

impl fmt::Debug for PrismDidState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrismDidState")
            .field("active_public_keys", &self.public_keys.len())
            .field("retired_public_key_ids", &self.retired_public_key_ids.len())
            .field("active_services", &self.services.len())
            .field("retired_service_ids", &self.retired_service_ids.len())
            .field("contexts", &self.contexts.len())
            .field("deactivated", &self.deactivated)
            .finish_non_exhaustive()
    }
}

impl PrismDidState {
    /// Materialize self-certifying state from an exact create operation.
    ///
    /// This entry point is intended for local long-form DID resolution. Use
    /// [`Self::from_signed_create`] when accepting a published signed envelope.
    pub fn from_initial_state(raw: &RawPrismOperation) -> Result<Self, Error> {
        let operation = CreateDidOperation::parse(raw)?;
        let state = Self::from_create(operation, raw.operation_hash());
        crate::project_did_document(&state)?;
        Ok(state)
    }

    /// Verify a signed create envelope and materialize its initial state.
    pub fn from_signed_create(signed: &SignedPrismOperation) -> Result<Self, Error> {
        if signed.operation_kind() != OperationKind::CreateDid {
            return Err(OperationError::InvalidValue.into());
        }
        let raw = RawPrismOperation::parse(signed.operation_bytes())?;
        let state = Self::from_initial_state(&raw)?;
        state.verify_master_signature(signed)?;
        Ok(state)
    }

    /// Verify and atomically apply a signed DID update or deactivation.
    ///
    /// On any parsing, authentication, action, or final-invariant failure,
    /// `self` remains byte-for-byte equivalent to its pre-call value.
    pub fn apply_signed(&mut self, signed: &SignedPrismOperation) -> Result<(), Error> {
        if self.deactivated {
            return Err(StateError::Deactivated.into());
        }
        self.verify_master_signature(signed)?;
        let raw = RawPrismOperation::parse(signed.operation_bytes())?;
        let mut candidate = self.clone();
        match signed.operation_kind() {
            OperationKind::UpdateDid => candidate.reduce_update(&raw)?,
            OperationKind::DeactivateDid => candidate.reduce_deactivation(&raw)?,
            _ => return Err(OperationError::InvalidValue.into()),
        }
        *self = candidate;
        Ok(())
    }

    /// Borrow the canonical identifier of this state.
    #[must_use]
    pub const fn did(&self) -> &CanonicalPrismDid {
        &self.did
    }

    /// Return the exact hash of the last accepted SSI operation.
    pub const fn last_operation_hash(&self) -> Sha256Digest {
        self.last_operation_hash
    }

    /// Return whether the DID has been deactivated.
    #[must_use]
    pub const fn is_deactivated(&self) -> bool {
        self.deactivated
    }

    /// Iterate active keys in deterministic insertion order.
    pub fn active_public_keys(&self) -> impl Iterator<Item = &PrismPublicKey> {
        self.public_keys.iter()
    }

    /// Iterate active services in deterministic insertion order.
    pub fn active_services(&self) -> impl Iterator<Item = &PrismService> {
        self.services.iter()
    }

    /// Borrow validated additional contexts in exact signed wire order.
    #[must_use]
    pub fn contexts(&self) -> &[String] {
        &self.contexts
    }

    fn from_create(operation: CreateDidOperation, operation_hash: Sha256Digest) -> Self {
        Self {
            did: CanonicalPrismDid::from_digest(operation_hash.as_array()),
            last_operation_hash: operation_hash,
            public_keys: operation.public_keys().to_vec(),
            retired_public_key_ids: BTreeSet::new(),
            services: operation.services().to_vec(),
            retired_service_ids: BTreeSet::new(),
            contexts: operation.contexts().to_vec(),
            deactivated: false,
        }
    }

    fn verify_master_signature(&self, signed: &SignedPrismOperation) -> Result<(), Error> {
        let Some(key) = self
            .public_keys
            .iter()
            .find(|key| key.id() == signed.signed_with() && key.usage() == KeyUsage::Master)
        else {
            return Err(OperationError::InvalidSignature.into());
        };
        let PrismPublicKeyMaterial::Secp256k1(key) = key.material() else {
            return Err(OperationError::InvalidSignature.into());
        };
        if key.verify(signed.operation_bytes(), signed.signature()) {
            Ok(())
        } else {
            Err(OperationError::InvalidSignature.into())
        }
    }

    fn reduce_update(&mut self, raw: &RawPrismOperation) -> Result<(), Error> {
        let operation = UpdateDidOperation::parse(raw)?;
        self.check_history(operation.did(), operation.previous_operation_hash())?;
        for action in operation.actions() {
            self.apply_action(action.clone())?;
        }
        if self.public_keys.len() > MAX_VERIFICATION_METHODS || self.services.len() > MAX_SERVICES {
            return Err(StateError::InvalidCandidate.into());
        }
        if !self
            .active_public_keys()
            .any(|key| key.usage() == KeyUsage::Master)
        {
            return Err(StateError::InvalidCandidate.into());
        }
        if self.retired_public_key_ids.len() > MAX_RETIRED_IDENTIFIERS_PER_KIND
            || self.retired_service_ids.len() > MAX_RETIRED_IDENTIFIERS_PER_KIND
        {
            return Err(StateError::ResourceLimitExceeded.into());
        }
        crate::project_did_document(self)?;
        self.last_operation_hash = raw.operation_hash();
        Ok(())
    }

    fn reduce_deactivation(&mut self, raw: &RawPrismOperation) -> Result<(), Error> {
        let operation = DeactivateDidOperation::parse(raw)?;
        self.check_history(operation.did(), operation.previous_operation_hash())?;
        // Deactivation is terminal, so no later operation can reuse an active
        // identifier. Retaining or expanding tombstones here would only make
        // a valid final ledger operation depend on a local resource profile.
        self.public_keys.clear();
        self.services.clear();
        self.retired_public_key_ids.clear();
        self.retired_service_ids.clear();
        self.contexts.clear();
        self.last_operation_hash = raw.operation_hash();
        self.deactivated = true;
        Ok(())
    }

    fn check_history(
        &self,
        did: &CanonicalPrismDid,
        previous_operation_hash: Sha256Digest,
    ) -> Result<(), Error> {
        if did == &self.did && previous_operation_hash == self.last_operation_hash {
            Ok(())
        } else {
            Err(StateError::HistoryMismatch.into())
        }
    }

    fn apply_action(&mut self, action: UpdateAction) -> Result<(), Error> {
        match action {
            UpdateAction::AddKey(key) => {
                if self
                    .public_keys
                    .iter()
                    .any(|active| active.id() == key.id())
                    || self.retired_public_key_ids.contains(key.id())
                {
                    return Err(StateError::IdentifierConflict.into());
                }
                self.public_keys.push(key);
            }
            UpdateAction::RemoveKey(id) => {
                let index = self
                    .public_keys
                    .iter()
                    .position(|key| key.id() == id)
                    .ok_or(Error::InvalidState(StateError::IdentifierConflict))?;
                let removed = self.public_keys.remove(index);
                self.retired_public_key_ids.insert(removed.id().to_owned());
            }
            UpdateAction::AddService(service) => {
                if self
                    .services
                    .iter()
                    .any(|active| active.id() == service.id())
                    || self.retired_service_ids.contains(service.id())
                {
                    return Err(StateError::IdentifierConflict.into());
                }
                self.services.push(service);
            }
            UpdateAction::RemoveService(id) => {
                let index = self
                    .services
                    .iter()
                    .position(|service| service.id() == id)
                    .ok_or(Error::InvalidState(StateError::IdentifierConflict))?;
                let removed = self.services.remove(index);
                self.retired_service_ids.insert(removed.id().to_owned());
            }
            UpdateAction::UpdateService {
                id,
                service_type,
                endpoint,
            } => {
                let entry = self
                    .services
                    .iter_mut()
                    .find(|service| service.id() == id)
                    .ok_or(Error::InvalidState(StateError::IdentifierConflict))?;
                if let Some(service_type) = service_type {
                    entry.replace_type(service_type);
                }
                if let Some(endpoint) = endpoint {
                    entry.replace_endpoint(endpoint);
                }
            }
            UpdateAction::PatchContext(contexts) => self.contexts = contexts,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use identus_crypto::Secp256k1PrivateKey;
    use prost::Message;

    use super::*;
    use crate::proto;

    const PRIVATE_KEY: [u8; 32] = [1; 32];

    fn operation(field: u8, payload: &[u8]) -> RawPrismOperation {
        let mut bytes = vec![field << 3 | 2];
        prost::encoding::encode_varint(payload.len() as u64, &mut bytes);
        bytes.extend_from_slice(payload);
        RawPrismOperation::parse(&bytes).unwrap()
    }

    fn initial_state() -> PrismDidState {
        let public = Secp256k1PrivateKey::from_slice(&PRIVATE_KEY)
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let create = proto::CreateDid {
            did_data: Some(proto::DidCreationData {
                public_keys: vec![proto::PublicKey {
                    id: "master".to_owned(),
                    usage: proto::KeyUsage::Master as i32,
                    key_data: Some(proto::public_key::KeyData::Compressed(
                        proto::CompressedEcKeyData {
                            curve: "secp256k1".to_owned(),
                            data: public.to_vec(),
                        },
                    )),
                }],
                services: vec![proto::Service {
                    id: "service".to_owned(),
                    r#type: "LinkedDomains".to_owned(),
                    service_endpoint: "https://example.com".to_owned(),
                }],
                context: Vec::new(),
            }),
        };
        PrismDidState::from_initial_state(&operation(1, &create.encode_to_vec())).unwrap()
    }

    fn signed_update(
        state: &PrismDidState,
        actions: Vec<proto::UpdateDidAction>,
    ) -> SignedPrismOperation {
        let update = proto::UpdateDid {
            previous_operation_hash: state.last_operation_hash().to_vec(),
            id: state.did().suffix().to_owned(),
            actions,
        };
        signed(2, &update.encode_to_vec())
    }

    fn signed(field: u8, payload: &[u8]) -> SignedPrismOperation {
        let raw = operation(field, payload);
        let signature = Secp256k1PrivateKey::from_slice(&PRIVATE_KEY)
            .unwrap()
            .sign(raw.as_bytes());
        let mut envelope = vec![0x0a, 6];
        envelope.extend_from_slice(b"master");
        envelope.push(0x12);
        prost::encoding::encode_varint(signature.len() as u64, &mut envelope);
        envelope.extend_from_slice(&signature);
        envelope.push(0x1a);
        prost::encoding::encode_varint(raw.as_bytes().len() as u64, &mut envelope);
        envelope.extend_from_slice(raw.as_bytes());
        SignedPrismOperation::parse(&envelope).unwrap()
    }

    fn action(action: proto::update_did_action::Action) -> proto::UpdateDidAction {
        proto::UpdateDidAction {
            action: Some(action),
        }
    }

    #[test]
    fn failed_later_action_rolls_back_the_complete_update() {
        let mut state = initial_state();
        let before = state.clone();
        let update = signed_update(
            &state,
            vec![
                action(proto::update_did_action::Action::PatchContext(
                    proto::PatchContextAction {
                        context: vec!["https://example.com/context".to_owned()],
                    },
                )),
                action(proto::update_did_action::Action::RemoveService(
                    proto::RemoveServiceAction {
                        service_id: "missing".to_owned(),
                    },
                )),
            ],
        );
        assert_eq!(
            state.apply_signed(&update),
            Err(Error::InvalidState(StateError::IdentifierConflict))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn debug_redacts_state_identifiers_values_and_key_material() {
        let state = initial_state();
        let debug = format!("{state:?}");

        assert!(!debug.contains(state.did().suffix()));
        assert!(!debug.contains("master"));
        assert!(!debug.contains("example.com"));
        assert!(!debug.contains(&hex::encode(PRIVATE_KEY)));
    }

    #[test]
    fn tombstones_reserve_identifiers_but_not_active_capacity() {
        let mut state = initial_state();
        let remove = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::RemoveService(
                proto::RemoveServiceAction {
                    service_id: "service".to_owned(),
                },
            ))],
        );
        state.apply_signed(&remove).unwrap();
        assert_eq!(state.active_services().count(), 0);

        let readd = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::AddService(
                proto::AddServiceAction {
                    service: Some(proto::Service {
                        id: "service".to_owned(),
                        r#type: "LinkedDomains".to_owned(),
                        service_endpoint: "https://new.example.com".to_owned(),
                    }),
                },
            ))],
        );
        let before = state.clone();
        assert_eq!(
            state.apply_signed(&readd),
            Err(Error::InvalidState(StateError::IdentifierConflict))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn final_master_invariant_allows_ordered_rotation() {
        let mut state = initial_state();
        let public = Secp256k1PrivateKey::from_slice(&[2; 32])
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let rotate = signed_update(
            &state,
            vec![
                action(proto::update_did_action::Action::RemoveKey(
                    proto::RemoveKeyAction {
                        key_id: "master".to_owned(),
                    },
                )),
                action(proto::update_did_action::Action::AddKey(
                    proto::AddKeyAction {
                        key: Some(proto::PublicKey {
                            id: "master-next".to_owned(),
                            usage: proto::KeyUsage::Master as i32,
                            key_data: Some(proto::public_key::KeyData::Compressed(
                                proto::CompressedEcKeyData {
                                    curve: "secp256k1".to_owned(),
                                    data: public.to_vec(),
                                },
                            )),
                        }),
                    },
                )),
            ],
        );
        state.apply_signed(&rotate).unwrap();
        assert_eq!(state.active_public_keys().count(), 1);
        assert_eq!(
            state.active_public_keys().next().unwrap().id(),
            "master-next"
        );
    }

    #[test]
    fn signature_covers_exact_operation_bytes() {
        let mut state = initial_state();
        let update = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::PatchContext(
                proto::PatchContextAction {
                    context: vec!["https://example.com/context".to_owned()],
                },
            ))],
        );
        let mut tampered = update.as_bytes().to_vec();
        let last = tampered.last_mut().unwrap();
        *last ^= 1;
        let tampered = SignedPrismOperation::parse(&tampered).unwrap();
        assert_eq!(
            state.apply_signed(&tampered),
            Err(Error::InvalidOperation(OperationError::InvalidSignature))
        );
    }

    #[test]
    fn historical_churn_does_not_consume_active_service_capacity() {
        let mut state = initial_state();
        let mut active = "service".to_owned();
        for sequence in 0..=MAX_SERVICES {
            let next = format!("service-{sequence}");
            let update = signed_update(
                &state,
                vec![
                    action(proto::update_did_action::Action::RemoveService(
                        proto::RemoveServiceAction { service_id: active },
                    )),
                    action(proto::update_did_action::Action::AddService(
                        proto::AddServiceAction {
                            service: Some(proto::Service {
                                id: next.clone(),
                                r#type: "LinkedDomains".to_owned(),
                                service_endpoint: "https://example.com".to_owned(),
                            }),
                        },
                    )),
                ],
            );
            state.apply_signed(&update).unwrap();
            active = next;
        }
        assert_eq!(state.active_services().count(), 1);
        assert!(state.retired_service_ids.len() > MAX_SERVICES);
        assert_eq!(state.services.len(), 1);
    }

    #[test]
    fn create_rejects_an_aggregate_service_budget_that_cannot_project() {
        let public = Secp256k1PrivateKey::from_slice(&PRIVATE_KEY)
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let endpoint = format!(r#"{{"x":[{}]}}"#, vec!["0"; 100].join(","));
        let services = (0..MAX_SERVICES)
            .map(|index| proto::Service {
                id: format!("service-{index}"),
                r#type: "LinkedDomains".to_owned(),
                service_endpoint: endpoint.clone(),
            })
            .collect();
        let create = proto::CreateDid {
            did_data: Some(proto::DidCreationData {
                public_keys: vec![proto::PublicKey {
                    id: "master".to_owned(),
                    usage: proto::KeyUsage::Master as i32,
                    key_data: Some(proto::public_key::KeyData::Compressed(
                        proto::CompressedEcKeyData {
                            curve: "secp256k1".to_owned(),
                            data: public.to_vec(),
                        },
                    )),
                }],
                services,
                context: Vec::new(),
            }),
        };
        let raw = operation(1, &create.encode_to_vec());

        assert!(CreateDidOperation::parse(&raw).is_ok());
        assert_eq!(
            PrismDidState::from_initial_state(&raw),
            Err(Error::InvalidState(StateError::InvalidCandidate))
        );
    }

    #[test]
    fn update_rolls_back_an_aggregate_service_budget_that_cannot_project() {
        let mut state = initial_state();
        let before = state.clone();
        let endpoint = format!(r#"{{"x":[{}]}}"#, vec!["0"; 100].join(","));
        let actions = (1..MAX_SERVICES)
            .map(|index| {
                action(proto::update_did_action::Action::AddService(
                    proto::AddServiceAction {
                        service: Some(proto::Service {
                            id: format!("service-{index}"),
                            r#type: "LinkedDomains".to_owned(),
                            service_endpoint: endpoint.clone(),
                        }),
                    },
                ))
            })
            .collect();
        let update = signed_update(&state, actions);

        assert_eq!(
            state.apply_signed(&update),
            Err(Error::InvalidState(StateError::InvalidCandidate))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn deactivation_is_authenticated_and_terminal() {
        let mut state = initial_state();
        let deactivate = proto::DeactivateDid {
            previous_operation_hash: state.last_operation_hash().to_vec(),
            id: state.did().suffix().to_owned(),
        };
        state
            .apply_signed(&signed(6, &deactivate.encode_to_vec()))
            .unwrap();
        assert!(state.is_deactivated());
        assert_eq!(state.active_public_keys().count(), 0);
        assert_eq!(state.active_services().count(), 0);
        assert!(state.contexts().is_empty());

        let before = state.clone();
        assert_eq!(
            state.apply_signed(&signed(6, &deactivate.encode_to_vec())),
            Err(Error::InvalidState(StateError::Deactivated))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn property_generated_state_traces_preserve_atomic_invariants() {
        let mut state = initial_state();
        let mut seed = 0x9e37_79b9_u32;
        let mut next_service = 0_u32;

        for _ in 0..512 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let selected = seed % 5;
            let actions = match selected {
                0 => {
                    let id = format!("generated-{next_service}");
                    next_service += 1;
                    vec![action(proto::update_did_action::Action::AddService(
                        proto::AddServiceAction {
                            service: Some(proto::Service {
                                id: id.clone(),
                                r#type: "LinkedDomains".to_owned(),
                                service_endpoint: format!("https://example.com/{id}"),
                            }),
                        },
                    ))]
                }
                1 => {
                    let id = state
                        .services
                        .first()
                        .map_or_else(|| "missing".to_owned(), |service| service.id().to_owned());
                    vec![action(proto::update_did_action::Action::RemoveService(
                        proto::RemoveServiceAction { service_id: id },
                    ))]
                }
                2 => vec![action(proto::update_did_action::Action::RemoveService(
                    proto::RemoveServiceAction {
                        service_id: format!("absent-{next_service}"),
                    },
                ))],
                3 => vec![action(proto::update_did_action::Action::PatchContext(
                    proto::PatchContextAction {
                        context: vec![format!("https://example.com/context/{next_service}")],
                    },
                ))],
                _ => vec![
                    action(proto::update_did_action::Action::PatchContext(
                        proto::PatchContextAction {
                            context: vec!["https://example.com/rollback".to_owned()],
                        },
                    )),
                    action(proto::update_did_action::Action::RemoveKey(
                        proto::RemoveKeyAction {
                            key_id: "master".to_owned(),
                        },
                    )),
                ],
            };
            let update = signed_update(&state, actions);
            let before = state.clone();
            let expected_hash = update.operation_hash();
            let result = state.apply_signed(&update);

            if result.is_err() {
                assert_eq!(state, before, "failed generated update was not atomic");
                continue;
            }
            assert_eq!(state.last_operation_hash(), expected_hash);
            assert!(state.active_services().count() <= MAX_SERVICES);
            assert!(state.active_public_keys().count() <= MAX_VERIFICATION_METHODS);
            assert!(
                state
                    .active_public_keys()
                    .any(|key| key.usage() == KeyUsage::Master)
            );
            for (index, service) in state.services.iter().enumerate() {
                assert!(
                    state.services[..index]
                        .iter()
                        .all(|prior| prior.id() != service.id()),
                    "generated trace duplicated an active service identifier"
                );
                assert!(!state.retired_service_ids.contains(service.id()));
            }
        }
    }

    #[test]
    fn retired_identifier_budget_fails_atomically() {
        let mut state = initial_state();
        state.retired_service_ids = (0..MAX_RETIRED_IDENTIFIERS_PER_KIND)
            .map(|index| format!("retired-{index}"))
            .collect();
        let remove = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::RemoveService(
                proto::RemoveServiceAction {
                    service_id: "service".to_owned(),
                },
            ))],
        );
        let before = state.clone();

        assert_eq!(
            state.apply_signed(&remove),
            Err(Error::InvalidState(StateError::ResourceLimitExceeded))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn retired_key_budget_fails_atomically() {
        let mut state = initial_state();
        let public = Secp256k1PrivateKey::from_slice(&[2; 32])
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let add_auth = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::AddKey(
                proto::AddKeyAction {
                    key: Some(proto::PublicKey {
                        id: "auth".to_owned(),
                        usage: proto::KeyUsage::Authentication as i32,
                        key_data: Some(proto::public_key::KeyData::Compressed(
                            proto::CompressedEcKeyData {
                                curve: "secp256k1".to_owned(),
                                data: public.to_vec(),
                            },
                        )),
                    }),
                },
            ))],
        );
        state.apply_signed(&add_auth).unwrap();
        state.retired_public_key_ids = (0..MAX_RETIRED_IDENTIFIERS_PER_KIND)
            .map(|index| format!("retired-key-{index}"))
            .collect();
        let remove = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::RemoveKey(
                proto::RemoveKeyAction {
                    key_id: "auth".to_owned(),
                },
            ))],
        );
        let before = state.clone();

        assert_eq!(
            state.apply_signed(&remove),
            Err(Error::InvalidState(StateError::ResourceLimitExceeded))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn invalid_removal_wins_over_a_full_retired_identifier_budget() {
        let mut state = initial_state();
        state.retired_service_ids = (0..MAX_RETIRED_IDENTIFIERS_PER_KIND)
            .map(|index| format!("retired-{index}"))
            .collect();
        let remove = signed_update(
            &state,
            vec![action(proto::update_did_action::Action::RemoveService(
                proto::RemoveServiceAction {
                    service_id: "missing".to_owned(),
                },
            ))],
        );

        assert_eq!(
            state.apply_signed(&remove),
            Err(Error::InvalidState(StateError::IdentifierConflict))
        );
    }

    #[test]
    fn active_capacity_is_validated_on_the_final_ordered_candidate() {
        let mut state = initial_state();
        let fill = (1..MAX_SERVICES)
            .map(|index| {
                action(proto::update_did_action::Action::AddService(
                    proto::AddServiceAction {
                        service: Some(proto::Service {
                            id: format!("service-{index}"),
                            r#type: "LinkedDomains".to_owned(),
                            service_endpoint: "https://example.com".to_owned(),
                        }),
                    },
                ))
            })
            .collect();
        let fill = signed_update(&state, fill);
        state.apply_signed(&fill).unwrap();
        assert_eq!(state.active_services().count(), MAX_SERVICES);

        let replace = signed_update(
            &state,
            vec![
                action(proto::update_did_action::Action::AddService(
                    proto::AddServiceAction {
                        service: Some(proto::Service {
                            id: "service-next".to_owned(),
                            r#type: "LinkedDomains".to_owned(),
                            service_endpoint: "https://example.com/next".to_owned(),
                        }),
                    },
                )),
                action(proto::update_did_action::Action::RemoveService(
                    proto::RemoveServiceAction {
                        service_id: "service".to_owned(),
                    },
                )),
            ],
        );

        state.apply_signed(&replace).unwrap();
        assert_eq!(state.active_services().count(), MAX_SERVICES);
        assert!(
            state
                .active_services()
                .any(|service| service.id() == "service-next")
        );
    }

    #[test]
    fn deactivation_succeeds_when_retired_identifier_budgets_are_full() {
        let mut state = initial_state();
        state.retired_public_key_ids = (0..MAX_RETIRED_IDENTIFIERS_PER_KIND)
            .map(|index| format!("retired-key-{index}"))
            .collect();
        state.retired_service_ids = (0..MAX_RETIRED_IDENTIFIERS_PER_KIND)
            .map(|index| format!("retired-service-{index}"))
            .collect();
        let deactivate = proto::DeactivateDid {
            previous_operation_hash: state.last_operation_hash().to_vec(),
            id: state.did().suffix().to_owned(),
        };

        state
            .apply_signed(&signed(6, &deactivate.encode_to_vec()))
            .unwrap();

        assert!(state.is_deactivated());
        assert_eq!(state.active_public_keys().count(), 0);
        assert_eq!(state.active_services().count(), 0);
        assert!(state.retired_public_key_ids.is_empty());
        assert!(state.retired_service_ids.is_empty());
    }

    #[test]
    fn verifies_pinned_preprod_signed_create_envelope() {
        let fixtures = [
            (
                "0a076d61737465723012483046022100b8e57b812375ea9e3c3ae79459c8695a149419bf68c2e9e18a185dddce4b08af022100aa5fa6dc7dee7c0d0b91ce05c204527e84bba9ef87e278708512d0c91ac2403c1a410a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee",
                "5873d42d38408e10f5e9dc624c0fc18fbf4b5a7871046501eb7f7e4997f11360",
                "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c",
            ),
            (
                "0a076d61737465723012473045022036b87a5180bf8ccd3e0c7590c5e6649cb00d5b9937b730ccb0d679d63911c78a022100f68c1a3698f328a1599025195cbc926c7f3f87bb723d0249e8182c93049cbb681a410a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122103376eae12841185739e97c1d4bf190aa97fe8227c40f31d4c49afb722ebde484c",
                "d914c472224f4a3e34bccf3166496a3e5e1f7936bd88251f7ae987083c73f593",
                "f352a6272ab5180470045a53f763ce6d74e243adc10bac3e23db619e862c8c44",
            ),
        ];

        for (envelope_hex, operation_id, suffix) in fixtures {
            let envelope = hex::decode(envelope_hex).unwrap();
            let signed = SignedPrismOperation::parse(&envelope).unwrap();
            assert_eq!(hex::encode(signed.operation_id().as_bytes()), operation_id);
            let state = PrismDidState::from_signed_create(&signed).unwrap();
            assert_eq!(state.did().suffix(), suffix);
        }
    }
}
