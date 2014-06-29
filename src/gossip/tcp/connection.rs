use std::io::{TcpListener, TcpStream, IoError};
use std::io::net::ip::{SocketAddr, IpAddr};
use serialize::json::{Encoder, Decoder, DecoderError};
use serialize::{Encodable, Decodable};

use connection::Connection;
use result::{GossipResult, io_err};
use version::Version;

#[deriving(Clone, Share)]
pub struct TcpConnection {
    stream: TcpStream
}

impl TcpConnection {
    pub fn connect(ip: &str, port: u16) -> GossipResult<TcpConnection> {

        let stream = try!(TcpStream::connect(ip, port).map_err(io_err));

        Ok(TcpConnection {
            stream: stream
        })
    }

    pub fn from_stream(stream: TcpStream) -> TcpConnection {
        TcpConnection {
            stream: stream
        }
    }

    pub fn close(&mut self) {
        drop(self);
    }
}

impl Connection for TcpConnection {

    fn send<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&self, m: T) -> GossipResult<()> {
        let packets = Encoder::buffer_encode(&m);
        Ok(())
    }

    fn receive<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&self) -> GossipResult<T> {
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

        TcpConnection::connect(ip, port);
    }
}
