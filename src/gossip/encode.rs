use util::as_byte_slice;
use message::Versions;

pub fn encode<'a, T>(version: Versions, val: &'a T) -> &'a [u8] {
    let bytes = as_byte_slice(val);
    bytes
}
