use std::io::net::ip::IpAddr;
use util::GossipResult;

pub trait Transport {
    fn join<T>(&self, ip: &str, port: u16) -> GossipResult<T>;
}