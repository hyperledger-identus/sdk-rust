use std::fmt;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use zeroize::Zeroizing;

use crate::CredentialOfferError;

pub(crate) fn validate_json(
    input: &[u8],
    max_depth: usize,
    max_nodes: usize,
) -> Result<(), CredentialOfferError> {
    let mut state = ScanState {
        max_depth,
        max_nodes,
        nodes: 0,
        root_is_object: false,
        failure: None,
    };
    let mut deserializer = serde_json::Deserializer::from_slice(input);
    if (ScanSeed {
        state: &mut state,
        depth: 0,
    })
    .deserialize(&mut deserializer)
    .is_err()
    {
        return Err(state
            .failure
            .unwrap_or(CredentialOfferError::InvalidEmbeddedJson));
    }
    deserializer
        .end()
        .map_err(|_| CredentialOfferError::InvalidEmbeddedJson)?;
    if !state.root_is_object {
        return Err(CredentialOfferError::InvalidEmbeddedJson);
    }
    Ok(())
}

struct ScanState {
    max_depth: usize,
    max_nodes: usize,
    nodes: usize,
    root_is_object: bool,
    failure: Option<CredentialOfferError>,
}

impl ScanState {
    fn visit_node<E: de::Error>(&mut self) -> Result<(), E> {
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > self.max_nodes {
            return self.fail(CredentialOfferError::JsonTooManyNodes);
        }
        Ok(())
    }

    fn enter_container<E: de::Error>(&mut self, depth: usize) -> Result<(), E> {
        if depth > self.max_depth {
            return self.fail(CredentialOfferError::JsonTooDeep);
        }
        Ok(())
    }

    fn fail<T, E: de::Error>(&mut self, reason: CredentialOfferError) -> Result<T, E> {
        self.failure = Some(reason);
        Err(E::custom(ScanFailure))
    }
}

struct ScanFailure;

impl fmt::Display for ScanFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("JSON value violates the Credential Offer resource policy")
    }
}

struct ScanSeed<'a> {
    state: &'a mut ScanState,
    depth: usize,
}

impl<'de> DeserializeSeed<'de> for ScanSeed<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.state.visit_node()?;
        deserializer.deserialize_any(ScanVisitor {
            state: self.state,
            depth: self.depth,
        })
    }
}

struct ScanVisitor<'a> {
    state: &'a mut ScanState,
    depth: usize,
}

impl<'de> Visitor<'de> for ScanVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded JSON value")
    }

    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        let _value = Zeroizing::new(value);
        Ok(())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let child_depth = self.depth.saturating_add(1);
        self.state.enter_container(child_depth)?;
        while sequence
            .next_element_seed(ScanSeed {
                state: &mut *self.state,
                depth: child_depth,
            })?
            .is_some()
        {}
        Ok(())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let child_depth = self.depth.saturating_add(1);
        self.state.enter_container(child_depth)?;
        if self.depth == 0 {
            self.state.root_is_object = true;
        }
        let mut names: Vec<Zeroizing<String>> = Vec::new();

        while let Some(name) = map.next_key::<String>()? {
            let name = Zeroizing::new(name);
            if names
                .iter()
                .any(|existing| existing.as_str() == name.as_str())
            {
                return self.state.fail(CredentialOfferError::DuplicateJsonProperty);
            }
            names.push(name);
            map.next_value_seed(ScanSeed {
                state: &mut *self.state,
                depth: child_depth,
            })?;
        }
        Ok(())
    }
}
