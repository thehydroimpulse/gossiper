use error::{GossipResult,GossipError};
use std::io::net::ip::IpAddr;
use transport::Transport;

pub struct TcpTransport {
    connected: bool
}

impl Transport for TcpTransport {
    fn join<T>(&self, addr: IpAddr, port: u16) -> GossipResult<T> {
        Err(GossipError::new("not implemented.".to_owned(), 5))
    }
}
