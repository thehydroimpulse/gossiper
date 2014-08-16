/// A health represents the current state of the cluster. This will be extremely useful
/// to ping the health of a cluster and determine the high-level status of it.
///
/// Green = Perfect state.
/// Yellow = Nodes are failing, but the cluster is still operational.
/// Red = Not good. Cluster might be completely dead.
#[deriving(Show, PartialEq, Clone)]
pub enum Health {
    Green,
    Yellow,
    Red
}
