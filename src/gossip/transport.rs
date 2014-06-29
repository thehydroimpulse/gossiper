use std::io::net::ip::IpAddr;
use result::GossipResult;
use serialize::Decodable;
use serialize::json::{Decoder, DecoderError};
use std::io::IoError;

use connection::Connection;
use server::Server;

/// A transport handles the communication between nodes. It's abstract
/// enough to support many different implementations, such as Tcp, HTTP,
/// etc...
pub trait Transport {
    /// A new node wants to join the existing cluster. Joining an existing
    /// cluster is as easy as establishing a connection to one
    /// of the nodes in the cluster. The node that already has
    /// a membership in the cluster will be responsible for establishing
    /// a new gossip message to the rest of the nodes.
    fn join<'a, T>(&self, ip: &str, port: u16, server: &Server<'a, T>) -> GossipResult<()>;

    /// Try and receive a message that has been sent to this node.
    fn receive<T: Decodable<Decoder, DecoderError>>(&self) -> GossipResult<T>;

    /// Closes the transport. This requires each transport to clean
    /// up any resources they allocated and shutdown the transport.
    ///
    /// New connections may not be accepted beyond this point.
    fn close(&self) -> GossipResult<()>;
}
