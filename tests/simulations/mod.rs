use gossip::Addr;
use std::io::process::Process;
use std::io::process::Command;

pub mod ok;

pub struct Peer {
    addr: Addr,
    process: Process,
    test: String
}

impl Peer {
    pub fn new(host: &str, ip: u16, test: &str) -> Peer {
        match Command::new(test)
            .arg(host)
            .arg(ip.to_string())
            .spawn() {
            Ok(process) => Peer {
                addr: Addr::new(host, ip),
                process: process,
                test: test.to_string()
            },
            Err(err) => fail!("Failed to spawn peer: {}", err)
        }
    }
}

pub struct Cluster {
    peer: Peer,
    members: Vec<Peer>
}

impl Cluster {
    pub fn new(num: uint, test: &str) -> Cluster {
        let mut port = 5677;

        // Spin up the peer:
        let peer = Peer::new("localhost", port, test);
        let mut nodes = Vec::new();

        for i in range(1u, num) {
            nodes.push(Peer::new("localhost", port + 1, test));
        }

        Cluster {
            peer: peer,
            members: nodes
        }
    }
}
