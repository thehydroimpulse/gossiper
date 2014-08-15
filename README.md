[<img src="../master/logo.png?raw=true" width="450" />](https://github.com/thehydroimpulse/gossip.rs)

[![Build Status](https://travis-ci.org/thehydroimpulse/gossip.rs.svg?branch=master)](https://travis-ci.org/thehydroimpulse/gossip.rs) [![Stories in Ready](https://badge.waffle.io/thehydroimpulse/gossip.rs.png?label=ready&title=Ready)](https://waffle.io/thehydroimpulse/gossip.rs)

**Note**: This is a work-in-progress. It's not yet useable.

Gossip protocol written in Rust.

## Installing Gossip

Gossip is a Cargo package. You can simply include Gossip as a dependency.

```toml
# Cargo.toml
[project]

name = "foobar"
version = "0.0.1"
authors = []

[dependencies.gossip]
git = "https://github.com/thehydroimpulse/gossip.rs"
```

## What's A Gossip Protocol?

Wikipedia defines it as:

> A gossip protocol is a style of computer-to-computer communication protocol inspired by the form of gossip seen in social networks. Modern distributed systems often use gossip protocols to solve problems that might be difficult to solve in other ways, either because the underlying network has an inconvenient structure, is extremely large, or because gossip solutions are the most efficient ones available.

The concept goes like this:

> You walk into work one morning and Sam (fictional) approaches you. He tells you a secret about Billy. Excited about knowing Billy's secret, you run over to the break room to tell John. At the same time, Sam, the one who first told you, also goes and tells Jimmy. In the afternoon, all of you get together in the meeting room discussing this secret. Then, Amy, who doesn't know it yet, walks in and everybody starts telling her. At first, nobody knows if she knows the secret, so you asked, in which she replied "No?"

That's the basic workflow for gossip protocols, except, we're talking about computers and networks. This is how a network of computers can communicate without having a leader/master node. There are obvious trade-offs here. By achieving the no-leader semantics, you effectively have no control on how effective messages are getting across the network and who hasn't received them. That's the lack of consistency, yet you gain high-availability. It doesn't matter if nodes go down, there aren't any leaders, which means no quorum needs to be held, and no election processes need to happen. On top of that, any node is able to accept requests for information (i.e database queries).

For many systems and tasks, this isn't desireable. There are situations where having a consistent cluster is much simpler and more effective.

## Why Rust?

[Rust](http://www.rust-lang.org/) is Mozilla's new systems programming language that focuses on safety, concurrency and practicality. It doesn't have a mandatory GC, and focuses on safety, without sacrificing performance.

I believe Rust is perfect for distributed systems which are highly performant and fault tolerant. When talking about IO, most languages have (say C, C++, etc...) support for native threads (pthreads, for example). However, there have been newer languages that support more elaborate concurrency primitives in the form of green threads (Go, for example). But there's an obvious trade-off here. You either have native threads *or* green threads. Systems built in these languages are locked to that implementation.

Rust, on the other hand, doesn't have this limitation. It ships with a single API for managing tasks (akin to threads), but, it has two separate implementations: green and native. This allows someone to build systems without picking either of them. The user gets to pick based on which crate they bundle.

Rust is also more in-line to Erlang in terms of error handling. Each task is completely isolated and can be killed, then restarted.

## Getting Started

After adding Gossip as a dependency, you'll want to include the crate within your project:

```rust
extern crate gossip;
```

Now you'll have access to the Gossip system.

### Diving into an example

Let's get started with the Gossip library. We'll start off by creating a `Commit` record having a key and value. This will show you how to use custom types as broadcasts/messages.

The first step is to create a new server with an address and port. We'll then join an existing cluster through an existing membership and start receiving messages. We can easily pattern match on incoming messages and we can also send new ones.

```rust
#![feature(plugin, phase)]

extern crate serialize;
extern crate gossip;

#[phase(plugin, link)]
extern crate gossip_tag;

use serialize::{Encodable, Decodable};
use gossip::{Server, Message};

#[deriving(Encodable, Decodable)]
#[tag]
struct Commit {
    key: String,
    value: Vec<u8>
}

fn main() {
    // Create an in-memory array for our "commits".
    let mut commits = Vec::new();

    // Create a mostly-default server using the in-memory
    // transport.
    let mut node = Server::new("0.0.0.0", 5666);

    // Join an existing cluster. This will go and spawn a separate task
    // for the core gossip protocol, which will create tasks for new connections
    // and such.
    node.join("10.0.0.1", 5666);

    match node.recv() {
        // The first value (String) is the tag and the second is the actual broadcast.
        // A tag allows us to appropriately match a message.
        Message("Commit", broadcast) => {
            let commit: Commit = broadcast.decode();
            commits.push(commit);
        },
        Message("Doom", broadcast) => {
            node.send(Commit { key: "Doom", value: vec![0, 1, 0, 1, 0, 1]});
        }
    }
}
```

Now we have a single server listening on a new cluster.


### Joining An Existing Cluster

To join a cluster, you simply need the ip and port of a peer within that cluster (i.e., any current member).

```rust
server.join("10.0.0.4", 5499);
```

## Papers / Research

* **[Epidemic Broadcast Trees](http://www.gsd.inesc-id.pt/~jleitao/pdf/srds07-leitao.pdf)** (Original Plumtree design)
* [Cassandra's Gossip Protocol](http://www.datastax.com/docs/0.8/cluster_architecture/gossip)
* [How robust are gossip-based communication protocols](https://www.cs.utexas.edu/users/lorenzo/papers/p14-alvisi.pdf)
* [Using Gossip Protocols For Failure Detection, Monitoring, Messaging And Other Good Things](http://highscalability.com/blog/2011/11/14/using-gossip-protocols-for-failure-detection-monitoring-mess.html)
* [GEMS: Gossip-Enabled Monitoring Service for Scalable Heterogeneous Distributed Systems](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.160.2604)
* [A Gossip-Style Failure Detection Service](http://www.cs.cornell.edu/home/rvr/papers/GossipFD.pdf)
* [Controlled Epidemics: Riak's New Gossip Protocol and Metadata Store (Jordan West)](https://www.youtube.com/watch?v=s4cCUTPU8GI)
* [Spanning Tree](https://en.wikipedia.org/wiki/Spanning_tree)

## Testing

```
make test
```

## Building Gossip.rs

```
cargo build
```

## License

The MIT License (MIT)

Copyright (c) 2014 Daniel Fagnan <dnfagnan@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
