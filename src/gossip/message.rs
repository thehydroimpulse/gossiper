use uuid::Uuid;
use std::fmt::{Show, Formatter, FormatError};
use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use std::io::IoError;

use server::Server;

#[deriving(Encodable, Decodable)]
pub struct Message<T> {
    msg: T
}

impl<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>> Message<T> {
    pub fn new(msg: T) -> Message<T> {
        Message {
            msg: msg
        }
    }
}

#[deriving(Clone, Encodable, Decodable, PartialEq, Show)]
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
