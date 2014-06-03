use util::as_byte_slice;
use version::{ProtocolVersion, Version};
use std::mem::size_of;

pub fn encode<'a, T>(version: ProtocolVersion, val: &'a T) -> Vec<u8> {
    // Encode the message value to a slice of u8s. This allows us to send it across the network.
    let bytes = as_byte_slice(val);

    // Create a new Vector with a capacity of the slice + version.
    let mut stream: Vec<u8> = Vec::with_capacity(bytes.len() + size_of::<u8>());

    let Version(vers) = version;

    // The version is the first byte on the stream.
    stream.push(vers);
    stream.push_all(bytes);

    stream
}
