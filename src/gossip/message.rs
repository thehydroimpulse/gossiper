use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};
use util::as_byte_slice;

pub enum Versions {
    Version(u16)
}
