//! Server implementation.

use std::collections::hashmap::HashSet;
use std::comm::{Receiver, Sender};
use std::rc::{Rc, Weak};
use uuid::Uuid;

use addr::Addr;
use broadcast::Broadcast;

#[deriving(Show, PartialEq)]
pub enum Health {
    Green,
    Yellow,
    Red
}

#[deriving(Show, PartialEq)]
pub enum ServerMsg {
    /// Receive a particular broadcast. We will commit it in our log that can persist to disk.
    Message(Broadcast),
    /// A signal to kill the current server. This will send a IAmShuttingDown message as
    /// a gossip message to let the cluster know why it's shutting down.
    Shutdown,
    /// Kill a specific node in the cluster. This is a state change rather than a gossip. This will
    /// remove a specific node from the cluster.
    KillNode(InternalServer)
}

#[deriving(PartialEq)]
pub struct State {
    eager: HashSet<InternalServer>,
    lazy: HashSet<InternalServer>,
    health: Health,
    broadcasts: Vec<Broadcast>,
    graph: Graph
}

impl State {
    pub fn new() -> State {
        State {
            eager: HashSet::new(),
            lazy: HashSet::new(),
            health: Yellow,
            broadcasts: Vec::new(),
            graph: Graph::new()
        }
    }
}

/// The graph representation of our communication model. The most ideal representation
/// would be a spanning tree, however, that's not always possible because of the
/// highly-available properties of our distributed system. A spanning tree would essentially
/// prove to be the most minimal set of communication points possible to achieve
/// the successful distribution of our broadcasts.
///
/// We'll have to periodically compute if the graph is a spanning tree or not.
#[deriving(PartialEq)]
pub struct Graph {
    /// We group the graph by vertices so we can easily fetch all the edges of a
    /// particular vertex.
    vertices: HashSet<Vertex>,
    /// Is the tree in spanning mode? This should ensure that we are
    /// in an optimized-mode.
    spanning: bool
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashSet::new(),
            spanning: false
        }
    }
}

#[deriving(Eq, PartialEq, Hash)]
pub struct Vertex {
    server: InternalServer,
    edges: Vec<Rc<Vertex>>
}

pub struct Server {
    addr: Addr,
    state: State,
    servers: Vec<InternalServer>,
    receiver: Receiver<ServerMsg>,
    sender: Sender<ServerMsg>
}

impl Server {
    pub fn new(ip: &str, port: u16) -> Server {
        let (tx, rx) = channel();

        Server {
            addr: Addr::new(ip, port),
            state: State::new(),
            servers: Vec::new(),
            receiver: rx,
            sender: tx
        }
    }
}

#[deriving(Show, Eq, PartialEq, Hash)]
pub struct InternalServer {
    id: Uuid,
    addr: Addr
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_graph() {
        let g = Graph::new();
        assert_eq!(g.spanning, false);
        assert_eq!(g.vertices.len(), 0);
    }

    #[test]
    fn default_state() {
        let s = State::new();
        assert_eq!(s.eager.len(), 0);
        assert_eq!(s.lazy.len(), 0);
        assert_eq!(s.broadcasts.len(), 0);
        assert_eq!(s.health, Yellow);
    }

    #[test]
    fn default_server() {
        let s = Server::new("0.0.0.0", 5666);
        assert_eq!(s.addr.ip.as_slice(), "0.0.0.0");
        assert_eq!(s.addr.port, 5666);
    }
}
