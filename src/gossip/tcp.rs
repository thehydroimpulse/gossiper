use transport::Transport;
use std::io::net::ip::IpAddr;
use util::GossipResult;

pub struct TcpTransport {
    running: bool
}

impl TcpTransport {
    pub fn new() -> TcpTransport {
        TcpTransport {
            running: false
        }
    }
}

impl Transport for TcpTransport {
    fn join<T>(&self, ip: IpAddr, port: u16) -> GossipResult<T> {
        unimplemented!()
    }
}