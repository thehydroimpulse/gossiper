//! A wrapper around an ip address and a port number.

/// We work with an ip and port a lot. Let's make it easier
/// and bundle these in a single record.
#[deriving(Show, Clone, Eq, PartialEq, Hash)]
pub struct Addr {
    /// Most of the Rust APIs now use a string for the ip
    /// instead of the IpAddr enum variants (v4, v6).
    pub ip: String,
    /// Standard port number.
    pub port: u16
}

impl Addr {
    /// Working with allocated strings are quite awkward. Slices
    /// are much easier to work with and allow things such as:
    ///
    /// ```rust
    /// Addr::new("0.0.0.0", 8777);
    /// ```
    ///
    /// Instead of:
    ///
    /// ```rust
    /// Addr::new("0.0.0.0".to_string(), 8777);
    /// ```
    pub fn new(ip: &str, port: u16) -> Addr {
        Addr {
            ip: ip.to_string(),
            port: port
        }
    }
}
