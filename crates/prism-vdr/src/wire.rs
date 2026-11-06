//! Duplicate-aware scanning for security-relevant PRISM storage fields.

use crate::{Error, ErrorKind};

const MAX_PROTOBUF_FIELD_NUMBER: u64 = (1 << 29) - 1;

pub(crate) fn scan_payload(bytes: &[u8], create: bool, allow_data: bool) -> Result<(), Error> {
    let mut cursor = 0;
    let mut singular = 0_u8;
    let mut data_seen = false;
    while cursor < bytes.len() {
        let (key, next) = read_varint(bytes, cursor)?;
        cursor = next;
        let field = key >> 3;
        let wire_type = (key & 7) as u8;
        if field == 0 || field > MAX_PROTOBUF_FIELD_NUMBER {
            return Err(Error::new(ErrorKind::InvalidWire));
        }
        let value = if wire_type == 2 {
            Some(length_delimited(bytes, cursor)?)
        } else {
            None
        };
        match field {
            1 if create => claim(&mut singular, 0, wire_type, 2)?,
            2 if !create => claim(&mut singular, 0, wire_type, 2)?,
            50 if create => claim(&mut singular, 1, wire_type, 2)?,
            100..=102 if allow_data => {
                expect(wire_type, 2)?;
                if data_seen {
                    return Err(Error::new(ErrorKind::InvalidWire));
                }
                data_seen = true;
                if field == 102 {
                    let (start, end) = value.expect("wire type checked above");
                    scan_status(&bytes[start..end])?;
                }
            }
            100..=102 => return Err(Error::new(ErrorKind::InvalidWire)),
            _ => {}
        }
        cursor = skip_value(bytes, cursor, wire_type)?;
    }
    Ok(())
}

pub(crate) fn scan_status(bytes: &[u8]) -> Result<(), Error> {
    let mut cursor = 0;
    let mut singular = 0_u8;
    while cursor < bytes.len() {
        let (key, next) = read_varint(bytes, cursor)?;
        cursor = next;
        let field = key >> 3;
        let wire_type = (key & 7) as u8;
        match field {
            1 => claim(&mut singular, 0, wire_type, 0)?,
            2 => claim(&mut singular, 1, wire_type, 2)?,
            3 => claim(&mut singular, 2, wire_type, 2)?,
            _ => {}
        }
        cursor = skip_value(bytes, cursor, wire_type)?;
    }
    Ok(())
}

fn claim(seen: &mut u8, bit: u8, actual: u8, expected: u8) -> Result<(), Error> {
    expect(actual, expected)?;
    let mask = 1_u8 << bit;
    if *seen & mask != 0 {
        return Err(Error::new(ErrorKind::InvalidWire));
    }
    *seen |= mask;
    Ok(())
}

fn expect(actual: u8, expected: u8) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidWire))
    }
}

fn read_varint(bytes: &[u8], mut cursor: usize) -> Result<(u64, usize), Error> {
    let mut value = 0_u64;
    for shift in (0..70).step_by(7) {
        let byte = *bytes
            .get(cursor)
            .ok_or_else(|| Error::new(ErrorKind::InvalidWire))?;
        cursor += 1;
        if shift == 63 && byte > 1 {
            return Err(Error::new(ErrorKind::InvalidWire));
        }
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok((value, cursor));
        }
    }
    Err(Error::new(ErrorKind::InvalidWire))
}

fn skip_value(bytes: &[u8], cursor: usize, wire_type: u8) -> Result<usize, Error> {
    match wire_type {
        0 => read_varint(bytes, cursor).map(|(_, next)| next),
        1 => cursor
            .checked_add(8)
            .filter(|end| *end <= bytes.len())
            .ok_or_else(|| Error::new(ErrorKind::InvalidWire)),
        2 => {
            let (length, start) = read_varint(bytes, cursor)?;
            let length = usize::try_from(length).map_err(|_| Error::new(ErrorKind::InvalidWire))?;
            start
                .checked_add(length)
                .filter(|end| *end <= bytes.len())
                .ok_or_else(|| Error::new(ErrorKind::InvalidWire))
        }
        5 => cursor
            .checked_add(4)
            .filter(|end| *end <= bytes.len())
            .ok_or_else(|| Error::new(ErrorKind::InvalidWire)),
        _ => Err(Error::new(ErrorKind::InvalidWire)),
    }
}

fn length_delimited(bytes: &[u8], cursor: usize) -> Result<(usize, usize), Error> {
    let (length, start) = read_varint(bytes, cursor)?;
    let length = usize::try_from(length).map_err(|_| Error::new(ErrorKind::InvalidWire))?;
    let end = start
        .checked_add(length)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| Error::new(ErrorKind::InvalidWire))?;
    Ok((start, end))
}
