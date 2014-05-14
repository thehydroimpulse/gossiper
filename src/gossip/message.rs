use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};

/// Generic trait for the responses that users will need to
/// implement.
pub trait Response {}

pub struct Message {
    version: u8,
    /// 1 == Request, 2 == Response
    key: u8,
    id: u16
}