use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener};
use std::io::net::tcp::TcpAcceptor;
use std::io::IoResult;
use std::io::{Listener, Acceptor};

use state::State;
use message::Message;
use message::JoiningCluster;
use transport::Transport;

/// A server/node within a single gossip cluster. Each server has
/// a fast knowledge of it's cluster, which is all stored here.
pub struct Server {

    // Local address of the server. (Tcp)
    addr: SocketAddr,

    // The state will contain the spanning tree implementation and all the members
    // we'll be communicating with.
    state: State,
    transport: Option<~Transport>
}

impl Server {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: IpAddr, port: u16, transport: Option<~Transport>) -> Server {

        let addr = SocketAddr { ip: ip, port: port };
        let mut acceptor = TcpListener::bind(addr).listen().unwrap();

        let mut server = Server {
            // We're handling the creation of the SocketAddr to allow
            // for a more friendly API.
            addr: addr,
            state: State::new(),
            transport: transport
        };

        server
    }

    // Try and join a specific cluster given a peer node.
    pub fn join(&mut self, ip: IpAddr, port: u16) -> IoResult<()> {
        // Establish a new connection with the peer node.
        let mut stream = TcpStream::connect(SocketAddr {
            ip: ip,
            port: port
        });

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
        let server = Server::new(Ipv4Addr(127, 0, 0, 1), 4989, None);

        assert_eq!(server.addr.ip, Ipv4Addr(127, 0, 0, 1));
        assert_eq!(server.addr.port, 4989);
    }

    #[test]
    fn server_should_have_tcp() {
        let mut server = Server::new(Ipv4Addr(127, 0, 0, 1), 5993, None);
        let mut stream = TcpStream::connect(server.addr);

        match stream.write([1]) {
            Ok(_) => {},
            Err(err) => fail!("Failed: {}", err)
        }

        drop(stream);
    }

    #[test]
    fn server_join_cluster() {
        let mut peer = Server::new(Ipv4Addr(127, 0, 0, 1), 5994, None);
        let mut server = Server::new(Ipv4Addr(127, 0, 0, 1), 5944, None);

        server.join(Ipv4Addr(127, 0, 0, 1), 5944);
    }
}