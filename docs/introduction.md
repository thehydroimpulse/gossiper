# Introduction

A fundamental problem in distributed systems is the process of consensus. There are many popular ways to solve this with protocols such as Paxos, Zab, and the ever most popular Raft protocol that went viral.

These are all great. However, they provide a consensus protocol with strong consistency. This introduces the CAP theorem.

The CAP theorem stands for Consistency; Availability; and Partition tolerance. The theory introduces a restriction in what a distributed system may provide at any given time. Now, many people mistakenly assume this is **all** the time; however, that's not the case. At any given time, only two of the three choices may be chosen and the system must behave as such.

In all distributed systems, it's now a common thinking that having partition tolerance is needed. This leaves the choice between availability and strong-consistency.

The above protocols deals with the latter, while Gossip protocols, such as this project, focuses on the former.

Again, there are trade-offs involved and the different focuses have their appropriate uses. There isn't one clear winner, although having strong-consistency is clearly easier to work with.
