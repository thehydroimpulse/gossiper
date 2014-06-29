use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use std::io::IoError;

use result::GossipResult;
use message::Message;

pub trait Connection {
    fn send<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&self, msg: T) -> GossipResult<()>;
    fn receive<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&self) -> GossipResult<Message<'a, T>>;
}
