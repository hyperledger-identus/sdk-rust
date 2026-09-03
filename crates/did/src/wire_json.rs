use std::{collections::BTreeSet, fmt};

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Copy)]
pub(crate) struct JsonWireLimits {
    pub(crate) max_depth: usize,
    pub(crate) max_nodes: usize,
    pub(crate) max_object_members: usize,
    pub(crate) max_live_key_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum JsonWireError {
    DuplicateName,
    TooDeep,
    TooManyNodes,
    TooManyMembers,
    TooManyLiveKeyBytes,
    Malformed,
}

pub(crate) fn validate_unique_object_names(
    input: &[u8],
    limits: JsonWireLimits,
) -> Result<(), JsonWireError> {
    let mut state = ScanState {
        limits,
        nodes: 0,
        live_key_bytes: 0,
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
        return Err(state.failure.unwrap_or(JsonWireError::Malformed));
    }
    deserializer.end().map_err(|_| JsonWireError::Malformed)
}

struct ScanState {
    limits: JsonWireLimits,
    nodes: usize,
    live_key_bytes: usize,
    failure: Option<JsonWireError>,
}

impl ScanState {
    fn visit_node<E: de::Error>(&mut self) -> Result<(), E> {
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > self.limits.max_nodes {
            return self.fail(JsonWireError::TooManyNodes);
        }
        Ok(())
    }

    fn enter_container<E: de::Error>(&mut self, depth: usize) -> Result<(), E> {
        if depth > self.limits.max_depth {
            return self.fail(JsonWireError::TooDeep);
        }
        Ok(())
    }

    fn retain_key<E: de::Error>(&mut self, bytes: usize) -> Result<(), E> {
        self.live_key_bytes = self.live_key_bytes.saturating_add(bytes);
        if self.live_key_bytes > self.limits.max_live_key_bytes {
            return self.fail(JsonWireError::TooManyLiveKeyBytes);
        }
        Ok(())
    }

    fn release_keys(&mut self, bytes: usize) {
        self.live_key_bytes = self.live_key_bytes.saturating_sub(bytes);
    }

    fn fail<T, E: de::Error>(&mut self, reason: JsonWireError) -> Result<T, E> {
        self.failure = Some(reason);
        Err(E::custom(ScanFailure))
    }
}

struct ScanFailure;

impl fmt::Display for ScanFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("JSON wire value violates its resource policy")
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

    fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
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
        let mut names = BTreeSet::new();
        let mut retained_bytes = 0_usize;
        let mut members = 0_usize;

        while let Some(name) = map.next_key::<String>()? {
            members = members.saturating_add(1);
            if members > self.state.limits.max_object_members {
                return self.state.fail(JsonWireError::TooManyMembers);
            }
            if names.contains(&name) {
                return self.state.fail(JsonWireError::DuplicateName);
            }
            retained_bytes = retained_bytes.saturating_add(name.len());
            self.state.retain_key(name.len())?;
            names.insert(name);
            map.next_value_seed(ScanSeed {
                state: &mut *self.state,
                depth: child_depth,
            })?;
        }

        self.state.release_keys(retained_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> JsonWireLimits {
        JsonWireLimits {
            max_depth: 3,
            max_nodes: 16,
            max_object_members: 3,
            max_live_key_bytes: 12,
        }
    }

    #[test]
    fn accepts_complete_unique_json_and_object_local_name_reuse() {
        for input in [
            b"null".as_slice(),
            br#"{"left":{"id":1},"right":{"id":2}}"#,
            br#"[true,false,1,-2,3.5,"text"]"#,
        ] {
            assert_eq!(validate_unique_object_names(input, limits()), Ok(()));
        }
    }

    #[test]
    fn reports_duplicates_after_json_name_decoding() {
        for input in [
            br#"{"same":1,"same":2}"#.as_slice(),
            br#"{"name":1,"\u006eame":2}"#,
            br#"{"outer":{"same":1,"same":2}}"#,
        ] {
            assert_eq!(
                validate_unique_object_names(input, limits()),
                Err(JsonWireError::DuplicateName)
            );
        }
    }

    #[test]
    fn reports_each_resource_and_syntax_boundary() {
        let cases = [
            (br#"[[[[]]]]"#.as_slice(), JsonWireError::TooDeep),
            (
                br#"[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]"#,
                JsonWireError::TooManyNodes,
            ),
            (
                br#"{"a":1,"b":2,"c":3,"d":4}"#,
                JsonWireError::TooManyMembers,
            ),
            (
                br#"{"123456":1,"abcdef":2,"x":3}"#,
                JsonWireError::TooManyLiveKeyBytes,
            ),
            (br#"{"id":1} trailing"#, JsonWireError::Malformed),
            (br#"{"id":"unterminated}"#, JsonWireError::Malformed),
        ];
        for (input, reason) in cases {
            assert_eq!(validate_unique_object_names(input, limits()), Err(reason));
        }
    }
}
