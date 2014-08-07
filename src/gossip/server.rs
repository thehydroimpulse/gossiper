//! Server implementation.

use std::collections::hashmap::HashSet;
use std::comm::{Receiver, Sender};
use std::rc::{Rc, Weak};
use uuid::Uuid;

use addr::Addr;
use broadcast::Broadcast;

/// A health represents the current state of the cluster. This will be extremely useful
/// to ping the health of a cluster and determine the high-level status of it.
///
/// Green = Perfect state.
/// Yellow = Nodes are failing, but nbd.
/// Red = Not good. Cluster might be completely dead.
#[deriving(Show, PartialEq)]
pub enum Health {
    Green,
    Yellow,
    Red
}

/// Messages that can be sent and received to and from the server task.
#[deriving(Show, PartialEq)]
pub enum ServerMsg {
    /// Receive a particular broadcast. We will commit it in our log that can persist to disk.
    Message(Broadcast),
    /// A signal to kill the current server. This will send a IAmShuttingDown message as
    /// a gossip message to let the cluster know why it's shutting down.
    Shutdown,
    /// Kill a specific node in the cluster. This is a state change rather than a gossip. This will
    /// remove a specific node from the cluster.
    KillNode(Node)
}

#[deriving(PartialEq)]
pub struct State {
    eager: HashSet<Node>,
    lazy: HashSet<Node>,
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
    /// Create a new graph with an empty set and a default of spanning: false
    pub fn new() -> Graph {
        Graph {
            vertices: HashSet::new(),
            spanning: false
        }
    }
}

#[deriving(Eq, PartialEq, Hash)]
pub struct Vertex {
    server: Node,
    edges: Vec<Rc<Vertex>>
}

/// A server/node/peer is the most atomic unit within a cluster. Each node is equal with it's peers,
/// thus we don't have any leader or election processes. Each server is identified with a unique ID
/// that is randomly generated, along with the appropriate state.
///
/// A server requires a bit of metadata about the cluster, the cluster's state and things like
/// channels. Each server has it's own Receiver to handle incoming messages. A separate Receiver
/// is used for users of this library. We then have a Sender that sends to the server's receiver.
/// This is meant to be copied to the appropriate task.
pub struct Server {
    /// A unique id for the server. This allows servers to talk about each other in
    /// a consistent manner.
    id: Uuid,
    /// Each server has an Addr instance. Regardless of what type of transport it has.
    addr: Addr,
    /// The state handles the core Gossip protocol. It's basically a giant state machine
    /// that keeps track of which nodes to communicate with, which nodes are alive/dead/failing,
    /// etc...
    state: State,
    /// We need to know a list of servers in the cluster (excluding itself).
    servers: Vec<Node>,
    /// A receiver is used for incoming messages/variants.
    receiver: Receiver<ServerMsg>,
    /// Used to send to the previous receiver. This is meant to be copied to various tasks
    /// that need it.
    sender: Sender<ServerMsg>,
    manager: ServerTaskManager
}

impl Server {
    /// Create a brand new server with a bunch of defaults. It won't actually connect to
    /// anything nor do anything. That's up to the transports to initiate the connections
    /// and such.
    pub fn new(ip: &str, port: u16) -> Server {
        // Create a default channel for the server itself.
        let (tx, rx) = channel();

        let server = Server {
            id: Uuid::new_v4(),
            addr: Addr::new(ip, port),
            state: State::new(),
            servers: Vec::new(),
            receiver: rx,
            sender: tx,
            manager: ServerTaskManager::new()
        };

        server
    }
}

pub struct ServerTaskManager {
    rx: Receiver<ServerMsg>,
    tx: Sender<ServerMsg>
}

impl ServerTaskManager {
    pub fn new() -> ServerTaskManager {
        let (tx, rx) = channel();

        let manager = ServerTaskManager { rx: rx, tx: tx };

        spawn(proc() {
            manager.start();
        });

        rx.recv()
    }
}

/// A node is a server within the cluster without any state associated with it. We
/// only keep state and things like channels for the current server, not other ones in the cluster.
/// A node handles the concept of a node within the cluster that we need to interface with.
///
/// Each server holds enough metadata to work within the cluster, such as all the current members
/// of the cluster. We only need a small number of details for those servers, however, so we'd use
/// a Node instead of the Server record.
#[deriving(Show, Eq, PartialEq, Hash)]
pub struct Node {
    id: Uuid,
    addr: Addr
}

impl Node {
    /// Create a new node given an ip address and a port. This does not actually
    /// connect to that node or anything. They are simply identifiers.
    /// The transport handles
    pub fn new(ip: &str, port: u16) -> Node {
        Node {
            id: Uuid::new_v4(),
            addr: Addr::new(ip, port)
        }
    }
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
