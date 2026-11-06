use std::io::{self, Write};

use serde::Serialize;

use crate::SdJwtVcError;

struct BoundedWriter {
    bytes: Vec<u8>,
    count: usize,
    maximum: usize,
    retain: bool,
}

impl BoundedWriter {
    fn counting(maximum: usize) -> Self {
        Self {
            bytes: Vec::new(),
            count: 0,
            maximum,
            retain: false,
        }
    }

    fn retaining(maximum: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(maximum.min(1_024)),
            count: 0,
            maximum,
            retain: true,
        }
    }
}

impl Write for BoundedWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let next = self
            .count
            .checked_add(buffer.len())
            .filter(|bytes| *bytes <= self.maximum)
            .ok_or_else(|| io::Error::other("bounded JSON limit exceeded"))?;
        if self.retain {
            self.bytes.extend_from_slice(buffer);
        }
        self.count = next;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) fn bounded_json_size(
    value: &(impl Serialize + ?Sized),
    maximum: usize,
) -> Result<usize, SdJwtVcError> {
    let mut writer = BoundedWriter::counting(maximum);
    serde_json::to_writer(&mut writer, value).map_err(|_| SdJwtVcError::ResourceLimit)?;
    Ok(writer.count)
}

pub(crate) fn bounded_json_bytes(
    value: &(impl Serialize + ?Sized),
    maximum: usize,
) -> Result<Vec<u8>, SdJwtVcError> {
    let mut writer = BoundedWriter::retaining(maximum);
    serde_json::to_writer(&mut writer, value).map_err(|_| SdJwtVcError::ResourceLimit)?;
    Ok(writer.bytes)
}
