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
use addr::Addr;

/// A server/node within a single gossip cluster.
pub struct Server<'a, T> {
    pub id: Uuid,
    pub addr: Addr,
    state: State,
    transport: T,
    peers: Vec<Server<'a, T>>
}

impl<'a, T: Transport> Server<'a, T> {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(transport: T) -> Server<'a, T> {
        Server {
            id: Uuid::new_v4(),
            addr: transport.addr(),
            state: State::new(),
            transport: transport,
            peers: Vec::new()
        }
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
        let tcp = TcpTransport::listen("127.0.0.1", 5666).unwrap();
        let server = Server::new(tcp);

        assert_eq!(server.addr.ip.as_slice(), "127.0.0.1");
        assert_eq!(server.addr.port, 5666);
    }

    #[test]
    fn empty_peers() {
        let tcp = TcpTransport::listen("127.0.0.1", 5665).unwrap();
        let server = Server::new(tcp);

        assert_eq!(server.peers.len(), 0);
    }
}
