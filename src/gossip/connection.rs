use result::GossipResult;

pub trait Connection {
    fn send(&self, bytes: Vec<u8>) -> GossipResult<()>;
    fn receive(&self) -> GossipResult<&[u8]>;
}
