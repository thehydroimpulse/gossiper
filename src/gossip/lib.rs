#![crate_id = "gossip"]
#![crate_type = "lib"]
#![feature(globs, macro_rules, unsafe_destructor)]
#![deny(missing_doc)]
#![allow(unused_must_use,dead_code, unused_imports, unused_variable)]

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

pub mod server;
pub mod state;
pub mod message;
pub mod broadcast;
pub mod protocol;
pub mod transport;
pub mod tcp;
pub mod connection;
pub mod result;
pub mod response;
pub mod version;
pub mod health;
pub mod tagged;
