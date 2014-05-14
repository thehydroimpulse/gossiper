use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::net::tcp::{TcpAcceptor, TcpStream};

use collections::hashmap::HashMap;
use uuid::Uuid;

use transport::Transport;
use util::GossipResult;
use connection::Connection;
use message::Message;

/// An abstraction on top of a standard Tcp acceptor. Instead of
/// working with a low-level interface, with bytes, the TcpTransport
/// works at a higher level dealing with whole messages. Thus,
/// it's easy to send and receive messages through this interface.
///
/// Sending happens on the connection, receiving happens on the
/// transports (here).
pub struct TcpTransport {
    acceptor: TcpAcceptor,

    // Each connection will have it's own Uuid in correlation
    // with a Server.
    connections: HashMap<Uuid, Box<Connection>>
}

impl TcpTransport {
    pub fn new(ip: &str, port: u16) -> TcpTransport {
        TcpTransport {
            acceptor: TcpListener::bind(ip, port).listen().unwrap(),
            connections: HashMap::new()
        }
    }
}

impl Transport for TcpTransport {
    fn join<T>(&self, ip: &str, port: u16) -> GossipResult<T> {
        unimplemented!()
    }

    fn receive(&self) -> GossipResult<Message> {
        unimplemented!()
    }

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

    #[test]
    fn new_transport() {
        let addr = "127.0.0.1";
        let port = 5499;

        let transport = TcpTransport::new(addr, port);
        let connection = TcpConnection::new(addr, port);
    }
}