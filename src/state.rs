use std::collections::hashmap::HashSet;
use health::{Health, Yellow};
use broadcast::Broadcast;
use graph::Graph;
use node::Node;

#[deriving(PartialEq, Clone)]
pub struct State {
    eager: HashSet<Node>,
    lazy: HashSet<Node>,
    health: Health,
    broadcasts: Vec<Broadcast>,
    graph: Graph
}

impl State {
    /// Create a new default State that starts a new cluster in a
    /// Yellow state.
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

#[cfg(test)]
mod tests {
    use super::*;
    use health::Yellow;

    #[test]
    fn default_state() {
        let s = State::new();
        assert_eq!(s.eager.len(), 0);
        assert_eq!(s.lazy.len(), 0);
        assert_eq!(s.broadcasts.len(), 0);
        assert_eq!(s.health, Yellow);
    }
}
