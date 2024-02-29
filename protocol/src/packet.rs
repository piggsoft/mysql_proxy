use bytes::{Buf, Bytes};
use crate::data_type;
use crate::data_type::ParseError;


pub fn get_unit_le_by_length(bytes: &mut Bytes, n_bytes: usize) -> Result<u64, ParseError> {
    if bytes.remaining() < n_bytes {
        Err(ParseError::LengthNotEnough {
            expect: n_bytes,
            actual: bytes.remaining(),
        })
    } else {
        Ok(bytes.get_uint_le(n_bytes))
    }
}



enum ParseData {
    Int(u64),
    Str(String),
    Error(String),
    Null,
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
        0xFF => ParseData::Error(String::from("error")),
        _ => ParseData::Error(String::from("error")),
    };
    Ok(data)

}

pub fn parse_str_lenenc(bytes: &mut Bytes, n_bytes: usize) -> Result<ParseData, ParseError> {
    let string_length = parse_int_lenenc(bytes)?;
    if let ParseData::Int(string_length) = string_length {
        let string_bytes = bytes.split_to(string_length as usize);
        String::from_utf8(string_bytes.to_vec())
    }
    Ok(ParseData::Null)
}

struct Packet {
    payload_length: usize,
    sequence_id: u8,
    payload: Payload,
}

enum Payload {
    OK,
    ERR,
    EOF,
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