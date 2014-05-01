use error::{GossipResult,GossipError};
use std::io::net::ip::{IpAddr, SocketAddr};
use transport::Transport;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::net::tcp::TcpAcceptor;
use std::io::IoResult;

pub struct TcpTransport {
    acceptor: TcpAcceptor
}

impl TcpTransport {
    /// Each server starts up their own Tcp server for communication.
    pub fn new(addr: IpAddr, port: u16) -> IoResult<TcpTransport> {
        let addr = SocketAddr {
            ip: addr,
            port: port
        };

        let listener = try!(TcpListener::bind(addr));
        let acceptor = try!(listener.listen());

        Ok(TcpTransport {
            acceptor: acceptor
        })
    }
}

impl Transport for TcpTransport {
    fn join<T>(&self, addr: IpAddr, port: u16) -> GossipResult<T> {
        Err(GossipError::new("not implemented.".to_owned(), 5))
    }
}
