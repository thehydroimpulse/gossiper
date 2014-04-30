use server::Server;

#[deriving(Show,Eq,Hash)]
pub struct Cluster {
    nodes: Vec<()>
}