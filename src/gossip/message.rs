use std::io::net::ip::SocketAddr;

// A message represents a single communication to another peer server. Each
// message may be a request->response type protocol. If that's the case,
// each message within the broadcast will be signed with an appropriate ID so that
// we know which broadcast a message belongs to.
#[deriving(Show, Eq)]
pub enum Message {
    // This is the first broadcast message that a new server will send to a
    // chosen peer. This will allow the peer to know that a new server wishes to
    // join the cluster and the cluster metadata and current state should be sent
    // over.
    JoiningCluster(SocketAddr),
    DisconnectingCluster(SocketAddr)
}