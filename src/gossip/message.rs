use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};

/// Generic trait for the responses that users will need to
/// implement.
pub trait Response {}

#[deriving(Eq, Show)]
pub enum Versions {
    Version(u8)
}

#[deriving(Eq, Show)]
pub enum MessageKind {
    ResponseKind = 1,
    RequestKind = 2
}

pub struct Message<'a> {
    version: u8,
    /// 1 == Request, 2 == Response
    kind: u8,
    id: &'a [u8],
    bytes: &'a [u8]
}

impl<'a> Message<'a> {
    pub fn new(version: Versions, kind: MessageKind, id: &'a [u8], bytes: &'a [u8]) -> Message<'a> {
        let v = match version {
            Version(v) => v
        };

        Message {
            version: v,
            kind: kind as u8,
            id: id,
            bytes: bytes
        }
    }
}