pub trait EnumToString {
    fn enum_to_string(&self) -> &'static str;
}

pub use protocol_macros::EnumToString;
