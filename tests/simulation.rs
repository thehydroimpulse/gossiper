/// Describe a single simulation done by the system. A simulation allows much more control over
/// the system, including having a default proxy networking system. One can peak into the
/// system and assert the state and/or some combination of state.
///
/// Simulations are also monitored and logged with vital information. This allows one to
/// dump the results of a simulation into a file and also display graphs of some data.
///
/// Examples of data:
///
/// * Reliability: 100% means "the protocol was able to deliver a given message to
///                all active nodes" - Epidemic Broadcast Trees.
/// * RMR (Relative Message Redundancy): This is the message overhead in a gossip
///                                      protocol. The formula is as follows:
///
///     > (m/n-1) -1
///     where:
///         * m = total number of payload messages exchanged during the broadcast
///     procedure.
///         * n = total number of nodes that received that broadcast.
///  An RMR value of zero means that there is exactly one payload message exchange for each
///  node in the system. Zero is the most optimal value.
/// LDH (Last Delivery Hop): measures the number of hops required to deliver a broadcast message
///                          to all recipients. See the reference paper for more details.
pub struct Simulation {
    /// The number of servers that should be spun up during the simulation.
    num_servers: uint
}

