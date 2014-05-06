use server::Server;

pub struct Cluster {
    nodes: Vec<Server>
}

impl Cluster {
    pub fn new() -> Cluster {
        Cluster {
            nodes: vec![]
        }
    }
}