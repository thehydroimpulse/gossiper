use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::net::tcp::{TcpAcceptor, TcpStream};

use std::collections::hashmap::HashMap;
use serialize::Decodable;
use serialize::json::{Decoder, DecoderError};
use uuid::Uuid;
use std::io::IoError;

use transport::Transport;
use result::{GossipResult, from_io};
use connection::Connection;
use message::Message;

/// A tcp transport has two fundamental elements within: An acceptor (server)
/// and a set of connections. The only job of the acceptor is to, well,
/// accept new connections and store them.
///
/// Connections are how all the nodes will communicate. Each node within
/// the system has a connection to every other node in the cluster. That
/// means, if node A wants to communicate with server B, it'll look up
/// server's B connection and send a message through that medium.
pub struct TcpTransport {
    /// The acceptor is responsible for accepting new connections and
    /// storing them appropriately. The only times a new connection is
    /// added to the node are:
    ///     * A new node has joined the cluster and has to establish a
    ///       connection with each other node.
    ///     * A connection we had has disconnected and the disconnected
    ///       node has initiated a new connection with us. This can be
    ///       for failed nodes coming back alone or partitioned nodes.
    acceptor: TcpAcceptor,

    /// A single server might have 10s or even 100s of connections, so
    /// we need an effecient way to fetch them based on the node
    /// we want to communicate with. Each server will have it's own
    /// unique Uuidv4 which we'll use as the key for the hashmap.
    connections: HashMap<Uuid, Box<Connection>>
}

impl TcpTransport {

    /// Createa a new local transport. The ip and port are used for the
    /// Acceptor. Thus, having a local address of "0.0.0.0" is the common
    /// practice for the tcp server to be accessible from outside the
    /// current node.
    ///
    /// FIXME: Perhaps we should handle the errors a little nicer?
    pub fn new(ip: &str, port: u16) -> GossipResult<TcpTransport> {
        let listener = try!(TcpListener::bind(ip, port).map_err(from_io));
        let acceptor = try!(listener.listen().map_err(from_io));

        Ok(TcpTransport {
            acceptor: acceptor,
            connections: HashMap::new()
        })
    }
}

impl Transport for TcpTransport {

    fn new_connection(&mut self, ip: &str, port: u16)
        -> GossipResult<Box<Connection>> {
        unimplemented!()
    }

    /// By default, a node does **not** join a cluster automatically. Thus,
    /// one has to manually initiate the join operation.
    ///
    /// To join a cluster, one just needs to establish a connection
    /// with one that already has membership (peer node). The join operation first
    /// establishes the new connection **and** sends a "join" broadcast.
    ///
    /// The peer node is responsible for propagating the new membership details
    /// through a new broadcast.
    fn join<T>(&self, ip: &str, port: u16) -> GossipResult<T> {
        unimplemented!()
    }

    /// Receive a message from any of the connections.
    fn receive<T: Decodable<Decoder, DecoderError>>(&self) -> GossipResult<T> {
        unimplemented!()
    }

    /// Terminate the accept, along with disconnecting all connections. However,
    /// before doing so, the node will send one last broadcast letting the
    /// cluster know it's going offline.
    fn close(&self) -> GossipResult<()> {
        drop(self);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{Ipv4Addr};
    use tcp::connection::TcpConnection;
    use connection::Connection;

    #[test]
    fn new_transport() {
        let addr = "127.0.0.1";
        let port = 5499;

        let transport = TcpTransport::new(addr, port);
        let connection = TcpConnection::new(addr, port);
    }
}
