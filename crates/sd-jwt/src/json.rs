use std::io::{self, Write};

use serde::Serialize;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};

use crate::{PathSegment, SdJwtError, SdJwtLimits, limits::Work};

pub(crate) fn path_exists(root: &Value, segments: &[PathSegment]) -> bool {
    let mut current = root;
    for segment in segments {
        let Some(next) = (match (current, segment) {
            (Value::Object(map), PathSegment::Member(name)) => map.get(name),
            (Value::Array(array), PathSegment::Index(index)) => array.get(*index),
            _ => None,
        }) else {
            return false;
        };
        current = next;
    }
    true
}

pub(crate) fn parse(
    bytes: &[u8],
    limits: SdJwtLimits,
    work: &mut Work,
) -> Result<Value, SdJwtError> {
    if bytes.len() > limits.issuer_json_bytes {
        return Err(SdJwtError::ResourceLimit);
    }
    work.charge(bytes.len())?;
    let mut budget = JsonBudget {
        nodes: 0,
        members: 0,
        exceeded: false,
        limits,
        work,
    };
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let value = Node {
        budget: &mut budget,
        depth: 0,
    }
    .deserialize(&mut decoder)
    .map_err(|_| {
        if budget.exceeded {
            SdJwtError::ResourceLimit
        } else {
            SdJwtError::Json
        }
    })?;
    decoder.end().map_err(|_| SdJwtError::Json)?;
    Ok(value)
}

struct JsonBudget<'a> {
    nodes: usize,
    members: usize,
    exceeded: bool,
    limits: SdJwtLimits,
    work: &'a mut Work,
}

struct Node<'a, 'b> {
    budget: &'a mut JsonBudget<'b>,
    depth: usize,
}

impl<'de> DeserializeSeed<'de> for Node<'_, '_> {
    type Value = Value;
    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        self.budget.nodes += 1;
        if self.budget.nodes > self.budget.limits.json_nodes
            || self.depth > self.budget.limits.json_depth
            || self.budget.work.charge(1).is_err()
        {
            self.budget.exceeded = true;
            return Err(de::Error::custom("bounded JSON"));
        }
        deserializer.deserialize_any(NodeVisitor {
            budget: self.budget,
            depth: self.depth,
        })
    }
}

struct NodeVisitor<'a, 'b> {
    budget: &'a mut JsonBudget<'b>,
    depth: usize,
}

impl<'de> Visitor<'de> for NodeVisitor<'_, '_> {
    type Value = Value;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("JSON value")
    }
    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(Value::from(v))
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(Value::from(v))
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| de::Error::custom("number"))
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.to_owned()))
    }
    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = seq.next_element_seed(Node {
            budget: self.budget,
            depth: self.depth + 1,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            self.budget.members += 1;
            if self.budget.members > self.budget.limits.json_members {
                self.budget.exceeded = true;
                return Err(de::Error::custom("bounded JSON"));
            }
            if self.budget.work.charge(1).is_err() {
                self.budget.exceeded = true;
                return Err(de::Error::custom("bounded JSON"));
            }
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate or bounded JSON"));
            }
            let value = map.next_value_seed(Node {
                budget: self.budget,
                depth: self.depth + 1,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

pub(crate) fn encode<T: Serialize>(
    value: &T,
    max: usize,
    work: &mut Work,
) -> Result<Vec<u8>, SdJwtError> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        max,
        work,
    };
    serde_json::to_writer(&mut writer, value).map_err(|_| SdJwtError::ResourceLimit)?;
    Ok(writer.bytes)
}

pub(crate) fn encoded_len<T: Serialize>(value: &T, max: usize) -> Result<usize, SdJwtError> {
    let mut writer = CountingWriter { bytes: 0, max };
    serde_json::to_writer(&mut writer, value).map_err(|_| SdJwtError::ResourceLimit)?;
    Ok(writer.bytes)
}

struct CountingWriter {
    bytes: usize,
    max: usize,
}

impl Write for CountingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes = self
            .bytes
            .checked_add(bytes.len())
            .filter(|total| *total <= self.max)
            .ok_or_else(|| io::Error::other("bounded JSON"))?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct BoundedWriter<'a> {
    bytes: Vec<u8>,
    max: usize,
    work: &'a mut Work,
}
impl Write for BoundedWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.max.saturating_sub(self.bytes.len())
            || self.work.charge(bytes.len()).is_err()
        {
            return Err(io::Error::other("bounded JSON"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
