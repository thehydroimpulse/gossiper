use result::GossipResult;

pub trait Connection {
    fn send(&self, bytes: &[u8]) -> GossipResult<()>;
    fn receive(&self) -> GossipResult<&[u8]>;
}
