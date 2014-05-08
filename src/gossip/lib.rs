#![crate_id = "gossip"]
#![crate_type = "lib"]
#![feature(globs, macro_rules)]

/*!
Gossip protocol engine written in Rust.

WIP
*/

extern crate collections;
extern crate uuid;
extern crate rand;

pub mod server;
pub mod state;
pub mod util;
pub mod message;
pub mod broadcast;
pub mod protocol;
pub mod transport;
pub mod tcp;