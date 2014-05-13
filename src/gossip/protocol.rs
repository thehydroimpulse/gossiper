use std::str::{Chars, from_char};
use collections::hashmap::HashMap;
use util::{GossipResult, GossipError};

pub enum Protocol {
    Version(uint),
    Binary(Vec<u8>)
}

#[cfg(test)]
mod test {
    use super::*;
    use util::GossipError;
}