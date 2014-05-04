#![crate_id = "gossip"]
#![crate_type = "lib"]
#![feature(globs)]

/*!
Gossip protocol engine written in Rust.

WIP
*/

extern crate collections;

pub mod cluster;
pub mod server;
pub mod state;
pub mod transport;
pub mod error;
pub mod tcp;