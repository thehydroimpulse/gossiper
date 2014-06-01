use std::io::net::ip::IpAddr;
use result::GossipResult;
use message::Message;

pub trait Transport {
    fn join<T>(&self, ip: &str, port: u16) -> GossipResult<T>;

    /// Receive the first message that
    fn receive(&self) -> GossipResult<Message>;

    /// Closes the transport. This requires each transport to clean
    /// up any resources they allocated and shutdown the transport.
    ///
    /// New connections may not be accepted beyond this point.
    fn close(&self) -> GossipResult<()>;
}
