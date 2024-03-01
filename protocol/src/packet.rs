use bytes::{Buf, Bytes};

pub fn get_unit_le_by_length(bytes: &mut Bytes, n_bytes: usize) -> Result<u64, ParseError> {
    if bytes.remaining() < n_bytes {
        Err(
            ParseError::new(
                format!("expect size is {}, actual size is {}", n_bytes, bytes.remaining()),
                ParseErrorKind::LengthNotEnough,
            )
        )
    } else {
        Ok(bytes.get_uint_le(n_bytes))
    }
}

#[derive(strum_macros::Display)]
enum ParseData {
    Int(u64),
    Str(Bytes),
    Error(String),
    Null,
}

struct ParseError {
    message: String,
    kind: ParseErrorKind,
}

#[derive(strum_macros::Display)]
enum ParseErrorKind {
    LengthNotEnough,
    SystemError,
}

impl ParseError {
    fn new(message: String,
           kind: ParseErrorKind) -> Self {
        Self {
            message,
            kind,
        }
    }

    fn new_form_str(message: String) -> Self {
        Self {
            message,
            kind: ParseErrorKind::SystemError,
        }
    }
}

trait ToParseData {
    fn to_parse_data(self) -> ParseData;
}

impl ToParseData for u64 {
    fn to_parse_data(self) -> ParseData {
        ParseData::Int(self)
    }
}

pub fn parse_int_lenenc(bytes: &mut Bytes) -> Result<ParseData, ParseError> {
    let first_byte = get_unit_le_by_length(bytes, 1)?;
    let data = match first_byte {
        0..=0xFA => first_byte.to_parse_data(),
        0xFB => ParseData::Null,
        0xFC => get_unit_le_by_length(bytes, 2)?.to_parse_data(),
        0xFD => get_unit_le_by_length(bytes, 3)?.to_parse_data(),
        0xFE => get_unit_le_by_length(bytes, 8)?.to_parse_data(),
        0xFF => ParseData::Error("error".to_string()),
        _ => ParseData::Error("error".to_string()),
    };
    Ok(data)
}

pub fn parse_str_lenenc(bytes: &mut Bytes) -> Result<ParseData, ParseError> {
    let string_length = parse_int_lenenc(bytes)?;

    if let ParseData::Int(string_length) = string_length {
        let string_bytes = bytes.split_to(string_length as usize);
        Ok(ParseData::Str(string_bytes))
    } else {
        Err(ParseError::new_form_str(format!("error int<lenenc> {}", string_length)))
    }
}

fn read_packet(bytes: &mut Bytes) -> Result<Bytes, ParseError> {
    let payload_length = bytes.get_uint_le(3) as usize;
    let sequence_id = bytes.get_u8();
    let data = if payload_length == 0 {
        Bytes::new()
    } else {
        let mut _data = bytes.split_to(payload_length);

        if payload_length == 0xFF_FF_FF {
            let mut chain = _data.chain(read_packet(bytes)?);
            _data = chain.copy_to_bytes(chain.remaining())
        }
        _data
    };
    Ok(data)
}

struct Packet {
    payload_length: usize,
    sequence_id: u8,
    payload: Payload,
}

enum Payload {
    OK(OkPacket),
    ERR(ErrPacket),
    EOF(String),
}

struct OkPacket {
    ///int<1>
    header: u8,
    /// int<lenenc>
    /// https://www.cnblogs.com/onlyac/p/6044571.html
    affected_rows: u64,
    /// int<lenenc>
    last_insert_id: u64,

    args: OkPacketArgs,

}

struct ErrPacket {
    header: u8,
    error_code: u16,
    ///string[1]
    sql_state_marker: String,
    ///string[5]
    sql_state: String,
    ///string<EOF>
    error_message: String,
}

enum OkPacketArgs {
    ClientProtocol41 {
        ///int<2>
        status_flags: u16,
        ///int<2>
        warnings: u16,
    },
    ClientTransactions {
        ///int<2>
        status_flags: u16,
    },
    ClientSessionTrack {
        /// string<lenenc>
        /// int<lenenc> + string
        info: String,
    },
    ServerSessionStateChanged {
        /// string<lenenc>
        session_state_info: String,
    },
    Default {
        ///string<EOF>
        info: String,
    },
}

// impl Packet {
//     fn parse(bytes: &mut Bytes) -> Result<Self, ()> {
//         let payload_length = data_type::get_unit_le_length(bytes, 3)?;
//         Ok(Packet { payload_length: 1, sequence_id: 1, payload: Payload::OK })
//     }
// }

#[cfg(test)]
mod test {
    use bytes::{Buf, Bytes};

    use crate::packet::read_packet;

    #[test]
    fn test_read_packet() {
        let mut data: Vec<u8> = vec![
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b0000_0001,
            //0b0000_00001,
        ];
        for n in 1..=16_777_215 {
            data.push(0b0000_0001);
        }
        data.push(0b0000_0001);
        data.push(0b0000_0000);
        data.push(0b0000_0000);
        data.push(0b0000_0001);
        data.push(0b0000_0001);

        let mut bytes = Bytes::from(data);
        let bytes = read_packet(&mut bytes);
        if let Ok(mut bytes) = bytes {
            assert_eq!(16_777_216, bytes.remaining())
        }
    }

    #[test]
    fn test_read_packet_01() {
        let mut data: Vec<u8> = vec![
            0b0000_0001,
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
            0b0000_0001,
        ];
        let mut bytes = Bytes::from(data);
        let bytes = read_packet(&mut bytes);
        if let Ok(mut bytes) = bytes {
            assert_eq!(0b0000_0001, bytes.get_u8())
        }
    }
}