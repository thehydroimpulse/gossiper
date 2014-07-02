use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use std::io::IoError;

use result::GossipResult;
use message::Message;
use broadcast::Broadcast;

pub trait Connection {
    fn send<'a, T: Message
        + Encodable<Encoder<'a>, IoError>
        + Decodable<Decoder, DecoderError>>(&mut self, msg: Broadcast<T>) -> GossipResult<()>;

    fn receive<'a, T: Message
        + Encodable<Encoder<'a>, IoError>
        + Decodable<Decoder, DecoderError>>(&mut self) -> GossipResult<Broadcast<T>>;
}
