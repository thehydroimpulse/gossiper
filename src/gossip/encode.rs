use util::as_byte_slice;
use version::{ProtocolVersion, Version};

pub fn encode<'a, T>(version: ProtocolVersion, val: &'a T) -> &'a [u8] {
    let mut packets: Vec<u8> = Vec::new();
    let bytes = as_byte_slice(val);
    let Version(vers) = version;

    packets.push(vers as u8);

    bytes
}
