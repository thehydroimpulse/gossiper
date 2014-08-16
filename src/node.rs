use uuid::Uuid;
use addr::Addr;

/// A node is a server within the cluster without any state associated with it. We
/// only keep state and things like channels for the current server, not other ones in the cluster.
/// A node handles the concept of a node within the cluster that we need to interface with.
///
/// Each server holds enough metadata to work within the cluster, such as all the current members
/// of the cluster. We only need a small number of details for those servers, however, so we'd use
/// a Node instead of the Server record.
#[deriving(Show, Eq, PartialEq, Hash, Clone)]
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
