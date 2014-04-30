use error::{GossipResult};
use std::io::net::ip::IpAddr;

/// An abstracted transport trait that defines the required methods on
/// which a transport must implement. By default, Tcp is the default transport
/// used. However, once an HTTP server is implemented in Rust, it could be
/// an option.
pub trait Transport {
    /// This method is called on a `Server` type. This will join the server
    /// with a cluster given a peer node. This may populate the cluster property
    /// within the server **if** it's successful. Otherwise it'll return a
    /// GossipError.
    fn join<T>(&self, addr: IpAddr, port: u16) -> GossipResult<T>;
}