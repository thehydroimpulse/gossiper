# Gossip.rs [![Build Status](https://travis-ci.org/TheHydroImpulse/gossip.rs.svg)](https://travis-ci.org/TheHydroImpulse/gossip.rs)

**Note**: This is a work-in-progress. It's not yet useable.

Gossip protocol written in Rust.

## What's A Gossip Protocol?

Wikipedia defines it as:

> A gossip protocol is a style of computer-to-computer communication protocol inspired by the form of gossip seen in social networks. Modern distributed systems often use gossip protocols to solve problems that might be difficult to solve in other ways, either because the underlying network has an inconvenient structure, is extremely large, or because gossip solutions are the most efficient ones available.

The concept goes like this:

> You walk into work one morning and Sam (fictional) approaches you. He tells you a secret about Billy. Excited about knowing Billy's secret, you run over to the break room to tell John. At the same time, Sam, the one who first told you, also goes and tells Jimmy. In the afternoon, all of you get together in the meeting room discussing this secret. Then, Amy, who doesn't know it yet, walks in and everybody starts telling her. At first, nobody knows if she knows the secret, so you asked, in which she replied "No?"

That's the basic workflow for gossip protocols, except, we're talking about computers and networks. This is how a network of computers can communicate without having a leader/master node. There are obvious trade-offs here. By achieving the no-leader semantics, you effectively have no control on how effective messages are getting across the network and who hasn't received them. That's the lack of consistency, yet you gain high-availability. It doesn't matter if nodes go down, there aren't any leaders, which means no quorum needs to be held, and no election processes need to happen. On top of that, any node is able to accept requests for information (i.e database queries).

For many systems and tasks, this isn't desireable. There are situations where having a consistent cluster is much simpler and more effective.

## Why Rust?

[Rust](http://www.rust-lang.org/) is Mozilla's new systems programming language that focuses on safety, concurrency and practicality. It doesn't have GC (but you *can* have task-local GC!); it's also in the same realm as C++, but without many of the issues and complexities.

I believe Rust is perfect for distributed systems which are highly performant and fault tolerant. When talking about IO, most languages have (say C, C++, etc...) have support for native threads (pthreads, for example). However, there have been newer languages that support more elaborate concurrency primitives in the form of green threads (Go, for example). But there's an obvious trade-off here. You either have native threads *or* green threads. Systems built in these languages are locked to that implementation.

Rust, on the other hand, doesn't have this limitation. It ships with a single API for managing tasks (akin to threads), but, it has two separate implementations: green and native. This allows someone to build systems without picking either of them. The user gets to pick based on which crate they bundle.

## Getting Started

## Building Gossip.rs

Currently, Rust doesn't have a useable package manager *yet* (one is in the works). That means Makefiles are the current solution :(.

```rust
make
```

This will produce an `x.rlib` file under the `target` folder. You can then add this path/file to another Rust project.

## Importing Gossip.rs

From another Rust crate, you'll need to let Rust know about the gossip crate.

```rust
// foo/lib.rs
#![crate_id = "foo"]

extern crate gossip;
```

Building `foo` is pretty simple:

```
rustc -Ltarget foo/lib.rs
```

Replacing `target` with the previous target folder that holds the gossip `.rlib` library.


## Documentation

* [Introduction](docs/introduction.md)
* [Peers](docs/peers.md)
* [Cluster](docs/cluster.md)
* [Graph](docs/graph.md)
* [Transport](docs/transports.md)

## Use Cases

Because this is an agnostic gossip protocol (i.e., it can be used for any system built on-top of it.), we can't make certain guarantees that some systems make.

In the original Plumtree paper, it does simulations around 10,000+ node clusters in a P2P system. Because of the large number of peers, the cluster can't be fully connected (i.e., nodes don't have the possibility to communicate with every other node in the cluster.). Thus, it's partially connected; each node has the ability to communicate with a small subset of every other node.

However, when dealing with, say database clusters, you'll never really have 10,000+ nodes in a single cluster. A more realistic number might be a few hundred, maybe a little more. That allows a cluster to be fully connected (i.e., each node may talk to every other node.).

This library will focus on the second use-case for now (having a smaller number of nodes.) but could expand to having the ability to have a partially connected cluster.


## Other Implementations

Most gossip protocols are bundled up with the system implementation. For example, Cassandra has it's own implementation of a gossip protocol that's tied up with the whole implementation of Cassandra.

Notable/Inspired implementations:

* Cassandra
* Riak

## Papers / Research

* **[Epidemic Broadcast Trees](http://www.gsd.inesc-id.pt/~jleitao/pdf/srds07-leitao.pdf)** (Original Plumtree design)
* [Cassandra's Gossip Protocol](http://www.datastax.com/docs/0.8/cluster_architecture/gossip)
* [How robust are gossip-based communication protocols](https://www.cs.utexas.edu/users/lorenzo/papers/p14-alvisi.pdf)
* [Using Gossip Protocols For Failure Detection, Monitoring, Messaging And Other Good Things](http://highscalability.com/blog/2011/11/14/using-gossip-protocols-for-failure-detection-monitoring-mess.html)
* [GEMS: Gossip-Enabled Monitoring Service for Scalable Heterogeneous Distributed Systems](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.160.2604)
* [A Gossip-Style Failure Detection Service](http://www.cs.cornell.edu/home/rvr/papers/GossipFD.pdf)
* [Controlled Epidemics: Riak's New Gossip Protocol and Metadata Store (Jordan West)](https://www.youtube.com/watch?v=s4cCUTPU8GI)
* [Spanning Tree](https://en.wikipedia.org/wiki/Spanning_tree)

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