use std::iter::Iterator;

use broadcast::Broadcast;
use response::Response;
use std::io::net::tcp::TcpStream;

/// An iterator that receives new broadcasts and iterates over them.
pub struct Incoming {
    streams: Vec<TcpStream>
}

impl Incoming {
    pub fn new(streams: Vec<TcpStream>) -> Incoming {
        Incoming {
            streams: streams
        }
    }
}

impl Iterator<(Broadcast, Response)> for Incoming {
    fn next(&mut self) -> Option<(Broadcast, Response)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incoming() {
        
    }
}
