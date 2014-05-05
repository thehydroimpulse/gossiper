use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use cluster::Cluster;
use state::State;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::net::tcp::TcpAcceptor;
use std::io::IoResult;

/// A server/node within a single gossip cluster. Each server has
/// a fast knowledge of it's cluster, which is all stored here.
pub struct Server {
    addr: SocketAddr,
    cluster: Option<Cluster>,
    state: State,
    acceptor: Option<TcpAcceptor>
}

impl Server {
    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: IpAddr, port: u16) -> Server {

        Server {
            // We're handling the creation of the SocketAddr to allow
            // for a more friendly API.
            addr: SocketAddr {
                ip: ip,
                port: port
            },

            // By default, we aren't joining a cluster yet.
            cluster: None,
            state: State::new(),
            acceptor: None
        }
    }

    // Each server needs to have an open tcp connection to join a cluster.
    // This will allow each other member in the cluster to establish a connection
    // to the newest node.
    //
    // When a new node is connecting to the cluster, a new broadcast message will
    // be sent to the network announcing this particular member.
    pub fn listen(&mut self) -> IoResult<()> {
        let listener = try!(TcpListener::bind(self.addr));
        self.acceptor = Some(try!(listener.listen()));
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
        let server = Server::new(Ipv4Addr(127, 0, 0, 1), 4989);

        assert_eq!(server.addr.ip, Ipv4Addr(127, 0, 0, 1));
        assert_eq!(server.addr.port, 4989);
        match server.cluster {
            Some(_) => fail!("Expected a new server without joining a cluster."),
            None => {}
        }
    }

    #[test]
    fn server_should_have_empty_tcp() {
        let server = Server::new(Ipv4Addr(127, 0, 0, 1), 4559);

        match server.acceptor {
            Some(_) => fail!("Expected a new server that doesn't create a tcp server."),
            None => {}
        }
    }

    #[test]
    fn server_should_have_tcp() {
        let mut server = Server::new(Ipv4Addr(127, 0, 0, 1), 5993);

        // Create a new tcp server:
        server.listen();

        let mut stream = TcpStream::connect(server.addr);

        match stream.write([1]) {
            Ok(_) => {},
            Err(err) => fail!("Failed: {}", err)
        }

        drop(stream);
    }
}