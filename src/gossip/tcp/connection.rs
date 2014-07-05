use std::io::{TcpListener, TcpStream, IoError};
use std::io::net::ip::{SocketAddr, IpAddr};
use msgpack::{Encoder, Decoder};
use msgpack::from_msgpack;
use serialize::{Encodable, Decodable};

use connection::Connection;
use result::{GossipResult, io_err, decoder_err};
use version::Version;
use message::Message;
use broadcast::Broadcast;

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

    fn send<'a, T: Message
            + Encodable<Encoder<'a>, IoError>
            + Decodable<Decoder<'a>, IoError>> (&mut self, m: Broadcast<T>) -> GossipResult<()> {
        let packets = try!(Encoder::to_msgpack(&m).map_err(io_err));
        write!(self.stream,  "{}", packets.as_slice());
        Ok(())
    }

    fn receive<'a, T: Message
            + Encodable<Encoder<'a>, IoError>
            + Decodable<Decoder<'a>, IoError>>(&mut self) -> GossipResult<Broadcast<T>> {
        let raw = try!(self.stream.read_to_end().map_err(io_err));
        let obj: Broadcast<T> = try!(from_msgpack(raw).map_err(io_err));
        //let obj: Broadcast<T> = try!(Decodable::decode(&mut decoder).map_err(decoder_err));
        Ok(obj)
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
