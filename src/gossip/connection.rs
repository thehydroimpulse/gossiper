use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder};
use std::io::IoError;

use result::GossipResult;
use message::Message;

pub trait Connection {
    fn send<'a, T: Encodable<Encoder<'a>, IoError>>(&self, msg: Message<'a, T>) -> GossipResult<()>;
    fn receive<'a, T>(&self) -> GossipResult<Message<'a, T>>;
}
