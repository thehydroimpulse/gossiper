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
    fn new_request(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Request,
            msg: msg
        }
    }

    fn new_response(version: Version, msg: T) -> Message<'a, T> {
        Message {
            version: version,
            ty: Response,
            msg: msg
        }
    }
}
