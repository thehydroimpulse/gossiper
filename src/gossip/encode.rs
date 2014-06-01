use util::as_byte_slice;
use protocol::Protocol;

pub fn encode<'a, T>(version: Protocol, val: &'a T) -> &'a [u8] {
    let bytes = as_byte_slice(val);
    bytes
}
