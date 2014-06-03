use std::str::{Chars, from_char};
use collections::hashmap::HashMap;
use result::{GossipResult, GossipError};

pub enum Protocol {
    Binary(Vec<u8>)
}

#[cfg(test)]
mod test {
    use super::*;
    use result::GossipError;
}
