use std::io::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, IpAddr};

use connection::Connection;

pub struct TcpConnection {
    stream: TcpStream,
    addr: SocketAddr
}

impl TcpConnection {
    pub fn new(ip: IpAddr, port: u16) -> TcpConnection {
        let addr = SocketAddr {
            ip: ip,
            port: port
        };

        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(err) => fail!("Tcp stream failed to connect: {}", err)
        };

        TcpConnection {
            stream: stream,
            addr: addr
        }
    }
}

impl Connection for TcpConnection {}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::{TcpListener, TcpStream, Listener, Acceptor};
    use std::io::net::tcp::TcpAcceptor;

    #[test]
    fn open_stream() {
        let ip = Ipv4Addr(127, 0, 0, 1);
        let port = 5689;

        let addr = SocketAddr { ip: ip, port: port };
        let acceptor = TcpListener::bind(addr).listen().unwrap();

        TcpConnection::new(ip, port);
    }
}