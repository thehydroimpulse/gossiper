use std::io::net::ip::SocketAddr;
use uuid::Uuid;
use serialize::{Encodable, Decodable};

// A message represents a single communication to another peer server. Each
// message may be a request->response type protocol. If that's the case,
// each message within the broadcast will be signed with an appropriate ID so that
// we know which broadcast a message belongs to.
#[deriving(Show, Eq, Encodable, Decodable)]
pub enum Messages {
    Empty
}