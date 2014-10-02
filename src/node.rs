use std::task::TaskBuilder;
use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::io::net::tcp::TcpAcceptor;
use std::collections::HashMap;

use uuid::Uuid;
use addr::Addr;
use result::{GossipResult, GossipError, NotListening};
use broadcast::Broadcast;
use incoming::Incoming;

/// A dedicated Rust task to manage a single `TcpStream`. Each
/// stream is then associated to a given peer (although the peer
/// isn't always set right away).
///
/// This task then has channels to communicate with the main
/// AcceptorTask which can communicate with other StreamTasks.
struct StreamTask {
    peer: Option<Peer>,
    stream: TcpStream
}

impl StreamTask {
    pub fn new(stream: TcpStream) -> StreamTask {
        StreamTask {
            peer: None,
            stream: stream
        }
    }

    pub fn incoming(&mut self) {

    }
}

/// A dedicated task to handle the incoming connections
/// over a tcp socket (called streams). This task does
/// not need to communicate with the streams themselves, that's
/// done at another task that handles the core logic of the
/// protocol.
struct AcceptorTask {
    acceptor: TcpAcceptor,
    server_tx: Sender<Broadcast>,
    tx: Sender<Broadcast>,
    rx: Receiver<Broadcast>
}

impl AcceptorTask {
    pub fn new(host: &str, port: u16, server_tx: Sender<Broadcast>,
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
                Ok(stream) => spawn(proc() {
                    StreamTask::new(stream).incoming();
                }),
                Err(e) => println!("Error: {}", e)
            }
        }
    }
}

struct ServerTask {
    streams: HashMap<Peer, Sender<Broadcast>>,
    acceptor_tx: Sender<Broadcast>,
    tx: Sender<Broadcast>,
    rx: Receiver<Broadcast>
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

    pub fn run(&mut self) {

    }
}

/// A peer describes a member within the cluster/network that
/// is not the current one.
#[deriving(Show, PartialEq, Hash, Eq)]
pub struct Peer {
    id: Uuid,
    addr: Addr
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
    incoming_tx: Option<Sender<Broadcast>>,
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
