use std::io::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, IpAddr};

use connection::Connection;
use result::GossipResult;
use message::Message;

pub struct TcpConnection {
    stream: TcpStream
}

impl TcpConnection {
    pub fn new(ip: &str, port: u16) -> TcpConnection {

        let stream = match TcpStream::connect(ip, port) {
            Ok(stream) => stream,
            Err(err) => fail!("Tcp stream failed to connect: {}", err)
        };

        TcpConnection {
            stream: stream
        }
    }
}

impl Connection for TcpConnection {
    fn send(&self, bytes: &[u8]) -> GossipResult<()> {
        Ok(())
    }

    fn receive(&self) -> GossipResult<Message> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::{TcpListener, TcpStream, Listener, Acceptor};
    use std::io::net::tcp::TcpAcceptor;

    #[test]
    fn open_stream() {
        let ip = "127.0.0.1";
        let port = 5689;

        let acceptor = TcpListener::bind(ip, port).listen().unwrap();

        TcpConnection::new(ip, port);
    }
}
