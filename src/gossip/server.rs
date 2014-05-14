use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use std::io::{TcpListener, TcpStream, Listener, Acceptor, IoResult};
use std::io::net::tcp::TcpAcceptor;

use state::State;
use transport::Transport;
use connection::Connection;

/// A server/node within a single gossip cluster. Each server has
/// a fast knowledge of it's cluster, which is all stored here.
pub struct Server<'a> {

    // Local address of the server. (Tcp)
    ip: &'a str,
    port: u16,

    // The state will contain the spanning tree implementation and all the members
    // we'll be communicating with.
    state: State,

    transport: Option<Box<Transport>>,
    peers: Vec<Server<'a>>,
    connections: Vec<Box<Connection>>
}

impl<'a> Server<'a> {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: &'a str, port: u16, transport: Option<Box<Transport>>) -> Server<'a> {
        Server {
            // We're handling the creation of the SocketAddr to allow
            // for a more friendly API.
            ip: ip,
            port: port,
            state: State::new(),
            transport: transport,
            peers: vec![],
            connections: vec![]
        }
    }

    // Try and join a specific cluster given a peer node.
    pub fn join(&self, ip: &str, port: u16) -> IoResult<()> {
        // Establish a new connection with the peer node.
        let stream = TcpStream::connect(ip, port);

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::Ipv4Addr;
    use std::io::net::tcp::TcpStream;

    #[test]
    fn new_server() {
        let server = Server::new("127.0.0.1", 4989, None);

        assert_eq!(server.ip, "127.0.0.1");
        assert_eq!(server.port, 4989);
    }

    #[test]
    fn server_should_have_tcp() {
        // let server = Server::new("127.0.0.1", 5993, None);
        // let mut stream = TcpStream::connect(server.addr);
    }

    #[test]
    fn server_join_cluster() {
        let peer = Server::new("127.0.0.1", 5994, None);
        let server = Server::new("127.0.0.1", 5944, None);

        server.join("127.0.0.1", 5944);
    }
}