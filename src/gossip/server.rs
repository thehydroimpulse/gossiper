//! The server is the top-level API that one uses.

use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use std::io::{TcpListener, TcpStream, Listener, Acceptor, IoResult};
use std::io::net::tcp::TcpAcceptor;

use state::State;
use transport::Transport;
use connection::Connection;
use uuid::Uuid;
use result::GossipResult;

/// A server/node within a single gossip cluster.
///
/// Each server will have it's own transport service (Tcp server, for
/// example) and each server will have a connection (TcpStream) to
/// each other node within the cluster. Thus, each has a way for
/// servers to communicate with them
///
/// A (Server, Connection:[B, C]) ----> B (Server, Connection:[A, C])
/// ---- C (Server, Connection:[A, B])
///
/// If A sends a message to C, it uses it's connection to C. If C
/// wants to send a message to B, it uses it's connection to B.
///
/// Each server has a list of Connections, and it has a transport to
/// accept new connections.
///
/// Thus, the transport is used for accepting new connections.
///
pub struct Server<'a, T> {
    pub id: Uuid,
    pub ip: &'a str,
    pub port: u16,
    state: State,
    transport: T,
    peers: Vec<Server<'a, T>>
}

impl<'a, T: Transport> Server<'a, T> {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: &'a str, port: u16, transport: T) -> GossipResult<Server<'a, T>> {
        Ok(Server {
            id: Uuid::new_v4(),
            ip: ip,
            port: port,
            state: State::new(),
            transport: transport,
            peers: Vec::new()
        })
    }

    // Try and join a specific cluster given a peer node.
    pub fn join(&mut self, ip: &str, port: u16) -> GossipResult<()> {
        try!(self.transport.join(ip, port, &*self))
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::Ipv4Addr;
    use std::io::net::tcp::TcpStream;
    use tcp::transport::TcpTransport;
    use transport::Transport;

    #[test]
    fn new_server() {
        //let tcp = TcpTransport::listen("127.0.0.1", 5666).unwrap();
        //let server = Server::new("127.0.0.1", 4989, tcp).unwrap();

        //assert_eq!(server.ip, "127.0.0.1");
        //assert_eq!(server.port, 4989);
    }

}
