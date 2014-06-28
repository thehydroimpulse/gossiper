use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use std::io::IoError;

use version::Version;

#[deriving(Encodable, Decodable, PartialEq, Show)]
pub enum MessageType {
    Request,
    Response
}

#[deriving(Encodable, Decodable, PartialEq, Show)]
pub struct Message<'a, T> {
    version: Version,
    ty: MessageType,
    msg: T
}

impl<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>> Message<'a, T> {
    pub fn new_request(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Request,
            msg: msg
        }
    }

    pub fn new_response(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Response,
            msg: msg
        }
    }
}


#[deriving(Encodable, Decodable, PartialEq, Show)]
pub struct Join {
    id: Uuid,
    ip: String,
    port: u16
}

impl Join {
    pub fn new(id: Uuid, ip: String, port: u16) -> Join {
        Join {
            id: id,
            ip: ip,
            port: port
        }
    }
}
