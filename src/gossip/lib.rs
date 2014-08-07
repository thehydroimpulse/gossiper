#![crate_type = "lib"]
#![feature(globs, macro_rules, unsafe_destructor)]
#![deny(missing_doc)]

//! Gossip.rs is a gossip/epidemic protocol based on the
//! paper "Epidemic Broadcast Trees" in which it introduced
//! the novel idea of the Plumtree.
//!
//! Gossip provides a way for a group of nodes, otherwise
//! known as a cluster, to communicate and achieve
//! consensus. However, it's specifically meant to be a
//! highly available system (AP); thus, it supports
//! eventual consistency in face of partitions.
//!
//! This trade-off is fine for many distributed systems
//! that need to be highly available and fault-tolerant. Things
//! such as data processing, analytics, etc... are all great
//! candidates.

extern crate collections;
extern crate uuid;
extern crate rand;
extern crate serialize;
extern crate core;

extern crate msgpack;

pub use server::{Server, Message};
pub use result::{GossipResult, GossipError};
pub use addr::Addr;

pub mod result;
pub mod addr;
mod broadcast;
mod server;
