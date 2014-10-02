use std::task::TaskBuilder;
use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::io::net::tcp::TcpAcceptor;
use std::collections::{HashSet, HashMap};
use std::io::MemWriter;

use uuid::Uuid;
use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError, NotListening};

pub type Callback = (Broadcast, Response);
pub type BroadcastFrom = (Peer, Broadcast);

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

/// An iterator that receives new broadcasts and iterates over them.
pub struct Incoming {
    node_tx: Sender<BroadcastFrom>,
    tx: Sender<(Broadcast, Stream)>,
    rx: Receiver<(Broadcast, Stream)>,
    listening: bool
}

impl Incoming {
    pub fn new(node_tx: Sender<BroadcastFrom>, sender: Sender<Sender<(Broadcast, Stream)>>) -> Incoming {
        let (tx, rx) = channel();

        sender.send(tx.clone());

        Incoming {
            node_tx: node_tx,
            tx: tx,
            rx: rx,
            listening: true
        }
    }
}

impl Iterator<Callback> for Incoming {

    /// TODO: Handle the response with the Drop trait. This means
    ///       we'll need to have a reference to the `Stream`.
    fn next(&mut self) -> Option<Callback> {
        if self.listening {
            let (broadcast, stream) = self.rx.recv();
            let id = broadcast.id().clone();
            Some((broadcast, Response::new(id, stream)))
        } else {
            None
        }
    }
}

/// A dedicated Rust task to manage a single `TcpStream`. Each
/// stream is then associated to a given peer (although the peer
/// isn't always set right away).
///
/// This task then has channels to communicate with the main
/// AcceptorTask which can communicate with other StreamTasks.
struct StreamTask {
    peer: Option<Peer>,
    pub stream: Stream
}

impl StreamTask {
    pub fn new(stream: Stream) -> StreamTask {
        StreamTask {
            peer: None,
            stream: stream
        }
    }

    pub fn incoming(&mut self) {
    }
}

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

/// A dedicated task to handle the incoming connections
/// over a tcp socket (called streams). This task does
/// not need to communicate with the streams themselves, that's
/// done at another task that handles the core logic of the
/// protocol.
struct AcceptorTask {
    acceptor: TcpAcceptor,
    server_tx: Sender<Directive>,
    tx: Sender<Broadcast>,
    rx: Receiver<Broadcast>
}

enum Directive {
    StreamDirective(Stream),
    BroadcastDirective(Broadcast)
}

impl AcceptorTask {
    pub fn new(host: &str, port: u16, server_tx: Sender<Directive>,
               inter_tx: Sender<Sender<Broadcast>>) -> AcceptorTask {
        let listener = TcpListener::bind(host, port).unwrap();
        let (tx, rx) = channel();

        inter_tx.send(tx.clone());

        AcceptorTask {
            acceptor: listener.listen().unwrap(),
            server_tx: server_tx,
            tx: tx,
            rx: rx
        }
    }

    pub fn run(&mut self) {
        for stream in self.acceptor.incoming() {
            match stream {
                Ok(s) => {
                    // Handle the joining here...
                    let stream_send = s.clone();
                    let server = self.server_tx.clone();
                    spawn(proc() {
                        let stream = Stream::new(stream_send);
                        server.send(StreamDirective(stream.clone()));
                        StreamTask::new(stream).incoming();
                    });
                },
                Err(e) => println!("Error: {}", e)
            }
        }
    }
}

struct ServerTask {
    streams: HashMap<Peer, TcpStream>,
    acceptor_tx: Sender<Broadcast>,
    tx: Sender<Directive>,
    rx: Receiver<Directive>
}

impl ServerTask {
    pub fn new(host: String, port: u16) -> ServerTask {
        // Local channels that deal with broadcasts.
        let (tx, rx) = channel();

        // Intermediate channels for the acceptor task.
        // We'll use this to retrieve the sender of the
        // acceptor task.
        let (acceptor_tx, acceptor_rx) = channel();

        let server_tx = tx.clone();
        spawn(proc() {
            AcceptorTask::new(host.as_slice(), port, server_tx, acceptor_tx).run();
        });

        ServerTask {
            streams: HashMap::new(),
            acceptor_tx: acceptor_rx.recv(),
            tx: tx,
            rx: rx
        }
    }

    pub fn broadcast(&self, peer: &Peer) -> GossipResult<()> {
        Ok(())
    }

    pub fn run(&mut self) {
        for signal in self.rx.iter() {
            match signal {
                StreamDirective(stream) => {
                    // self.streams.insert(peer, stream);
                },
                BroadcastDirective(broadcast) => {}
            }
        }
    }
}

/// A peer describes a member within the cluster/network that
/// is not the current one.
#[deriving(Clone, Show, PartialEq, Hash, Eq)]
pub struct Peer {
    id: Uuid,
    addr: Addr
}

impl Peer {
    pub fn new(id: Uuid, host: &str, port: u16) -> Peer {
        Peer {
            id: id,
            addr: Addr::new(host, port)
        }
    }

    pub fn empty() -> Peer {
        Peer {
            id: Uuid::new_v4(),
            addr: Addr::new("localhost", 3444)
        }
    }
}

/// A `Node` is a single member within the gossip protocol. Nodes that
/// join together is called a cluster. This forms the distributed system
/// in which the gossip protocol runs within.
///
/// Each Node is an equal member in the cluster. That means there isn't a
/// single leader. This has a significant trade-off and one must understand
/// it before being able to use the system correctly.
///
/// Node: Handle the state.
/// Incoming: Handle incoming connections and broadcasts.
pub struct Node {
    /// Each node generates their own unique Uuid (v4) to uniquely
    /// identify each other within the cluster. Instead of saying
    /// "I'm node A", you would say "I'm node 123e4567-e89b-12d3-a456-426655440000".
    id: Uuid,

    /// A set of other members within the cluster. This forms the basic
    /// information about each Node. This doesn't, however, contain connection
    /// information and what not.
    members: Vec<Peer>,
    incoming_tx: Option<Sender<(Broadcast, Stream)>>,
    tx: Sender<(Peer, Broadcast)>,
    rx: Receiver<(Peer, Broadcast)>
}

impl Node {
    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    /// let mut node = Node::new();
    /// ```
    pub fn new() -> Node {
        let (tx, rx) = channel();

        Node {
            id: Uuid::new_v4(),
            members: Vec::new(),
            incoming_tx: None,
            tx: tx,
            rx: rx
        }
    }

    /// Initialize the Node to listen on the specified address/port
    /// combination. This will bootup the appropriate tasks to allow
    /// incoming connections and broadcasts.
    #[unimplemented]
    pub fn listen(&mut self, host: &str, port: u16) -> GossipResult<()> {
        let host = host.to_string();

        spawn(proc() {
            ServerTask::new(host, port).run();
        });

        Ok(())
    }

    /// Given a peer node, join it's existing cluster. Each node technically
    /// creates their own cluster automatically. Joining multiple nodes together
    /// is an explicit process. The peer node doesn't need to be the same one,
    /// but it's not a bad idea.
    #[unimplemented]
    pub fn join(&mut self, host: &str, port: u16) -> GossipResult<()> {
        Ok(())
    }

    /// Shutdown all the running tasks that are listening to new broadcasts
    /// and incoming connections. This will send one last broadcast
    /// to the current cluster notifying all other nodes of the shutdown.
    ///
    /// Afterwhich tasks will shutdown and the node will be terminated.
    #[unimplemented]
    pub fn shutdown(&mut self) {}

    /// Create a new `Incoming` iterator that iterates over newly received
    /// broadcasts that the user can handle.
    ///
    /// Usage:
    ///
    /// ```notrust
    /// use gossip::Node;
    /// spawn(proc() {
    ///     let mut node = Node::new();
    ///
    ///     node.listen("localhost", 4888).unwrap();
    ///
    ///     for (broadcast, mut res) in node.incoming() {
    ///         println!("Broadcast ...");
    ///         // ...
    ///     }
    /// });
    /// ```
    pub fn incoming(&mut self) -> Incoming {
        let (tx, rx) = channel();
        let incoming = Incoming::new(self.tx.clone(), tx);

        self.incoming_tx = Some(rx.recv());

        incoming
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_member_set() {
        let mut node = Node::new();
        assert_eq!(node.members.len(), 0);
    }

    #[test]
    fn bind_listening() {
        let mut node = Node::new();
    }
}
