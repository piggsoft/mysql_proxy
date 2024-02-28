use bytes::Bytes;
use crate::data_type;

struct Packet {
    payload_length: usize,
    sequence_id: u8,
    payload: Payload,
}

enum Payload {
    OK
}

// impl Packet {
//     fn parse(bytes: &mut Bytes) -> Result<Self, ()> {
//         let payload_length = data_type::get_unit_le_length(bytes, 3)?;
//         Ok(Packet { payload_length: 1, sequence_id: 1, payload: Payload::OK })
//     }
// }