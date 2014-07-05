use serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};
use std::io::IoError;

use result::GossipResult;
use message::Message;
use broadcast::Broadcast;

pub trait Connection {
    fn send<'a, T: Message
        + Encodable<Encoder<'a>, IoError>
        + Decodable<Decoder<'a>, IoError>>(&mut self, msg: Broadcast<T>) -> GossipResult<()>;

    fn receive<'a, T: Message
        + Encodable<Encoder<'a>, IoError>
        + Decodable<Decoder<'a>, IoError>>(&mut self) -> GossipResult<Broadcast<T>>;
}
