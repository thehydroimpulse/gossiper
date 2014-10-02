use std::iter::Iterator;

use broadcast::Broadcast;
use response::Response;
use node::Peer;

/// An iterator that receives new broadcasts and iterates over them.
pub struct Incoming {
    node_tx: Sender<(Peer, Broadcast)>,
    tx: Sender<Broadcast>,
    rx: Receiver<Broadcast>,
    listening: bool
}

impl Incoming {
    pub fn new(node_tx: Sender<(Peer, Broadcast)>, sender: Sender<Sender<Broadcast>>) -> Incoming {
        let (tx, rx) = channel();

        sender.send(tx.clone());

        Incoming {
            node_tx: node_tx,
            tx: tx,
            rx: rx,
            listening: true
        }
    }
}

impl Iterator<(Broadcast, Response)> for Incoming {
    fn next(&mut self) -> Option<(Broadcast, Response)> {
        if self.listening {
            let broadcast = self.rx.recv();
            let id = broadcast.id().clone();
            Some((broadcast, Response::new(id)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incoming() {

    }
}
