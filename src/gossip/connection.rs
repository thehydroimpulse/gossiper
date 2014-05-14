use util::GossipResult;

pub trait Connection {
    fn send(&self, bytes: &[u8]) -> GossipResult<()>;
}