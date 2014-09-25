#![feature(phase)]
extern crate gossip;

extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use docopt::FlagParser;
use gossip::{Node, GossipResult, GossipError};

docopt!(Args, "
Usage: kv <host> <port>

Options:
    -p, --peer  Peer node
", arg_port: u16)

fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    // Create a new node.
    let mut node = Node::new();

    node.bind(args.arg_host.as_slice(), args.arg_port);

    match args.flag_peer {
        Some(peer) => node.join(peer),
        None => {}
    }

    loop {
        node.get_broadcast();
    }
}
