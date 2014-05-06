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

pub mod cluster;
pub mod server;
pub mod state;
pub mod error;
pub mod message;
pub mod broadcast;