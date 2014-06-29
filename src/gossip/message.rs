use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use std::io::IoError;

use version::Version;
use server::Server;

#[deriving(Encodable, Decodable, PartialEq, Show)]
pub enum MessageType {
    Request,
    Response
}

#[deriving(Encodable, Decodable, PartialEq, Show)]
pub struct Message<'a, T> {
    version: Version,
    ty: MessageType,
    id: Uuid,
    msg: T
}

impl<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>> Message<'a, T> {
    pub fn new_request(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Request,
            id: Uuid::new_v4(),
            msg: msg
        }
    }

    pub fn new_response(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Response,
            id: Uuid::new_v4(),
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
    pub fn new<'a, T>(server: &Server<'a, T>) -> Join {
        Join {
            id: server.id,
            ip: server.ip.to_string(),
            port: server.port
        }
    }
}
