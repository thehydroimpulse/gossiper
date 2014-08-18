use std::collections::hashmap::HashSet;
use std::rc::Rc;

use node::Node;

/// The graph representation of our communication model. The most ideal representation
/// would be a spanning tree, however, that's not always possible because of the
/// highly-available properties of our distributed system. A spanning tree would essentially
/// prove to be the most minimal set of communication points possible to achieve
/// the successful distribution of our broadcasts.
///
/// We'll have to periodically compute if the graph is a spanning tree or not.
#[deriving(PartialEq, Clone)]
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

#[deriving(Eq, PartialEq, Hash, Clone)]
pub struct Vertex {
    server: Node,
    edges: Vec<Rc<Vertex>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_graph() {
        let g = Graph::new();
        assert_eq!(g.spanning, false);
        assert_eq!(g.vertices.len(), 0);
    }
}
