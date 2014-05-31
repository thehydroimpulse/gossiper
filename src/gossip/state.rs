use collections::HashSet;

#[deriving(Show,Eq)]
pub struct State {
    // An eager set contains the peers that the current node will
    // communicate with when a new message comes in. The goal is to form
    // a [Spanning Tree](https://en.wikipedia.org/wiki/Spanning_tree)
    eager_set: HashSet<String>,

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
    lazy_set: HashSet<String>,
    exchanges: Vec<String>,
    outstanding: Vec<String>
}

impl State {
    pub fn new() -> State {
        State {
            eager_set: HashSet::new(),
            lazy_set: HashSet::new(),
            exchanges: Vec::new(),
            outstanding: Vec::new()
        }
    }
}
