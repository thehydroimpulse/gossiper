use std::io::MemWriter;
use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::io::net::tcp::TcpAcceptor;
use uuid::Uuid;

use broadcast::Broadcast;
use protocol::Peer;
use result::{GossipResult, GossipError};

pub type Callback = (Broadcast, Response);
pub type BroadcastFrom = (Peer, Broadcast);

#[deriving(Clone)]
pub struct Stream {
    stream: TcpStream,
    peer: Option<Peer>
}

impl Stream {
    pub fn new(stream: TcpStream) -> Stream {
        Stream {
            stream: stream,
            peer: None
        }
    }
}

impl Iterator<Callback> for Stream {
    fn next(&mut self) -> Option<Callback> {
        None
    }
}

pub struct Response {
    id: Uuid,
    stream: Stream,
    wr: MemWriter
}

impl Response {
    pub fn new(id: Uuid, stream: Stream) -> Response {
        Response {
            id: id,
            stream: stream,
            wr: MemWriter::new()
        }
    }

    /// Acknowledge the incoming broadcast with a simple OK
    /// message back. Responses aren't always required, but it's
    /// often very useful to have a nice short way of saying
    /// "Got the message, it's all good!".
    ///
    /// This takes `self` as a value because we don't
    /// allow multiple responses. So the response will be moved and
    /// further responses won't be possible.
    pub fn ok(mut self) -> GossipResult<()> {
        write!(self.stream.stream, "{},OK", self.id);
        Ok(())
    }
}

#[unsafe_destructor]
impl Drop for Response {
    /// Handle the response on the drop call.
    fn drop(&mut self) {
    }
}

/// We work with an ip and port a lot. Let's make it easier
/// and bundle these in a single record.
#[deriving(Show, Clone, Eq, PartialEq, Hash)]
pub struct SockAddr {
    /// Most of the Rust APIs now use a string for the ip
    /// instead of the IpSockAddr enum variants (v4, v6).
    pub ip: String,
    /// Standard port number.
    pub port: u16
}

impl SockAddr {
    /// Working with allocated strings are quite awkward. Slices
    /// are much easier to work with and allow things such as:
    ///
    /// ```rust
    /// use gossip::SockAddr;
    /// SockAddr::new("0.0.0.0", 8777);
    /// ```
    ///
    /// Instead of:
    ///
    /// ```rust
    /// use gossip::SockAddr;
    /// SockAddr::new("0.0.0.0", 8777);
    /// ```
    pub fn new(ip: &str, port: u16) -> SockAddr {
        SockAddr {
            ip: ip.to_string(),
            port: port
        }
    }
}
