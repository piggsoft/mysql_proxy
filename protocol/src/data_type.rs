use bytes::{Buf, Bytes};

#[derive(Debug, PartialEq)]
enum ParseError {
    LengthNotEnough { expect: usize, actual: usize },
    NotSupportHeader { header: usize, data_type: &'static str },
}

pub fn get_unit_le_length(bytes: &mut Bytes, n_bytes: usize) -> Result<u64, ParseError> {
    if bytes.remaining() < n_bytes {
        Err(ParseError::LengthNotEnough { expect: n_bytes, actual: bytes.remaining() })
    } else {
        Ok(bytes.get_uint_le(n_bytes))
    }
}


pub fn parse(bytes: &mut Bytes) -> Result<u64, ParseError> {
    let first_byte = get_unit_le_length(bytes, 1)?;
    match first_byte {
        0..=0xFB => Ok(first_byte),
        0xFC => get_unit_le_length(bytes, 2),
        0xFD => get_unit_le_length(bytes, 3),
        0xFE => get_unit_le_length(bytes, 8),
        _ => Err(ParseError::NotSupportHeader { header: 0xFF, data_type: "LengthEncodedInteger" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unit_le_length_01() {
        let mut bytes = Bytes::from(Vec::from([0b0000_0001, 0b0000_0010]));
        let result = get_unit_le_length(&mut bytes, 1).unwrap();
        assert_eq!(1, result);
    }

    #[test]
    fn test_get_unit_le_length_02() {
        let mut bytes = Bytes::from(Vec::from([0b0000_0001, 0b0000_0010]));
        let result = get_unit_le_length(&mut bytes, 2).unwrap();
        assert_eq!(0x02_01, result);
    }

    #[test]
    fn test_get_unit_le_length_03() {
        let mut bytes = Bytes::from(Vec::from([0b0000_0001, 0b0000_0010]));
        let result = get_unit_le_length(&mut bytes, 3);
        assert_eq!(Err(ParseError::LengthNotEnough { expect: 3, actual: 2 }), result);
    }

    #[test]
    fn test_get_unit_le_length_04() {
        let mut bytes = Bytes::from(Vec::from([0b1111_1111, 0b0000_0010, 0b0101_0010]));
        let result = parse(&mut bytes);
        assert_eq!(Err(ParseError::NotSupportHeader { header: 0xFF, data_type: "LengthEncodedInteger"}), result);
    }
}
