use std::collections::hashmap::HashSet;
use protocol::{Health, Yellow};
use broadcast::Broadcast;

pub struct State {
    eager: HashSet<String>,
    lazy: HashSet<String>,
    health: Health,
    broadcasts: Vec<Broadcast>
}

impl State {
    /// Create a new default State that starts a new cluster in a
    /// Yellow state.
    pub fn new() -> State {
        State {
            eager: HashSet::new(),
            lazy: HashSet::new(),
            health: Yellow,
            broadcasts: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Yellow;

    #[test]
    fn default_state() {
        let s = State::new();
        assert_eq!(s.eager.len(), 0);
        assert_eq!(s.lazy.len(), 0);
        assert_eq!(s.broadcasts.len(), 0);
        assert_eq!(s.health, Yellow);
    }
}
