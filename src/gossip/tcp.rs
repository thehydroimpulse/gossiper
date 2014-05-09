use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::net::tcp::TcpAcceptor;

use transport::Transport;
use util::GossipResult;

pub struct TcpTransport {
    acceptor: TcpAcceptor
}

impl TcpTransport {
    pub fn new(ip: IpAddr, port: u16) -> TcpTransport {
        let addr = SocketAddr {
            ip: ip,
            port: port
        };

        TcpTransport {
            acceptor: TcpListener::bind(addr).listen().unwrap()
        }
    }
}

impl Transport for TcpTransport {
    fn join<T>(&self, ip: IpAddr, port: u16) -> GossipResult<T> {
        unimplemented!()
    }
}