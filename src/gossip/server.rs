use std::io::net::ip::IpAddr;
use std::io::net::ip::SocketAddr;
use collections::HashSet;
use cluster::Cluster;

/// A server/node within a single gossip cluster. Each server has
/// a fast knowledge of it's cluster, which is all stored here.
#[deriving(Show,Eq)]
pub struct Server {
    addr: SocketAddr,
    cluster: Option<Cluster>,
    metadata: Metadata
}

#[deriving(Show,Eq)]
pub struct Metadata {
    // An eager set contains the peers that the current node will
    // communicate with when a new message comes in. The goal is to form
    // a [Spanning Tree](https://en.wikipedia.org/wiki/Spanning_tree)
    eager_set: HashSet<~str>,

    // The lazy set contains the nodes where if they were within the eager set,
    // would add additional, duplicate edges to the cluster graph. This means that
    // given Node A, B, C, and D:
    //
    // A- B
    // |\/|
    // |/\|
    // C- D
    //
    // Each node has the rest of the nodes within their eager set. This means that
    // given a new broadcast, they'll be duplicate, wasteful messages sent across.
    //
    // This isn't the ideal state that the cluster should be in. We're striving
    // to form a spanning tree where the links to A and D, C and B, and B and D
    // are cut.
    //
    // A --- B
    // |
    // |
    // C --- D
    //
    // This is the most optimal graph our cluster's state would represent.
    //
    // We still need to keep the cut edges, which we'll put inside the lazy set.
    // This is used for healing the tree (when a node goes down or we have a network
    // partition somewhere resulting in some nodes missing broadcasts.)
    lazy_set: HashSet<~str>,
    exchanges: Vec<~str>,
    outstanding: Vec<~str>
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            eager_set: HashSet::new(),
            lazy_set: HashSet::new(),
            exchanges: Vec::new(),
            outstanding: Vec::new()
        }
    }
}

impl Server {

    /// Create a new server given an address (ipv4 or ipv6) and a port.
    /// This function will **not** do any connection initializations. This
    /// is handled by further methods.
    pub fn new(ip: IpAddr, port: u16) -> Server {
        Server {
            // We're handling the creation of the SocketAddr to allow
            // for a more friendly API.
            addr: SocketAddr {
                ip: ip,
                port: port
            },

            // By default, we aren't joining a cluster yet.
            cluster: None,
            metadata: Metadata::new()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::Ipv4Addr;

    #[test]
    fn new_server() {
        let server = Server::new(Ipv4Addr(127, 0, 0, 1), 4989);

        assert_eq!(server.addr.ip, Ipv4Addr(127, 0, 0, 1));
        assert_eq!(server.addr.port, 4989);
        match server.cluster {
            Some(_) => fail!("Expected a new server without joining a cluster."),
            None => {}
        }
    }
}