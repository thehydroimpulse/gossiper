use error::{GossipResult};
use std::io::net::ip::IpAddr;
use transport::Transport;

pub struct TcpTransport {
    connected: bool
}

impl Transport for TcpTransport {
    pub fn join<T>(&self, addr: IpAddr, port: u16) -> GossipResult<T>;
}
