use std::io::{TcpListener, TcpStream, IoError};
use std::io::net::ip::{SocketAddr, IpAddr};
use serialize::json::{Encoder, Decoder, DecoderError};
use serialize::json;
use serialize::{Encodable, Decodable};

use connection::Connection;
use result::{GossipResult, io_err, decoder_err};
use version::Version;
use message::Message;

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

    fn send<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&mut self, m: Message<T>) -> GossipResult<()> {
        let packets = Encoder::buffer_encode(&m);
        write!(self.stream,  "{}", packets.as_slice());
        Ok(())
    }

    fn receive<'a, T: Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>>(&mut self) -> GossipResult<Message<T>> {
        let raw = try!(self.stream.read_to_str().map_err(io_err));
        let json_obj = json::from_str(raw.as_slice());
        let mut decoder = json::Decoder::new(json_obj.unwrap());
        let obj: Message<T> = try!(Decodable::decode(&mut decoder).map_err(decoder_err));
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
