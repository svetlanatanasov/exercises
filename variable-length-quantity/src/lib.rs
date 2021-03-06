#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&n| {
            let mut bytes = vec![0x7f & n as u8];
            let mut n = n >> 7;
            while n != 0 {
                bytes.insert(0, (0x7f & n as u8) | 0x80);
                n >>= 7;
            }
            bytes
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values: Vec<u32> = vec![0];
    let last_byte_index = bytes.len() - 1;
    for (i, &byte) in bytes.iter().enumerate() {
        if let Some(num) = values.last_mut() {
            if *num >= (std::u32::MAX >> 4) {
                return Err(Error::Overflow);
            }
            *num = (*num << 7) | u32::from(byte & 0x7F);
        }
        if byte & 0x80 == 0x00 && i != last_byte_index {
            values.push(0);
        } else if byte & 0x80 != 0x00 && i == last_byte_index {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(values)
}
