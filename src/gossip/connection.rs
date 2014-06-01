use result::GossipResult;
use message::Message;

pub trait Connection {
    fn send(&self, bytes: &[u8]) -> GossipResult<()>;
    fn receive(&self) -> GossipResult<Message>;
}
