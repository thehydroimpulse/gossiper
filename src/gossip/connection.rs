use std::io::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, IpAddr};

pub struct Connection {
    stream: TcpStream,
    addr: SocketAddr
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16) -> Connection {
        let addr = SocketAddr {
            ip: ip,
            port: port
        };

        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(err) => fail!("Tcp stream failed to connect: {}", err)
        };

        Connection {
            stream: stream,
            addr: addr
        }
    }
}