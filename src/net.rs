//! Abstractions over networking. These are mostly
//! just utility libraries used for simulation testing, like
//! simulating a slow network, partitions, checking if a node
//! is reachable, etc...
//!
//! This handles the testing at the network level, not the software
//! level, which important to have.

use std::io::process::Command;
use result::{GossipResult, GossipError, NodeUnreachable};

/// Slow down the network using "tc".
#[cfg(target_os = "linux")]
#[experimental]
pub fn slow() {
    Command::new("tc")
        .arg("qdisc")
        .arg("add")
        .arg("dev")
        .arg("eth0")
        .arg("root")
        .arg("netem")
        .arg("delay")
        .arg("50ms")
        .arg("10ms")
        .arg("distribution")
        .arg("normal")
        .spawn();
}

#[cfg(not(target_os = "linux"))]
pub fn slow() {
    fail!("`slow` is only supported on Linux.");
}

/// Drop some packets, eh?
#[cfg(target_os = "linux")]
#[experimental]
pub fn flaky() {
    Command::new("tc")
        .arg("qdisc")
        .arg("add")
        .arg("dev")
        .arg("eth0")
        .arg("root")
        .arg("netem")
        .arg("loss")
        .arg("20%")
        .arg("75%")
        .spawn();
}

#[cfg(not(target_os = "linux"))]
pub fn flaky() {
    fail!("`flaky` is only supported on Linux.");
}

/// Speed up the network to it's maximum.
#[cfg(target_os = "linux")]
#[experimental]
pub fn fast() {
    Command::new("tc")
        .arg("qdisc")
        .arg("del")
        .arg("dev")
        .arg("eth0")
        .arg("root")
        .spawn()
}

#[cfg(not(target_os = "linux"))]
pub fn fast() {
    fail!("`fast` is only supported on Linux.");
}

#[experimental]
pub fn is_reachable(ip: &str, port: u16) -> GossipResult<()> {
    match Command::new("ping")
        .arg("-w")
        .arg("1")
        .arg(format!("{}:{}", ip, port))
        .spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(GossipError::new("Node is unreachable", NodeUnreachable))
    }
}

/// Use `iptables` to create a new network partition around
/// a number of nodes in a given cluster.
#[experimental]
pub fn partition() {

}

/// Reset `iptables` back to it's proper configuration.
#[experimental]
pub fn heal() {

}
